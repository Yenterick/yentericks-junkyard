use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

use crate::models::model::Model;
use crate::view::color_scheme::ColorScheme;

#[derive(Debug, Clone, Default)]
pub struct ModelBoxState {
    pub list_state: ListState,
    pub input_buffer: String,
    pub add_mode: bool,
}

impl ModelBoxState {
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

pub struct ModelBox<'a> {
    pub models: &'a [Model],
}

impl<'a> StatefulWidget for ModelBox<'a> {
    type State = ModelBoxState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ModelBoxState) {
        let [vertical_area] = Layout::vertical([Constraint::Length(16)])
            .flex(Flex::Center)
            .areas(area);

        let [models_area] = Layout::horizontal([Constraint::Length(36)])
            .flex(Flex::Center)
            .areas(vertical_area);

        let [list_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(models_area);

        let [vertical_inner_area] = Layout::vertical([Constraint::Length(3)])
            .flex(Flex::Center)
            .areas(area);

        let [input_area] = Layout::horizontal([Constraint::Length(36)])
            .flex(Flex::Center)
            .areas(vertical_inner_area);

        let [input_text] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(input_area);

        if state.add_mode {
            Block::bordered()
                .border_type(BorderType::Rounded)
                .fg(ColorScheme::Blue.color())
                .title(Span::from(" Model Name ").bold())
                .render(input_area, buf);

            Paragraph::new(format!(" {}", state.input_buffer.as_str())).render(input_text, buf);
        } else {
            Block::bordered()
                .border_type(BorderType::Rounded)
                .fg(ColorScheme::Blue.color())
                .title(Span::from("| 🛠️ Models |").bold().into_centered_line())
                .title_bottom(Line::from_iter([
                    Span::from(" ^A ").bold(),
                    Span::styled(
                        "add ",
                        Style::default()
                            .fg(ColorScheme::White.color())
                            .bg(ColorScheme::Blue.color()),
                    ),
                    Span::from(" ^D ").bold(),
                    Span::styled(
                        "delete ",
                        Style::default()
                            .fg(ColorScheme::White.color())
                            .bg(ColorScheme::Blue.color()),
                    ),
                    Span::from(" ^C ").bold(),
                    Span::styled(
                        "create ",
                        Style::default()
                            .fg(ColorScheme::White.color())
                            .bg(ColorScheme::Blue.color()),
                    ),
                    Span::from(" "),
                ]))
                .render(models_area, buf);

            let list: List = List::from_iter(
                self.models
                    .iter()
                    .map(|model| ListItem::from(model.name.as_str())),
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
}
