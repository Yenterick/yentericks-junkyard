use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, StatefulWidget, Widget},
};

use crate::{
    cli::{
        events::screen_action::ScreenAction, pages::page::Page, state::fields_state::FieldsState,
        theme::color_scheme::ColorScheme,
    },
    models::model::{Field, Model},
};

pub struct Fields<'a> {
    model: &'a mut Model,
}

impl<'a> Fields<'a> {
    pub fn from(model: &'a mut Model) -> Self {
        Self { model }
    }
}

impl<'a> Page for Fields<'a> {
    type State = FieldsState;

    fn handle_key(&mut self, key: KeyEvent, state: &mut Self::State) -> ScreenAction {
        ScreenAction::None
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
            .title(Span::from(format!(" {} Fields ", self.model.name)).into_right_aligned_line().bold())
            .title_bottom(Line::from_iter([
                Span::from(" ^a "),
                Span::from("add ")
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
            self.model
                .fields
                .iter()
                .map(|field: &Field| ListItem::from(format!(" {}", &field.name))),
        )
        .highlight_symbol(" >")
        .highlight_style(
            Style::default()
                .bg(ColorScheme::BABY_BLUE)
                .fg(ColorScheme::INK_BLACK)
                .add_modifier(Modifier::BOLD),
        );

        StatefulWidget::render(list, list_area, buf, &mut state.list_state);

        if let Some(modal) = &state.input_modal {
            modal.render(area, buf);
        }

        if let Some(modal) = &state.delete_confirmation_modal {
            modal.render(area, buf);
        }
    }
}
