use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Widget},
};

use crate::cli::theme::color_scheme::ColorScheme;

pub struct Outline;

impl Widget for Outline {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Plain)
            .fg(ColorScheme::AIR_FORCE_BLUE)
            .title(
                Line::from_iter([
                    Span::from(" Yenterick's Junkyard "),
                    Span::from("1.0.0 ").fg(ColorScheme::PALE_SKY).bold(),
                ])
                .alignment(Alignment::Right),
            )
            .title_bottom(Line::from_iter([
                Span::from(" ^q "),
                Span::from("exit ")
                    .bold()
                    .fg(ColorScheme::INK_BLACK)
                    .bg(ColorScheme::AIR_FORCE_BLUE),
                Span::from(" h "),
                Span::from("left ")
                    .bold()
                    .fg(ColorScheme::INK_BLACK)
                    .bg(ColorScheme::AIR_FORCE_BLUE),
                Span::from(" j "),
                Span::from("down ")
                    .bold()
                    .fg(ColorScheme::INK_BLACK)
                    .bg(ColorScheme::AIR_FORCE_BLUE),
                Span::from(" k "),
                Span::from("up ")
                    .bold()
                    .fg(ColorScheme::INK_BLACK)
                    .bg(ColorScheme::AIR_FORCE_BLUE),
                Span::from(" l "),
                Span::from("right ")
                    .bold()
                    .fg(ColorScheme::INK_BLACK)
                    .bg(ColorScheme::AIR_FORCE_BLUE),
                Span::from(" ↵ "),
                Span::from("confirm ")
                    .bold()
                    .fg(ColorScheme::INK_BLACK)
                    .bg(ColorScheme::AIR_FORCE_BLUE),
                Span::from(" "),
            ]))
            .render(area, buf);
    }
}
