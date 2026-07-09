use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
};

use crate::cli::{events::screen_action::ScreenAction, theme::color_scheme::ColorScheme};

pub struct ErrorModal {
    content: String,
}

impl ErrorModal {
    pub fn new(content: String) -> ErrorModal {
        ErrorModal { content }
    }

    pub fn handle_key(&self, key: KeyEvent) -> ScreenAction {
        match key.code {
            _ => ScreenAction::Confirm,
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let centered_area = area.centered(Constraint::Percentage(56), Constraint::Length(4));
        Clear.render(centered_area, buf);

        let block: Block = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::BRICK_EMBER)
            .fg(ColorScheme::INK_BLACK)
            .title(Span::from(" Error ").into_centered_line());

        let inner: Rect = block.inner(centered_area);

        block.render(centered_area, buf);

        let [_, text_area, button_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(inner);

        Paragraph::new(self.content.to_owned())
            .centered()
            .render(text_area, buf);

        Line::from_iter([
            Span::from("<< Press "),
            Span::from(" any key ")
                .bold()
                .bg(ColorScheme::INK_BLACK)
                .fg(ColorScheme::BRICK_EMBER),
            Span::from(" to continue >>"),
        ])
        .centered()
        .render(button_area, buf);
    }
}
