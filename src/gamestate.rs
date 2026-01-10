use ggez::{
    event,
    graphics,
    input::keyboard::KeyInput,
    Context, GameResult,
};

use std::collections::VecDeque;

use crate::maze::{Maze, Tile};
use crate::tank::Tank;
use crate::{GridPosition, Instruction, TURN_INSTRUCTIONS, DESIRED_FPS, GRID_SIZE};

pub struct GameState {
    maze: Maze,
    tank: Tank,
    instruction_queue: VecDeque<Instruction>,
    game_won: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            maze: Maze::new(),
            tank: Tank::new(GridPosition::new(0, 0)),
            instruction_queue: VecDeque::new(),
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
        while ctx.time.check_update_time(DESIRED_FPS) {
            if !self.game_won {
                if let Some(instr) = self.instruction_queue.pop_front() {
                    self.execute_instruction(instr);
                }
            }
        }
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
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.tank.pos().into())
                .color([1.0, 0.0, 0.0, 1.0]),
        );

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _: bool) -> GameResult {
        if self.instruction_queue.len() < TURN_INSTRUCTIONS {
            if let Some(instr) = Instruction::from_key(&input.event.logical_key) {
                self.instruction_queue.push_back(instr);
            }
        }
        Ok(())
    }
}