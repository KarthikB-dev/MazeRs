use ggez::{
    event,
    graphics,
    input::mouse,
    Context, GameResult,
};

use crate::assets::GameAssets;
use crate::sidebar::Sidebar;
use crate::tank::Tank;
use crate::map::{Map, MapPos, hardcoded_map, GRID_SIZE};
use crate::game_types::{GamePhase, Instruction};
use crate::screen::CELL_SIZE;

pub struct GameState {
    map: Map,
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
            map: hardcoded_map(),
            tank: Tank::new(MapPos::new(0, 0)),
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
            Instruction::Move(dir) => self.tank.set_pos(self.tank.pos().moved(&self.map, dir)),
            Instruction::Interact => {}
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

        // Draw map
        for y in 0..GRID_SIZE.1 {
            for x in 0..GRID_SIZE.0 {
                self.map.tiles[y as usize][x as usize].draw(&mut canvas, x, y, CELL_SIZE as f32);
            }
        }

        // Draw tank
        self.tank.draw(&mut canvas, &self.assets);

        // Draw sidebar
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
