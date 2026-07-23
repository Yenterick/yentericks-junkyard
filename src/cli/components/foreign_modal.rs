use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, List, ListState, Paragraph, StatefulWidget, Widget},
};

use crate::{
    cli::{events::screen_action::ScreenAction, theme::color_scheme::ColorScheme},
    models::model::Field,
};

pub struct ForeignModal {
    pub list_state: ListState,
    model_names: Vec<String>,
    source: String,
    content: String,
}

impl ForeignModal {
    pub fn new(model_names: Vec<String>, source: String, content: String) -> Self {
        let mut modal = ForeignModal {
            list_state: ListState::default(),
            model_names,
            source,
            content,
        };

        modal.list_state.select_first();
        modal
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> ScreenAction {
        match key.code {
            KeyCode::Char('j') => {
                self.list_state.select_next();
                ScreenAction::None
            }

            KeyCode::Char('k') => {
                self.list_state.select_previous();
                ScreenAction::None
            }

            KeyCode::Esc => ScreenAction::Back,

            KeyCode::Enter => ScreenAction::ReturnField(Field::foreign(
                self.source.to_owned(),
                self.model_names[self.list_state.selected().unwrap_or(0)].to_owned(),
            )),

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
            .title(Span::from(" Add Foreign ").into_centered_line().bold())
            .render(centered_area, buf);

        let [_, text_area, _, bottom_area, _] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(centered_area);

        let [_, list_box_area, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(18),
            Constraint::Fill(1),
        ])
        .areas(bottom_area);

        Paragraph::new(self.content.to_owned())
            .centered()
            .render(text_area, buf);

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

        Paragraph::new(self.content.to_owned())
            .centered()
            .render(text_area, buf);

        let list: List = List::new(
            self.model_names
                .iter()
                .map(|model_name| model_name.to_owned()),
        )
        .highlight_symbol(" >")
        .highlight_style(
            Style::default()
                .bg(ColorScheme::BABY_BLUE)
                .fg(ColorScheme::INK_BLACK)
                .add_modifier(Modifier::BOLD),
        );

        let list_area: Rect = list_block.inner(list_box_area);
        list_block.render(list_box_area, buf);
        StatefulWidget::render(list, list_area, buf, &mut self.list_state);
    }
}
