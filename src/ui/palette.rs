use ratatui::style::Color;

// Color Palette by Catppuccin Frappé: https://github.com/catppuccin/catppuccin

/// Lavender: primary accent for focused borders, highlights, and selections
pub const ACCENT: Color = Color::Rgb(186, 187, 241);
/// Overlay: muted color for unfocused borders and secondary text
pub const MUTED: Color = Color::Rgb(115, 121, 148);
/// Subtext: labels and sub-headlines
pub const SUBTEXT: Color = Color::Rgb(165, 173, 206);
/// Text: primary body text
pub const TEXT: Color = Color::Rgb(198, 208, 245);
/// Surface: subtle surface for badges and elevated elements
pub const SURFACE: Color = Color::Rgb(65, 69, 89);
/// Red: error indicators
pub const RED: Color = Color::Rgb(231, 130, 132);
/// Yellow: warnings and type badges
pub const YELLOW: Color = Color::Rgb(229, 200, 144);
/// Green: success and active indicators
pub const GREEN: Color = Color::Rgb(166, 209, 137);
/// Peach: method badges and secondary highlights
pub const PEACH: Color = Color::Rgb(239, 159, 118);
/// Blue: links and informational elements
pub const BLUE: Color = Color::Rgb(140, 170, 238);
