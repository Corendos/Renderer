use winit::dpi::LogicalPosition;
use winit::ElementState;

pub struct Input {
    pub mouse_movement: LogicalPosition,
    pub old_mouse_position: Option<LogicalPosition>,
    pub new_mouse_position: Option<LogicalPosition>,
    pub mouse_left_button_state: ElementState,
}

impl Input {
    pub fn new() -> Input {
        Input {
            mouse_movement: LogicalPosition::new(0.0, 0.0),
            old_mouse_position: None,
            new_mouse_position: None,
            mouse_left_button_state: ElementState::Released
        }
    }
    
    pub fn update(&mut self) {
        if self.new_mouse_position.is_some() {
            if self.old_mouse_position.is_some() {
                self.mouse_movement = LogicalPosition::new(
                    self.new_mouse_position.unwrap().x - self.old_mouse_position.unwrap().x,
                    self.new_mouse_position.unwrap().y - self.old_mouse_position.unwrap().y);
            }
            self.old_mouse_position = self.new_mouse_position;
        } else {
            self.mouse_movement = LogicalPosition::new(0.0, 0.0);
        }
    }
}