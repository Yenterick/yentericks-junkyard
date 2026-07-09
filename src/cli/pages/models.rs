use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, StatefulWidget, Widget},
};

use crate::{
    cli::{
        events::{pages::Pages, screen_action::ScreenAction},
        pages::page::Page,
        state::models_state::ModelsState,
        theme::color_scheme::ColorScheme,
    },
    models::model::Model,
};

pub struct Models {
    models: Vec<Model>,
}

impl Models {
    pub fn new() -> Models {
        Models { models: Vec::new() }
    }
}

impl Page for Models {
    type State = ModelsState;

    fn handle_key(&mut self, key: KeyEvent, state: &mut Self::State) -> ScreenAction {
        match (key.code, key.modifiers) {
            (KeyCode::Char('j'), _) => {
                state.list_state.select_next();
                ScreenAction::None
            }

            (KeyCode::Char('k'), _) => {
                state.list_state.select_previous();
                ScreenAction::None
            }

            (KeyCode::Char('q'), _) => ScreenAction::PreviousPage(Pages::TemplateSelection),

            (KeyCode::Enter, _) => {
                state.selected_model = state.list_state.selected();
                ScreenAction::NextPage(Pages::Fields)
            }

            (KeyCode:: Char('a'), KeyModifiers::CONTROL) => {
                ScreenAction::None
            }

            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                if self.models.len() < 1 {
                    ScreenAction::OpenError(String::from("You need to have at least one model!"))
                } else {
                    ScreenAction::NextPage(Pages::ProjectConfiguration)
                }
            }

            _ => ScreenAction::None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let centered_area: Rect =
            area.centered(Constraint::Percentage(50), Constraint::Percentage(50));
        let [list_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(centered_area);

        Block::bordered()
            .border_type(BorderType::Plain)
            .fg(ColorScheme::BABY_BLUE)
            .title(Span::from(" Models ").into_right_aligned_line().bold())
            .title_bottom(Line::from_iter([
                Span::from(" ^a "),
                Span::from("add ")
                    .bold()
                    .fg(ColorScheme::INK_BLACK)
                    .bg(ColorScheme::BABY_BLUE),
                Span::from(" ^c "),
                Span::from("confirm ")
                    .bold()
                    .fg(ColorScheme::INK_BLACK)
                    .bg(ColorScheme::BABY_BLUE),
                Span::from(" ^d "),
                Span::from("delete ")
                    .bold()
                    .fg(ColorScheme::INK_BLACK)
                    .bg(ColorScheme::BABY_BLUE),
                Span::from(" "),
            ]))
            .render(centered_area, buf);

        let list: List = List::new(
            self.models
                .iter()
                .map(|model: &Model| ListItem::from(format!(" {}", &model.name))),
        )
        .highlight_symbol(" >")
        .highlight_style(
            Style::default()
                .bg(ColorScheme::BABY_BLUE)
                .fg(ColorScheme::INK_BLACK)
                .add_modifier(Modifier::BOLD),
        );

        StatefulWidget::render(list, list_area, buf, &mut state.list_state);
    }
}
