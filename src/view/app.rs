use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, ToSpan},
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

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

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Rgb(211, 69, 22))
        .title(
            Span::from(" > Yenterick's Junkyard < ")
                .bold()
                .into_centered_line(),
        )
        .title_bottom(Line::from_iter([
            Span::from(" esc Exit ").bold(),
            Span::from(" h Left ").bold(),
            Span::from(" j Down ").bold(),
            Span::from(" k Up ").bold(),
            Span::from(" l Right ").bold(),
        ]))
        .render(border_area, frame.buffer_mut());
}
