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
    GridPosition, Instruction, GRID_SIZE, GamePhase,
};

pub struct GameState {
    maze: Maze,
    tank: Tank,
    // instruction_queue is now current_script
    current_script: Vec<Instruction>,
    phase: GamePhase,
    execution_step: usize,
    execute_timer: f32, // To slow down execution for visibility
    game_won: bool,
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
                // Buttons do nothing yet
            }
            Instruction::Noop => {}
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.phase {
            GamePhase::Plan => {
                // Waiting for user input via mouse
            }
            GamePhase::Execution => {
                if !self.game_won {
                    self.execute_timer += ctx.time.delta().as_secs_f32();
                    if self.execute_timer >= 0.5 {
                        self.execute_timer = 0.0;
                        if self.execution_step < self.current_script.len() {
                            let instr = self.current_script[self.execution_step];
                            self.execute_instruction(instr);
                            self.execution_step += 1;
                        } else {
                            // Script finished
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

        // Define a margin size to create the boundary effect
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

                // Get base rect from position
                let base_rect: graphics::Rect = GridPosition::new(x, y).into();

                // Shrink rect by margin
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
        // Also apply margin to tank so it fits "inside" the tile boundaries
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

        // ==========================
        // Sidebar
        // ==========================
        Sidebar::draw(ctx, &mut canvas, &self.current_script, self.phase)?;

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