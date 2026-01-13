use iced::Task;
use crate::{InventoryApp, Message};
use crate::messages::CalculatorOp;

impl InventoryApp {
    pub fn handle_toggle_calculator(&mut self) {
        self.calculator.toggle_visibility();
        // Set default position if not set
        if self.calculator.visible && self.calculator.position.is_none() {
            self.calculator.set_position(450.0, 200.0);
        }
    }

    pub fn handle_calculator_input(&mut self, digit: String) {
        self.calculator.input_digit(digit);
    }

    pub fn handle_calculator_operation(&mut self, op: CalculatorOp) {
        self.calculator.set_operation(op);
    }

    pub fn handle_calculator_equals(&mut self) {
        self.calculator.calculate_result();
    }

    pub fn handle_calculator_clear(&mut self) {
        self.calculator.clear();
    }

    pub fn handle_calculator_drag_start(&mut self) {
        self.calculator.dragging = true;
    }

    pub fn handle_calculator_drag_move(&mut self, x: f32, y: f32) {
        if self.calculator.dragging {
            // Adjust position to keep calculator centered on cursor
            let calc_x = x - 150.0; // Half of calculator width (300 / 2)
            let calc_y = y - 210.0; // Half of calculator height (420 / 2)
            self.calculator.set_position(calc_x, calc_y);
        }
    }

    pub fn handle_calculator_drag_end(&mut self) -> Task<Message> {
        self.calculator.end_drag();
        self.auto_save()
    }
}
