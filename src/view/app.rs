use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Alignment, Constraint, Flex, Layout},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, ToSpan},
    widgets::{Block, BorderType, Padding, Paragraph, Widget, Wrap},
};

#[derive(Debug)]
struct AppState {
    selected_template: TemplateSet,
}

#[derive(Debug)]
struct TemplateSet {
    name: String,
}

// Custom imports
use crate::view::color_scheme::ColorScheme;

pub fn run(mut terminal: DefaultTerminal, path: &str) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|frame: &mut Frame<'_>| render(frame, path))?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, path: &str) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(4)
        .flex(Flex::Center)
        .areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(ColorScheme::Orange.color())
        .title(
            Span::from(" > Yenterick's Junkyard < ")
                .bold()
                .into_centered_line(),
        )
        .title_bottom(Line::from_iter([
            Span::from(" esc ").bold(),
            Span::styled(
                "quit ",
                Style::default()
                    .fg(ColorScheme::White.color())
                    .bg(ColorScheme::Orange.color()),
            ),
            Span::from("  h ").bold(),
            Span::styled(
                "left ",
                Style::default()
                    .fg(ColorScheme::White.color())
                    .bg(ColorScheme::Orange.color()),
            ),
            Span::from("  j ").bold(),
            Span::styled(
                "down ",
                Style::default()
                    .fg(ColorScheme::White.color())
                    .bg(ColorScheme::Orange.color()),
            ),
            Span::from("  k ").bold(),
            Span::styled(
                "up ",
                Style::default()
                    .fg(ColorScheme::White.color())
                    .bg(ColorScheme::Orange.color()),
            ),
            Span::from("  l ").bold(),
            Span::styled(
                "right ",
                Style::default()
                    .fg(ColorScheme::White.color())
                    .bg(ColorScheme::Orange.color()),
            ),
            Span::from("  ↵ ").bold(),
            Span::styled(
                "confirm ",
                Style::default()
                    .fg(ColorScheme::White.color())
                    .bg(ColorScheme::Orange.color()),
            ),
            Span::from(" "),
        ]))
        .render(border_area, frame.buffer_mut());

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(ColorScheme::Blue.color())
        .title(
            Span::from(" > Template Selection < ")
                .bold()
                .into_centered_line(),
        )
        .render(inner_area, frame.buffer_mut());
}
