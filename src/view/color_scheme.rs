use ratatui::style::Color;

pub enum ColorScheme {
    DarkBlue,
    Orange,
    Blue,
    Silver,
    Green,
    White,
}

impl ColorScheme {
    pub fn color(&self) -> Color {
        match self {
            &ColorScheme::DarkBlue => Color::Rgb(30, 38, 80),
            &ColorScheme::Orange => Color::Rgb(211, 69, 22),
            &ColorScheme::Blue => Color::Rgb(40, 96, 127),
            &ColorScheme::Silver => Color::Rgb(103, 115, 122),
            &ColorScheme::Green => Color::Rgb(97, 120, 77),
            &ColorScheme::White => Color::Rgb(223, 208, 183),
        }
    }
}
