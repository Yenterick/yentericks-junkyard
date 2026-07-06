use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame: &mut Frame<'_>| render(frame))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

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

fn render(frame: &mut Frame) {}
