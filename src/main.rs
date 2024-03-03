use input::event::pointer::{ButtonState, PointerButtonEvent};

mod adjust;
mod buttons;
mod events;

use crate::adjust::{GnomeSensitivity, OpenRazerSensitivity, Sensitivity};
use crate::events::listen_for_mouse_button_events;

fn main() {
    listen_for_mouse_button_events(handle_event);
}

fn handle_event(event: PointerButtonEvent) {
    if event.button() == buttons::RIGHT {
        if event.button_state() == ButtonState::Pressed {
            OpenRazerSensitivity::set_relative_normal(1.0 / 10.0);
            GnomeSensitivity::set_relative_normal(10.0);
        } else {
            OpenRazerSensitivity::reset();
            GnomeSensitivity::reset();
        }
    }
}
