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
use crate::network::{NetworkManager, Packet};

#[derive(PartialEq)]
enum WinState {
    None,
    LocalWin,
    RemoteWin,
    Draw,
}

pub struct GameState {
    map: Map,
    local_tank: Tank,
    remote_tank: Tank,
    assets: GameAssets,
    
    local_script: Vec<Instruction>,
    remote_script: Vec<Instruction>,
    
    phase: GamePhase,
    execution_step: usize,
    execute_timer: f32,
    win_state: WinState,
    
    network: NetworkManager,
}

impl GameState {
    pub fn new_multiplayer(ctx: &mut Context, network: NetworkManager) -> GameResult<Self> {
        let assets = GameAssets::new(ctx)?;
        let map = hardcoded_map();

        // Player positions based on player_id
        let (local_pos, remote_pos) = if network.player_id == 0 {
            (MapPos::new(0, 0), MapPos::new(GRID_SIZE.0 - 1, GRID_SIZE.1 - 1))
        } else {
            (MapPos::new(GRID_SIZE.0 - 1, GRID_SIZE.1 - 1), MapPos::new(0, 0))
        };

        Ok(Self {
            map,
            local_tank: Tank::new(local_pos),
            remote_tank: Tank::new(remote_pos),
            assets,
            local_script: Vec::new(),
            remote_script: Vec::new(),
            phase: GamePhase::Plan,
            execution_step: 0,
            execute_timer: 0.0,
            win_state: WinState::None,
            network,
        })
    }

    fn execute_instruction(&mut self, local_instr: Instruction, remote_instr: Instruction) {
        // Move local tank
        match local_instr {
            Instruction::Move(dir) => {
                let new_pos = self.local_tank.pos().moved(&self.map, dir);
                self.local_tank.set_pos(new_pos);
            }
            Instruction::Interact => {
                // Check win condition
                let goal_pos = if self.network.player_id == 0 {
                    MapPos::new(GRID_SIZE.0 - 1, GRID_SIZE.1 - 1)
                } else {
                    MapPos::new(0, 0)
                };
                if self.local_tank.pos().x == goal_pos.x && self.local_tank.pos().y == goal_pos.y {
                    if self.win_state == WinState::None {
                        self.win_state = WinState::LocalWin;
                    } else if self.win_state == WinState::RemoteWin {
                        self.win_state = WinState::Draw;
                    }
                }
            }
            Instruction::Noop => {}
        }

        // Move remote tank
        match remote_instr {
            Instruction::Move(dir) => {
                let new_pos = self.remote_tank.pos().moved(&self.map, dir);
                self.remote_tank.set_pos(new_pos);
            }
            Instruction::Interact => {
                // Check win condition
                let goal_pos = if self.network.player_id == 0 {
                    MapPos::new(0, 0)
                } else {
                    MapPos::new(GRID_SIZE.0 - 1, GRID_SIZE.1 - 1)
                };
                if self.remote_tank.pos().x == goal_pos.x && self.remote_tank.pos().y == goal_pos.y {
                    if self.win_state == WinState::None {
                        self.win_state = WinState::RemoteWin;
                    } else if self.win_state == WinState::LocalWin {
                        self.win_state = WinState::Draw;
                    }
                }
            }
            Instruction::Noop => {}
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Poll network for incoming packets
        while let Ok(packet) = self.network.rx.try_recv() {
            match packet {
                Packet::Moves(moves) => {
                    self.remote_script = moves;
                    // If waiting, transition to execution
                    if self.phase == GamePhase::Execution && self.execution_step == 0 {
                        // Ready to execute
                    }
                }
            }
        }

        if self.win_state != WinState::None {
            return Ok(());
        }

        match self.phase {
            GamePhase::Plan => {
                // Waiting for user input
            }
            GamePhase::Execution => {
                self.execute_timer += ctx.time.delta().as_secs_f32();
                if self.execute_timer >= 0.5 {
                    self.execute_timer = 0.0;
                    
                    let local_len = self.local_script.len();
                    let remote_len = self.remote_script.len();
                    let max_steps = std::cmp::max(local_len, remote_len);

                    if self.execution_step < max_steps {
                        let l_instr = if self.execution_step < local_len {
                            self.local_script[self.execution_step]
                        } else {
                            Instruction::Noop
                        };
                        let r_instr = if self.execution_step < remote_len {
                            self.remote_script[self.execution_step]
                        } else {
                            Instruction::Noop
                        };
                        
                        self.execute_instruction(l_instr, r_instr);
                        self.execution_step += 1;
                    } else {
                        // Execution complete
                        self.phase = GamePhase::Plan;
                        self.local_script.clear();
                        self.remote_script.clear();
                        self.execution_step = 0;
                    }
                }
            }
        }

        let dt = ctx.time.delta().as_secs_f32();
        self.local_tank.update_animation(dt);
        self.remote_tank.update_animation(dt);

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

        // Draw tanks
        self.local_tank.draw(&mut canvas, &self.assets);
        self.remote_tank.draw(&mut canvas, &self.assets);

        // Draw sidebar
        Sidebar::draw(ctx, &mut canvas, &self.local_script, self.phase)?;

        // Draw win state
        if self.win_state != WinState::None {
            let overlay = graphics::Rect::new(0.0, 0.0, crate::screen::MAP_WIDTH, crate::screen::MAP_HEIGHT);
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(overlay)
                    .color([0.0, 0.0, 0.0, 0.8])
            );

            let msg = match self.win_state {
                WinState::LocalWin => "YOU WIN!",
                WinState::RemoteWin => "YOU LOST",
                WinState::Draw => "DRAW!",
                _ => "",
            };

            let mut text = graphics::Text::new(msg);
            text.set_scale(60.0);
            let dims = text.measure(ctx)?;
            let pos = [
                (crate::screen::MAP_WIDTH - dims.x) / 2.0,
                (crate::screen::MAP_HEIGHT - dims.y) / 2.0
            ];
            canvas.draw(
                &text,
                graphics::DrawParam::new()
                    .dest(pos)
                    .color([1.0, 1.0, 0.0, 1.0])
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
        if button != mouse::MouseButton::Left {
            return Ok(());
        }

        if self.win_state != WinState::None {
            return Ok(());
        }

        if self.phase != GamePhase::Plan {
            return Ok(());
        }

        let was_plan_phase = self.phase == GamePhase::Plan;
        Sidebar::handle_click(
            x,
            y,
            &mut self.local_script,
            &mut self.phase,
        );

        if was_plan_phase && self.phase == GamePhase::Execution {
            // Send moves to opponent
            let packet = Packet::Moves(self.local_script.clone());
            if let Err(e) = self.network.tx.try_send(packet) {
                eprintln!("Failed to send moves: {}", e);
                self.phase = GamePhase::Plan;
                return Ok(());
            }

            // Start execution
            self.execution_step = 0;
            self.execute_timer = 0.0;
        }

        Ok(())
    }
}
