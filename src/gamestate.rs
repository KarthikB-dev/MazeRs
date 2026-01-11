use ggez::{
    event,
    graphics,
    input::mouse,
    Context, GameResult,
};

use crate::maze::{Maze, Tile};
use crate::sidebar::Sidebar;
use crate::tank::Tank;
use crate::{
    GridPosition, Instruction, GRID_SIZE, GamePhase, MAP_WIDTH, MAP_HEIGHT,
};

pub struct GameState {
    maze: Maze,
    tank: Tank,
    current_script: Vec<Instruction>,
    phase: GamePhase,
    execution_step: usize,
    execute_timer: f32,
    game_won: bool,
    win_timer: f32, // New timer for the delay before quitting
}

impl GameState {
    pub fn new() -> Self {
        Self {
            maze: Maze::new(),
            tank: Tank::new(GridPosition::new(0, 0)),
            current_script: Vec::new(),
            phase: GamePhase::Plan,
            execution_step: 0,
            execute_timer: 0.0,
            game_won: false,
            win_timer: 0.0,
        }
    }

    fn execute_instruction(&mut self, instr: Instruction) {
        match instr {
            Instruction::Move(dir) => {
                let new_pos = self.tank.pos().moved(dir);
                if let Some(tile) = self.maze.tile_at(new_pos) {
                    if !matches!(tile, Tile::Wall) {
                        self.tank.set_pos(new_pos);
                    }
                }
            }
            Instruction::Interact => {
                if let Some(Tile::Goal) = self.maze.tile_at(self.tank.pos()) {
                    self.game_won = true;
                }
            }
            Instruction::Noop => {}
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // If game is won, handle the countdown to quit
        if self.game_won {
            self.win_timer += ctx.time.delta().as_secs_f32();
            if self.win_timer >= 2.0 {
                ctx.request_quit();
            }
            return Ok(());
        }

        match self.phase {
            GamePhase::Plan => {}
            GamePhase::Execution => {
                self.execute_timer += ctx.time.delta().as_secs_f32();
                if self.execute_timer >= 0.5 {
                    self.execute_timer = 0.0;
                    if self.execution_step < self.current_script.len() {
                        let instr = self.current_script[self.execution_step];
                        self.execute_instruction(instr);
                        self.execution_step += 1;
                    } else {
                        // If script finished and we didn't win, reset
                        if !self.game_won {
                            self.phase = GamePhase::Plan;
                            self.current_script.clear();
                            self.execution_step = 0;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.1, 0.1, 1.0]));

        let margin = 2.0;

        // Draw maze
        for y in 0..GRID_SIZE.1 {
            for x in 0..GRID_SIZE.0 {
                let tile = self.maze.tiles()[y as usize][x as usize];
                let color = match tile {
                    Tile::Empty => [0.2, 0.2, 0.2, 1.0],
                    Tile::Wall => [0.0, 0.0, 0.0, 1.0],
                    Tile::Goal => [0.0, 1.0, 0.0, 1.0],
                    Tile::Button => [0.0, 0.0, 1.0, 1.0],
                };

                let base_rect: graphics::Rect = GridPosition::new(x, y).into();
                let draw_rect = graphics::Rect::new(
                    base_rect.x + margin,
                    base_rect.y + margin,
                    base_rect.w - margin * 2.0,
                    base_rect.h - margin * 2.0,
                );

                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(draw_rect)
                        .color(color),
                );
            }
        }

        // Draw tank
        let tank_base_rect: graphics::Rect = self.tank.pos().into();
        let tank_draw_rect = graphics::Rect::new(
            tank_base_rect.x + margin,
            tank_base_rect.y + margin,
            tank_base_rect.w - margin * 2.0,
            tank_base_rect.h - margin * 2.0,
        );

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(tank_draw_rect)
                .color([1.0, 0.0, 0.0, 1.0]),
        );

        // Sidebar
        Sidebar::draw(ctx, &mut canvas, &self.current_script, self.phase)?;

        // Victory Overlay
        if self.game_won {
            let overlay_rect = graphics::Rect::new(0.0, 0.0, MAP_WIDTH, MAP_HEIGHT);
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(overlay_rect)
                    .color([0.0, 0.0, 0.0, 0.7]),
            );

            let mut text = graphics::Text::new("YOU WIN!");
            text.set_scale(60.0);
            
            let text_dims = text.measure(ctx)?;
            let text_pos = [
                (MAP_WIDTH - text_dims.x) / 2.0,
                (MAP_HEIGHT - text_dims.y) / 2.0,
            ];

            canvas.draw(
                &text,
                graphics::DrawParam::new()
                    .dest(text_pos)
                    .color([1.0, 1.0, 0.0, 1.0]),
            );
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: mouse::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if self.game_won {
            return Ok(());
        }

        if button != mouse::MouseButton::Left {
            return Ok(());
        }

        let prev_phase = self.phase;
        Sidebar::handle_click(
            x,
            y,
            &mut self.current_script,
            &mut self.phase,
        );

        if prev_phase == GamePhase::Plan && self.phase == GamePhase::Execution {
            self.execution_step = 0;
            self.execute_timer = 0.0;
        }
        
        Ok(())
    }
}
