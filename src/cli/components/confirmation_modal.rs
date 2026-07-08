use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
};

use crate::cli::{
    events::confirmation_choice::ConfirmationChoice, state::confirmation_state::ConfirmationState,
    theme::color_scheme::ColorScheme,
};

pub struct ConfirmationModal {
    content: String,
    pub state: ConfirmationState,
}

impl ConfirmationModal {
    pub fn new(content: String) -> ConfirmationModal {
        ConfirmationModal {
            content,
            state: ConfirmationState::new(),
        }
    }

    pub fn choice(&self) -> Option<ConfirmationChoice> {
        self.state.choice
    }

    pub fn reset(&mut self) {
        self.state.choice = None;
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.state.choice = Some(ConfirmationChoice::Yes);
            }

            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.state.choice = Some(ConfirmationChoice::No);
            }

            _ => {
                self.reset();
            }
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let centered_area = area.centered(Constraint::Percentage(56), Constraint::Percentage(16));
        Clear.render(centered_area, buf);

        Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::BABY_BLUE)
            .fg(ColorScheme::INK_BLACK)
            .title(Span::from(" Confirmation ").into_centered_line())
            .render(centered_area, buf);

        let [_, text_area, _, buttons_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .margin(1)
        .areas(centered_area);

        Paragraph::new(self.content.to_owned())
            .centered()
            .render(text_area, buf);

        Line::from_iter([
            Span::from(" Yes (Y/y) ").bg(ColorScheme::AIR_FORCE_BLUE),
            Span::from("     ").bg(ColorScheme::BABY_BLUE),
            Span::from(" No (N/n) ").bg(ColorScheme::AIR_FORCE_BLUE),
        ])
        .centered()
        .render(buttons_area, buf);
    }
}
