use crate::messages::CalculatorOp;

#[derive(Debug)]
pub struct Calculator {
    pub display: String,
    pub current_value: f64,
    pub operation: Option<CalculatorOp>,
    pub new_number: bool,
    pub visible: bool,
    pub position: Option<(f32, f32)>,
    pub dragging: bool,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            display: String::from("0"),
            current_value: 0.0,
            operation: None,
            new_number: true,
            visible: false,
            position: None,
            dragging: false,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = Some((x, y));
    }

    pub fn end_drag(&mut self) {
        self.dragging = false;
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn input_digit(&mut self, digit: String) {
        if self.new_number {
            self.display = digit;
            self.new_number = false;
        } else {
            if self.display == "0" {
                self.display = digit;
            } else {
                self.display.push_str(&digit);
            }
        }
    }

    pub fn set_operation(&mut self, op: CalculatorOp) {
        if let Ok(value) = self.display.parse::<f64>() {
            if let Some(operation) = &self.operation {
                self.current_value = self.calculate(self.current_value, value, operation);
                self.display = self.current_value.to_string();
            } else {
                self.current_value = value;
            }
            self.operation = Some(op);
            self.new_number = true;
        }
    }

    pub fn calculate_result(&mut self) {
        if let (Some(operation), Ok(value)) = (&self.operation, self.display.parse::<f64>()) {
            self.current_value = self.calculate(self.current_value, value, operation);
            self.display = self.current_value.to_string();
            self.operation = None;
            self.new_number = true;
        }
    }

    pub fn clear(&mut self) {
        self.display = String::from("0");
        self.current_value = 0.0;
        self.operation = None;
        self.new_number = true;
    }

    fn calculate(&self, a: f64, b: f64, op: &CalculatorOp) -> f64 {
        match op {
            CalculatorOp::Add => a + b,
            CalculatorOp::Subtract => a - b,
            CalculatorOp::Multiply => a * b,
            CalculatorOp::Divide => {
                if b != 0.0 {
                    a / b
                } else {
                    0.0
                }
            }
        }
    }
}
