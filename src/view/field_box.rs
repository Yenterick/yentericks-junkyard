use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, ListState, StatefulWidget, Widget},
};

use crate::{
    models::model::{Field, Model},
    view::color_scheme::ColorScheme,
};

#[derive(Debug, Clone, Default)]
pub struct FieldBoxState {
    pub list_state: ListState,
    pub add_mode: bool,
}

impl FieldBoxState {
    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn selected(&self) -> usize {
        if let Some(index) = self.list_state.selected() {
            return index;
        } else {
            return 0;
        }
    }
}

pub struct FieldBox<'a> {
    pub model: &'a Model,
}

impl<'a> StatefulWidget for FieldBox<'a> {
    type State = FieldBoxState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut FieldBoxState) {
        let [vertical_area] = Layout::vertical([Constraint::Length(16)])
            .flex(Flex::Center)
            .areas(area);

        let [fields_area] = Layout::horizontal([Constraint::Length(36)])
            .flex(Flex::Center)
            .areas(vertical_area);

        let [list_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(fields_area);

        Block::bordered()
            .border_type(BorderType::Rounded)
            .fg(ColorScheme::Silver.color())
            .title(
                Span::from(format!("| {} |", self.model.name))
                    .bold()
                    .into_centered_line(),
            )
            .title_bottom(Line::from_iter([
                Span::from(" ^A ").bold(),
                Span::styled(
                    "add ",
                    Style::default()
                        .fg(ColorScheme::White.color())
                        .bg(ColorScheme::Silver.color()),
                ),
                Span::from(" ^D ").bold(),
                Span::styled(
                    "delete ",
                    Style::default()
                        .fg(ColorScheme::White.color())
                        .bg(ColorScheme::Silver.color()),
                ),
                Span::from(" "),
            ]))
            .render(fields_area, buf);

        let list: List = List::from_iter(
            self.model
                .fields
                .iter()
                .map(|field| ListItem::from(field.name.as_str())),
        )
        .highlight_symbol(" > ")
        .scroll_padding(1)
        .highlight_style(
            Style::default()
                .fg(ColorScheme::White.color())
                .bg(ColorScheme::Green.color())
                .add_modifier(Modifier::BOLD | Modifier::ITALIC),
        );

        StatefulWidget::render(list, list_area, buf, &mut state.list_state);
        state.list_state.select(Some(0));
    }
}
