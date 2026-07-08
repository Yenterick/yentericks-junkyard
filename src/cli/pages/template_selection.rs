use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, List, ListItem, StatefulWidget, Widget},
};

use crate::cli::{
    events::{pages::Pages, screen_action::ScreenAction},
    models::template::Template,
    pages::page::Page,
    state::template_selection_status::TemplateSelectionStatus,
    theme::color_scheme::ColorScheme,
};

pub struct TemplateSelection {
    pub templates: Vec<Template>,
}

impl Page for TemplateSelection {
    type State = TemplateSelectionStatus;

    fn handle_key(&mut self, key: KeyEvent, state: &mut Self::State) -> ScreenAction {
        match key.code {
            KeyCode::Char('j') => {
                state.list_state.select_next();
                ScreenAction::None
            }

            KeyCode::Char('k') => {
                state.list_state.select_previous();
                ScreenAction::None
            }

            KeyCode::Enter => {
                state.selected_template = state.list_state.selected();
                ScreenAction::NextPage(Pages::Models)
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
            .title(
                Span::from(" Template Selection ")
                    .into_right_aligned_line()
                    .bold(),
            )
            .render(centered_area, buf);

        let list: List = List::new(
            self.templates
                .iter()
                .map(|template: &Template| ListItem::from(format!(" {}", &template.name))),
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
