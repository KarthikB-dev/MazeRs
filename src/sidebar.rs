use ggez::{
    graphics::{self, Rect},
    input::mouse,
    Context, GameResult,
};
use crate::game_types::{Direction, GamePhase, Instruction};
use crate::screen::{MAP_WIDTH, MAP_HEIGHT, SIDEBAR_WIDTH};

const TURN_INSTRUCTIONS: usize = 10;

pub struct Sidebar;

impl Sidebar {
    pub fn draw(
        ctx: &mut Context, // Note: changed _ctx to ctx to access mouse state
        canvas: &mut graphics::Canvas,
        current_script: &[Instruction],
        phase: GamePhase,
    ) -> GameResult {
        // Get mouse state for button effects
        let mouse_pos = ctx.mouse.position();
        let is_mouse_down = ctx.mouse.button_pressed(mouse::MouseButton::Left);

        // Helper closure to darken color on click
        let get_color = |rect: Rect, base_color: [f32; 4]| -> [f32; 4] {
            if rect.contains(mouse_pos) && is_mouse_down {
                [
                    base_color[0] * 0.7, // Darken RGB by 30%
                    base_color[1] * 0.7,
                    base_color[2] * 0.7,
                    base_color[3],
                ]
            } else {
                base_color
            }
        };

        // Background
        let sidebar_rect = Rect::new(MAP_WIDTH, 0.0, SIDEBAR_WIDTH, MAP_HEIGHT);
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(sidebar_rect)
                .color([0.3, 0.3, 0.3, 1.0]),
        );

        // 1. Available Instructions (Top)
        let instructions = Self::get_all_instructions();
        for (i, instr) in instructions.iter().enumerate() {
            let rect = Self::get_instruction_button_rect(i);

            // Determine color with click effect
            let base_color = [0.5, 0.5, 0.6, 1.0];
            let color = get_color(rect, base_color);

            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(rect)
                    .color(color),
            );

            let text_str = match instr {
                Instruction::Move(Direction::Up) => "Up",
                Instruction::Move(Direction::Down) => "Down",
                Instruction::Move(Direction::Left) => "Left",
                Instruction::Move(Direction::Right) => "Right",
                Instruction::Interact => "Act",
                Instruction::Noop => "Wait",
            };
            let mut text = graphics::Text::new(text_str);
            text.set_scale(16.0);
            canvas.draw(
                &text,
                graphics::DrawParam::new()
                    .dest([rect.x + 5.0, rect.y + 10.0])
                    .color([1.0, 1.0, 1.0, 1.0]),
            );
        }

        // 2. Done/Execute Button (Middle)
        let done_rect = Self::get_done_button_rect();
        let can_finish = current_script.len() == TURN_INSTRUCTIONS;

        let base_color = if can_finish {
            [0.0, 0.8, 0.0, 1.0]
        } else {
            [0.2, 0.2, 0.2, 1.0]
        };
        let color = get_color(done_rect, base_color);

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(done_rect)
                .color(color),
        );
        let button_text = if phase == GamePhase::Execution {
            "Running..."
        } else {
            "Execute"
        };
        let mut done_text = graphics::Text::new(button_text);
        done_text.set_scale(20.0);
        canvas.draw(
            &done_text,
            graphics::DrawParam::new()
                .dest([done_rect.x + 15.0, done_rect.y + 10.0])
                .color([1.0, 1.0, 1.0, 1.0]),
        );

        // 3. Script Slots (Bottom - 2 Columns)
        for i in 0..TURN_INSTRUCTIONS {
            let rect = Self::get_script_slot_rect(i);
            let has_instr = i < current_script.len();

            let base_color = if has_instr {
                [0.6, 0.6, 0.7, 1.0]
            } else {
                [0.4, 0.4, 0.4, 1.0]
            };
            // Apply click effect to slots too (visual feedback for removing)
            let color = get_color(rect, base_color);

            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(rect)
                    .color(color),
            );

            if has_instr {
                let instr = current_script[i];
                let text_str = match instr {
                    Instruction::Move(Direction::Up) => "Up",
                    Instruction::Move(Direction::Down) => "Down",
                    Instruction::Move(Direction::Left) => "Left",
                    Instruction::Move(Direction::Right) => "Right",
                    Instruction::Interact => "Act",
                    Instruction::Noop => "Wait",
                };
                let mut text = graphics::Text::new(format!("{}: {}", i + 1, text_str));
                text.set_scale(16.0);
                canvas.draw(
                     &text,
                     graphics::DrawParam::new()
                         .dest([rect.x + 5.0, rect.y + 5.0])
                         .color([0.0, 0.0, 0.0, 1.0]),
                );
            } else {
                 let mut text = graphics::Text::new(format!("{}: ...", i + 1));
                 text.set_scale(16.0);
                 canvas.draw(
                     &text,
                     graphics::DrawParam::new()
                         .dest([rect.x + 5.0, rect.y + 5.0])
                         .color([0.7, 0.7, 0.7, 1.0]),
                );
            }
        }

        Ok(())
    }

    pub fn handle_click(
        x: f32,
        y: f32,
        current_script: &mut Vec<Instruction>,
        phase: &mut GamePhase,
    ) {
        if *phase != GamePhase::Plan {
            return;
        }

        // Instructions
        let instructions = Self::get_all_instructions();
        for (i, instr) in instructions.iter().enumerate() {
            let rect = Self::get_instruction_button_rect(i);
            if rect.contains([x, y]) {
                if current_script.len() < TURN_INSTRUCTIONS {
                    current_script.push(*instr);
                }
                return;
            }
        }

        // Script Slots (Remove)
        for i in 0..current_script.len() {
            let rect = Self::get_script_slot_rect(i);
            if rect.contains([x, y]) {
                current_script.remove(i);
                return;
            }
        }

        // Done Button
        let done_rect = Self::get_done_button_rect();
        if done_rect.contains([x, y]) {
            if current_script.len() == TURN_INSTRUCTIONS {
                *phase = GamePhase::Execution;
            }
        }
    }

    fn get_instruction_button_rect(index: usize) -> Rect {
        let start_x = MAP_WIDTH + 20.0;
        let start_y = 50.0;
        let w = 70.0;
        let h = 40.0;
        let gap_x = 10.0;
        let gap_y = 10.0;

        let col = (index % 2) as f32;
        let row = (index / 2) as f32;

        Rect::new(start_x + col * (w + gap_x), start_y + row * (h + gap_y), w, h)
    }

    fn get_done_button_rect() -> Rect {
        let start_x = MAP_WIDTH + 50.0;
        let start_y = 210.0;
        Rect::new(start_x, start_y, 150.0, 40.0)
    }

    fn get_script_slot_rect(index: usize) -> Rect {
        let start_x = MAP_WIDTH + 20.0;
        let start_y = 270.0;

        let w = 120.0;
        let h = 25.0;
        let gap_x = 10.0;
        let gap_y = 5.0;

        // Split: First half (0-4) in left col, Second half (5-9) in right col
        let items_per_col = 5;

        let col = (index / items_per_col) as f32;
        let row = (index % items_per_col) as f32;

        Rect::new(start_x + col * (w + gap_x), start_y + row * (h + gap_y), w, h)
    }

    fn get_all_instructions() -> [Instruction; 6] {
        [
            Instruction::Move(Direction::Up),
            Instruction::Move(Direction::Down),
            Instruction::Move(Direction::Left),
            Instruction::Move(Direction::Right),
            Instruction::Interact,
            Instruction::Noop,
        ]
    }
}
