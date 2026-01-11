use ggez::{
    event,
    graphics,
    input::mouse,
    Context, GameResult,
    mint::Vector2,
};

use crate::assets::GameAssets;
use crate::maze::{Maze, Tile};
use crate::sidebar::Sidebar;
use crate::tank::{Direction, Tank};
use crate::{
    GridPosition,
    Instruction,
    GRID_SIZE,
    GamePhase,
};

pub struct GameState {
    maze: Maze,
    tank: Tank,
    assets: GameAssets,
    current_script: Vec<Instruction>,
    phase: GamePhase,
    execution_step: usize,
    execute_timer: f32,
    game_won: bool,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let assets = GameAssets::new(ctx)?;

        Ok(Self {
            maze: Maze::new(),
            tank: Tank::new(GridPosition::new(0, 0)),
            assets,
            current_script: Vec::new(),
            phase: GamePhase::Plan,
            execution_step: 0,
            execute_timer: 0.0,
            game_won: false,
        })
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

        let dt = ctx.time.delta().as_secs_f32();
        self.tank.update_animation(dt);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.1, 0.1, 1.0]));

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
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(GridPosition::new(x, y).into())
                        .color(color),
                );
            }
        }

        // Draw tank
        let current_body_frame = self.tank.current_frame();
        let body_image = &self.assets.tank.body_sprites[current_body_frame];
        let tank_pos_rect: graphics::Rect = self.tank.pos().into();

        let rotation_angle = match self.tank.direction() {
            Direction::Up => 0.0,
            Direction::Right => std::f32::consts::PI / 2.0,
            Direction::Down => std::f32::consts::PI,
            Direction::Left => 3.0 * std::f32::consts::PI / 2.0,
        };

        // Draw tank body
        canvas.draw(
            body_image,
            graphics::DrawParam::new()
                .dest(tank_pos_rect.point())
                .rotation(rotation_angle)
                .scale(Vector2 {
                    x: tank_pos_rect.w / body_image.width() as f32,
                    y: tank_pos_rect.h / body_image.height() as f32,
                })
                .offset(Vector2 { x: 0.5, y: 0.5 }),
        );

        // Draw turret
        canvas.draw(
            &self.assets.tank.turret_sprite,
            graphics::DrawParam::new()
                .dest(tank_pos_rect.point())
                .rotation(rotation_angle)
                .scale(Vector2 {
                    x: tank_pos_rect.w / self.assets.tank.turret_sprite.width() as f32,
                    y: tank_pos_rect.h / self.assets.tank.turret_sprite.height() as f32,
                })
                .offset(Vector2 { x: 0.5, y: 0.5 }),
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
