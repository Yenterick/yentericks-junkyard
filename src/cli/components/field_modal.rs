use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, List, Paragraph, StatefulWidget, Widget},
};
use tui_checkbox::Checkbox;

use crate::{
    cli::{
        events::screen_action::ScreenAction, state::field_modal_state::FieldModalState,
        theme::color_scheme::ColorScheme,
    },
    models::model::{DataType, Field},
};

pub struct FieldModal {
    pub model_name: String,
    pub state: FieldModalState,
    datatypes: [DataType; 5],
}

impl FieldModal {
    pub fn new(content: String) -> Self {
        FieldModal {
            model_name: content,
            state: FieldModalState::new(),
            datatypes: [
                DataType::String,
                DataType::Integer,
                DataType::Float,
                DataType::Boolean,
                DataType::Date,
            ],
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> ScreenAction {
        match (key.code, key.modifiers) {
            (KeyCode::Char('j'), KeyModifiers::CONTROL) => {
                self.state.list_state.select_next();
                ScreenAction::None
            }

            (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                self.state.list_state.select_previous();
                ScreenAction::None
            }

            (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                self.state.allow_null = !self.state.allow_null;
                ScreenAction::None
            }

            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                self.state.unique = !self.state.unique;
                ScreenAction::None
            }

            (KeyCode::Char(c), _) => {
                if self.state.input_buffer.len() < 16 {
                    if c != ' ' {
                        self.state.input_buffer.push(c.to_ascii_lowercase());
                    } else {
                        self.state.input_buffer.push('_');
                    }
                }

                ScreenAction::None
            }

            (KeyCode::Backspace, _) => {
                self.state.input_buffer.pop();
                ScreenAction::None
            }

            (KeyCode::Enter, _r) => {
                if !self.state.input_buffer.trim().is_empty()
                    && !self
                        .state
                        .input_buffer
                        .strip_prefix('_')
                        .is_some_and(|s| !s.is_empty())
                {
                    self.state.input_buffer =
                        if let Some(value) = self.state.input_buffer.trim().strip_prefix('_') {
                            value.to_string()
                        } else {
                            self.state.input_buffer.clone()
                        };
                    return ScreenAction::ReturnField(Field::new(
                        self.state.input_buffer.clone(),
                        self.datatypes[self.state.list_state.selected().unwrap_or(0)].clone(),
                        self.state.unique,
                        self.state.allow_null,
                    ));
                } else {
                    ScreenAction::OpenError(String::from("The name cannot be empty!"))
                }
            }

            (KeyCode::Esc, _) => ScreenAction::Back,

            _ => ScreenAction::None,
        }
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let centered_area: Rect =
            area.centered(Constraint::Percentage(48), Constraint::Percentage(36));
        Clear.render(centered_area, buf);

        Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::BABY_BLUE)
            .fg(ColorScheme::INK_BLACK)
            .title(Span::from(" Field Append ").into_centered_line().bold())
            .render(centered_area, buf);

        let [left_area, right_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                .margin(1)
                .areas(centered_area);

        let list_block: Block<'_> = Block::bordered()
            .border_type(BorderType::Plain)
            .bg(ColorScheme::INK_BLACK)
            .fg(ColorScheme::BABY_BLUE)
            .title(
                Line::from_iter([
                    Span::from(" ^k "),
                    Span::from("Up ")
                        .bold()
                        .fg(ColorScheme::INK_BLACK)
                        .bg(ColorScheme::BABY_BLUE),
                ])
                .centered(),
            )
            .title_bottom(
                Line::from_iter([
                    Span::from(" ^j "),
                    Span::from("Down ")
                        .bold()
                        .fg(ColorScheme::INK_BLACK)
                        .bg(ColorScheme::BABY_BLUE),
                ])
                .centered(),
            );

        let list_area: Rect = list_block.inner(right_area);
        list_block.render(right_area, buf);

        let list: List = List::new(
            self.datatypes
                .iter()
                .map(|data_type: &DataType| format!(" {}", data_type.as_sequelize())),
        )
        .highlight_symbol(" >")
        .highlight_style(
            Style::default()
                .bg(ColorScheme::BABY_BLUE)
                .fg(ColorScheme::INK_BLACK)
                .add_modifier(Modifier::BOLD),
        );

        StatefulWidget::render(list, list_area, buf, &mut self.state.list_state);

        let [_, input_area, _, null_area, _, unique_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .flex(Flex::Center)
        .areas(left_area);

        let [input_box] = Layout::horizontal([Constraint::Length(16)])
            .flex(Flex::Start)
            .areas(input_area);

        Paragraph::new(Line::from_iter([Span::styled(
            format!("{:<16}", self.state.input_buffer),
            Style::default()
                .bg(ColorScheme::INK_BLACK)
                .fg(ColorScheme::BABY_BLUE),
        )]))
        .render(input_box, buf);

        Checkbox::new(
            Line::from_iter([
                Span::from("allow "),
                Span::from("^n")
                    .bold()
                    .fg(ColorScheme::BABY_BLUE)
                    .bg(ColorScheme::INK_BLACK),
                Span::from("ull"),
            ])
            .bold(),
            self.state.allow_null,
        )
        .render(null_area, buf);
        Checkbox::new(
            Line::from_iter([
                Span::from("^u")
                    .bold()
                    .fg(ColorScheme::BABY_BLUE)
                    .bg(ColorScheme::INK_BLACK),
                Span::from("nique"),
            ])
            .bold(),
            self.state.unique,
        )
        .render(unique_area, buf);
    }
}
