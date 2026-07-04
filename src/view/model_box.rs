use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, ListState, StatefulWidget, Widget},
};

use crate::models::model::Model;
use crate::view::app::AppState;
use crate::view::color_scheme::ColorScheme;

#[derive(Debug, Clone)]
pub struct ModelBox {
    pub name: String,
}

impl StatefulWidget for ModelBox {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let [vertical_area] = Layout::vertical([Constraint::Length(16)])
            .flex(Flex::Center)
            .areas(area);

        let [models_area] = Layout::horizontal([Constraint::Length(32)])
            .flex(Flex::Center)
            .areas(vertical_area);

        let [list_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(models_area);

        let [vertical_inner_area] = Layout::vertical([Constraint::Length(3)])
            .flex(Flex::Center)
            .areas(area);

        let [input_area] = Layout::horizontal([Constraint::Length(32)])
            .flex(Flex::Center)
            .areas(vertical_inner_area);

        if state.add_model {
            Block::bordered()
                .border_type(BorderType::Rounded)
                .fg(ColorScheme::Blue.color())
                .title(Span::from(" Model Name ").bold())
                .render(input_area, buf);
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
        }
    }
}
