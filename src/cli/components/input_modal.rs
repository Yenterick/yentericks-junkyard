use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
};

use crate::cli::{events::screen_action::ScreenAction, theme::color_scheme::ColorScheme};

pub struct InputModal {
    pub content: String,
    pub input_buffer: String,
}

impl InputModal {
    pub fn new(content: String) -> InputModal {
        InputModal {
            content,
            input_buffer: String::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> ScreenAction {
        match key.code {
            KeyCode::Char(c) => {
                if self.input_buffer.len() < 16 {
                    if c != ' ' {
                        self.input_buffer.push(c.to_ascii_lowercase());
                    } else {
                        self.input_buffer.push('_');
                    }
                }

                ScreenAction::None
            }

            KeyCode::Backspace => {
                self.input_buffer.pop();
                ScreenAction::None
            }

            KeyCode::Enter => {
                if !self.input_buffer.trim().is_empty()
                    && !self
                        .input_buffer
                        .strip_prefix('_')
                        .is_some_and(|s| !s.is_empty())
                {
                    self.input_buffer =
                        if let Some(value) = self.input_buffer.trim().strip_prefix('_') {
                            value.to_string()
                        } else {
                            self.input_buffer.clone()
                        };
                    return ScreenAction::ReturnInput(self.input_buffer.clone());
                } else {
                    ScreenAction::OpenError(String::from("The name cannot be empty!"))
                }
            }

            KeyCode::Esc => ScreenAction::Back,

            _ => ScreenAction::None,
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let centered_area: Rect = area.centered(Constraint::Percentage(56), Constraint::Length(4));
        Clear.render(centered_area, buf);

        Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::BABY_BLUE)
            .fg(ColorScheme::INK_BLACK)
            .title(Span::from(" Input ").into_centered_line())
            .render(centered_area, buf);

        let [_, text_area, input_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(centered_area);

        Paragraph::new(self.content.to_owned())
            .centered()
            .render(text_area, buf);

        let [input_box] = Layout::horizontal([Constraint::Length(16)])
            .flex(Flex::Center)
            .areas(input_area);

        Paragraph::new(Line::from_iter([Span::styled(
            format!("{:<16}", self.input_buffer),
            Style::default()
                .bg(ColorScheme::INK_BLACK)
                .fg(ColorScheme::BABY_BLUE),
        )]))
        .render(input_box, buf);
    }
}
