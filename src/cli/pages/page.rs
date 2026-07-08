use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::cli::events::screen_action::ScreenAction;

pub trait Page {
    type State;

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State);

    fn handle_key(&mut self, key: KeyEvent, state: &mut Self::State) -> ScreenAction;
}
