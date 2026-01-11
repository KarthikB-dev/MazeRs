use ggez::{
    event,
    graphics,
    input::mouse,
    Context, GameResult,
};
use crate::assets::GameAssets;
use crate::sidebar::Sidebar;
use crate::tank::Tank;
use crate::map::{Map, MapPos, generate_random_map};
use crate::game_types::{GamePhase, Instruction};
use crate::screen::{MAP_WIDTH, MAP_HEIGHT};
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
    assets: GameAssets,
    local_tank: Tank,
    remote_tank: Tank,
    local_script: Vec<Instruction>,
    remote_script: Vec<Instruction>,
    phase: GamePhase,
    execution_step: usize,
    execute_timer: f32,
    win_state: WinState,
    network: NetworkManager,
}

impl GameState {
    pub fn new(ctx: &mut Context, network: NetworkManager) -> GameResult<Self> {
        // Define starting positions for both players
        let map = generate_random_map(25, 25);
        let p1_start = MapPos::new(0, 0);  // Top-left corner
        let p2_start = MapPos::new(map.width as u16 - 1, map.height as u16 - 1);  // Bottom-right corner

        let (local_pos, remote_pos) = if network.player_id == 0 {
            (p1_start, p2_start)
        } else {
            (p2_start, p1_start)
        };

        Ok(Self {
            map: map,
            assets: GameAssets::new(ctx)?,
            local_tank: Tank::new(local_pos),
            remote_tank: Tank::new(remote_pos),
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
                // Check win condition - goal is opposite corner
                let goal_pos = if self.network.player_id == 0 {
                    MapPos::new(4, 4)
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
                // Check win condition - goal is opposite corner
                let goal_pos = if self.network.player_id == 0 {
                    MapPos::new(0, 0)
                } else {
                    MapPos::new(4, 4)
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
                    // If we're waiting for opponent, transition to execution
                    if self.phase == GamePhase::Waiting {
                        self.phase = GamePhase::Execution;
                        self.execution_step = 0;
                    }
                }
            }
        }

        if self.win_state != WinState::None {
            return Ok(());
        }

        match self.phase {
            GamePhase::Plan => {}
            GamePhase::Waiting => {}
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
                        // Execution complete, reset for next turn
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
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.1, 0.1, 1.0]));

        // Draw Maze & Goals
        let p1_goal = MapPos::new(4, 0);  // Top-right corner
        let p2_goal = MapPos::new(0, 4);  // Bottom-left corner

        let (_my_goal, _opp_goal) = if self.network.player_id == 0 {
            (p1_goal, p2_goal)
        } else {
            (p2_goal, p1_goal)
        };

        // Draw map
        let cell_size = (MAP_WIDTH / self.map.width as f32).min(MAP_HEIGHT / self.map.height as f32);
        for (y, tile_row) in self.map.tiles.iter().enumerate() {
            for (x, tile) in tile_row.iter().enumerate() {
                tile.draw(&mut canvas, x as u16, y as u16, cell_size as f32);
            }
        }

        // Draw tanks
        self.local_tank.draw(&mut canvas, &self.assets, cell_size);
        self.remote_tank.draw(&mut canvas, &self.assets, cell_size);

        // Sidebar
        Sidebar::draw(ctx, &mut canvas, &self.local_script, self.phase)?;

        // UI Messages
        if self.phase == GamePhase::Waiting {
             let text = graphics::Text::new("Waiting for Opponent...");
             canvas.draw(
                 &text,
                 graphics::DrawParam::new()
                     .dest([10.0, 10.0])
                     .scale([2.0, 2.0])
             );
        }

        // Draw win state
        if self.win_state != WinState::None {
            let overlay = graphics::Rect::new(0.0, 0.0, MAP_WIDTH, MAP_HEIGHT);
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
            let pos = [(MAP_WIDTH - dims.x)/2.0, (MAP_HEIGHT - dims.y)/2.0];
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
        _button: mouse::MouseButton,
        x: f32,
        y: f32
    ) -> GameResult {
        if self.win_state != WinState::None { return Ok(()); }
        if self.phase != GamePhase::Plan { return Ok(()); }

        let was_plan_phase = self.phase == GamePhase::Plan;
        Sidebar::handle_click(x, y, &mut self.local_script, &mut self.phase);

        // If sidebar changed phase to Execution, handle the transition
        if was_plan_phase && self.phase == GamePhase::Execution {
            // Send our moves to opponent (non-blocking)
            let packet = Packet::Moves(self.local_script.clone());
            if let Err(e) = self.network.tx.try_send(packet) {
                eprintln!("Failed to send packet: {}", e);
                // Reset phase if send failed
                self.phase = GamePhase::Plan;
                return Ok(());
            }

            // Determine if we can start executing immediately
            if !self.remote_script.is_empty() {
                // Opponent already sent their moves, start execution
                self.phase = GamePhase::Execution;
                self.execution_step = 0;
                self.execute_timer = 0.0;
            } else {
                // Wait for opponent
                self.phase = GamePhase::Waiting;
            }
        }

        Ok(())
    }
}
