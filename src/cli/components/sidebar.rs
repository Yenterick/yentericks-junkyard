use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::cli::{
    events::page_status::PageStatus, state::sidebar_state::SidebarState,
    theme::color_scheme::ColorScheme,
};

pub struct Sidebar;

impl StatefulWidget for Sidebar {
    type State = SidebarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut SidebarState) {
        let mut lines: Vec<Line> = Vec::new();

        for (page, status) in &state.page_status {
            let style: Style;
            let icon: &str;

            match status {
                PageStatus::Completed => {
                    style = Style::default()
                        .fg(ColorScheme::AIR_FORCE_BLUE)
                        .add_modifier(Modifier::ITALIC);

                    icon = "✓";
                }

                PageStatus::Locked => {
                    style = Style::default().fg(ColorScheme::PALE_SKY);

                    icon = "○";
                }

                PageStatus::InProcess => {
                    style = Style::default()
                        .fg(ColorScheme::INK_BLACK)
                        .bg(ColorScheme::AIR_FORCE_BLUE)
                        .add_modifier(Modifier::BOLD);

                    icon = "➜";
                }
            }

            lines.push(
                Line::from(vec![
                    Span::raw(" "),
                    Span::raw(icon),
                    Span::raw(" "),
                    Span::raw(format!(
                        "{:<width$}",
                        page.to_string(),
                        width = area.width.saturating_sub(4) as usize
                    )),
                ])
                .style(style),
            );
        }

        Paragraph::new(lines).render(area, buf);
    }
}
