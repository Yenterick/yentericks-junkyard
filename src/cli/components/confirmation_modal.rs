use crossterm::event::{self, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
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
            event::KeyCode::Char('y') | event::KeyCode::Char('Y') => {
                self.state.choice = Some(ConfirmationChoice::Yes);
            }

            event::KeyCode::Char('n') | event::KeyCode::Char('N') => {
                self.state.choice = Some(ConfirmationChoice::No);
            }

            _ => {
                self.reset();
            }
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(20));
        Clear.render(centered_area, buf);

        let block: Block<'_> = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::BABY_BLUE)
            .fg(ColorScheme::INK_BLACK);

        Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::BABY_BLUE)
            .fg(ColorScheme::INK_BLACK)
            .title(Span::from(" Confirmation ").into_centered_line())
            .render(centered_area, buf);

        let inner: Rect = block.inner(centered_area);

        let [text_area, buttons_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)])
                .flex(Flex::Center)
                .margin(1)
                .areas(inner);

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
