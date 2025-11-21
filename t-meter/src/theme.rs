use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub background: Option<Color>,
    pub foreground: Color,
    pub title: Color,
    pub progress_start: Color,
    pub progress_end: Color,
    pub progress_empty: Color,
    pub progress_indicator: Color,
    pub marker: Color,
    pub marker_label: Color,
    pub quote: Color,
    pub legend_elapsed: Color,
    pub legend_remaining: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn toggle(&self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }
}

impl std::str::FromStr for ThemeMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light" => Ok(ThemeMode::Light),
            "dark" => Ok(ThemeMode::Dark),
            _ => Err(format!("Invalid theme mode: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub light: ColorScheme,
    pub dark: ColorScheme,
}

impl Theme {
    pub fn get_colors(&self, mode: ThemeMode) -> &ColorScheme {
        match mode {
            ThemeMode::Light => &self.light,
            ThemeMode::Dark => &self.dark,
        }
    }
}

// Predefined themes
pub fn get_default_theme() -> Theme {
    Theme {
        name: "default".to_string(),
        light: ColorScheme {
            background: None,
            foreground: Color::White,
            title: Color::White,
            progress_start: Color::White,
            progress_end: Color::White,
            progress_empty: Color::DarkGray,
            progress_indicator: Color::Yellow,
            marker: Color::White,
            marker_label: Color::White,
            quote: Color::Gray,
            legend_elapsed: Color::White,
            legend_remaining: Color::DarkGray,
        },
        dark: ColorScheme {
            background: None,
            foreground: Color::White,
            title: Color::Cyan,
            progress_start: Color::Cyan,
            progress_end: Color::Blue,
            progress_empty: Color::Rgb(40, 40, 40),
            progress_indicator: Color::Yellow,
            marker: Color::Gray,
            marker_label: Color::DarkGray,
            quote: Color::Rgb(100, 100, 100),
            legend_elapsed: Color::Cyan,
            legend_remaining: Color::Rgb(60, 60, 60),
        },
    }
}

pub fn get_ocean_theme() -> Theme {
    Theme {
        name: "ocean".to_string(),
        light: ColorScheme {
            background: None,
            foreground: Color::Rgb(0, 102, 153),
            title: Color::Rgb(0, 102, 153),
            progress_start: Color::Rgb(0, 153, 204),
            progress_end: Color::Rgb(0, 204, 255),
            progress_empty: Color::Rgb(204, 229, 255),
            progress_indicator: Color::Rgb(255, 153, 0),
            marker: Color::Rgb(0, 102, 153),
            marker_label: Color::Rgb(0, 77, 128),
            quote: Color::Rgb(102, 153, 179),
            legend_elapsed: Color::Rgb(0, 153, 204),
            legend_remaining: Color::Rgb(153, 204, 229),
        },
        dark: ColorScheme {
            background: None,
            foreground: Color::Rgb(102, 204, 255),
            title: Color::Rgb(102, 204, 255),
            progress_start: Color::Rgb(51, 153, 255),
            progress_end: Color::Rgb(0, 255, 255),
            progress_empty: Color::Rgb(0, 51, 102),
            progress_indicator: Color::Rgb(255, 204, 0),
            marker: Color::Rgb(153, 204, 255),
            marker_label: Color::Rgb(102, 153, 204),
            quote: Color::Rgb(77, 128, 153),
            legend_elapsed: Color::Rgb(51, 153, 255),
            legend_remaining: Color::Rgb(51, 102, 153),
        },
    }
}

pub fn get_forest_theme() -> Theme {
    Theme {
        name: "forest".to_string(),
        light: ColorScheme {
            background: None,
            foreground: Color::Rgb(34, 139, 34),
            title: Color::Rgb(34, 139, 34),
            progress_start: Color::Rgb(50, 205, 50),
            progress_end: Color::Rgb(154, 205, 50),
            progress_empty: Color::Rgb(193, 225, 193),
            progress_indicator: Color::Rgb(255, 215, 0),
            marker: Color::Rgb(34, 139, 34),
            marker_label: Color::Rgb(0, 100, 0),
            quote: Color::Rgb(107, 142, 35),
            legend_elapsed: Color::Rgb(50, 205, 50),
            legend_remaining: Color::Rgb(144, 238, 144),
        },
        dark: ColorScheme {
            background: None,
            foreground: Color::Rgb(144, 238, 144),
            title: Color::Rgb(144, 238, 144),
            progress_start: Color::Rgb(34, 139, 34),
            progress_end: Color::Rgb(0, 255, 127),
            progress_empty: Color::Rgb(25, 51, 25),
            progress_indicator: Color::Rgb(255, 255, 102),
            marker: Color::Rgb(107, 142, 35),
            marker_label: Color::Rgb(85, 107, 47),
            quote: Color::Rgb(85, 107, 47),
            legend_elapsed: Color::Rgb(34, 139, 34),
            legend_remaining: Color::Rgb(60, 90, 60),
        },
    }
}

pub fn get_sunset_theme() -> Theme {
    Theme {
        name: "sunset".to_string(),
        light: ColorScheme {
            background: None,
            foreground: Color::Rgb(255, 99, 71),
            title: Color::Rgb(255, 99, 71),
            progress_start: Color::Rgb(255, 140, 0),
            progress_end: Color::Rgb(255, 69, 0),
            progress_empty: Color::Rgb(255, 228, 196),
            progress_indicator: Color::Rgb(255, 215, 0),
            marker: Color::Rgb(255, 99, 71),
            marker_label: Color::Rgb(205, 92, 92),
            quote: Color::Rgb(188, 143, 143),
            legend_elapsed: Color::Rgb(255, 140, 0),
            legend_remaining: Color::Rgb(255, 182, 193),
        },
        dark: ColorScheme {
            background: None,
            foreground: Color::Rgb(255, 182, 193),
            title: Color::Rgb(255, 182, 193),
            progress_start: Color::Rgb(255, 99, 71),
            progress_end: Color::Rgb(255, 20, 147),
            progress_empty: Color::Rgb(102, 51, 51),
            progress_indicator: Color::Rgb(255, 255, 102),
            marker: Color::Rgb(255, 140, 0),
            marker_label: Color::Rgb(205, 92, 92),
            quote: Color::Rgb(139, 69, 19),
            legend_elapsed: Color::Rgb(255, 99, 71),
            legend_remaining: Color::Rgb(128, 64, 64),
        },
    }
}

pub fn get_monochrome_theme() -> Theme {
    Theme {
        name: "monochrome".to_string(),
        light: ColorScheme {
            background: None,
            foreground: Color::Black,
            title: Color::Black,
            progress_start: Color::Rgb(20, 20, 20),
            progress_end: Color::Rgb(100, 100, 100),
            progress_empty: Color::Rgb(220, 220, 220),
            progress_indicator: Color::Rgb(0, 0, 0),
            marker: Color::Rgb(40, 40, 40),
            marker_label: Color::Rgb(20, 20, 20),
            quote: Color::Rgb(80, 80, 80),
            legend_elapsed: Color::Rgb(40, 40, 40),
            legend_remaining: Color::Rgb(160, 160, 160),
        },
        dark: ColorScheme {
            background: None,
            foreground: Color::White,
            title: Color::White,
            progress_start: Color::Rgb(220, 220, 220),
            progress_end: Color::Rgb(160, 160, 160),
            progress_empty: Color::Rgb(50, 50, 50),
            progress_indicator: Color::Rgb(255, 255, 255),
            marker: Color::Rgb(220, 220, 220),
            marker_label: Color::Rgb(240, 240, 240),
            quote: Color::Rgb(180, 180, 180),
            legend_elapsed: Color::Rgb(220, 220, 220),
            legend_remaining: Color::Rgb(100, 100, 100),
        },
    }
}

pub fn get_contrast_theme() -> Theme {
    Theme {
        name: "contrast".to_string(),
        light: ColorScheme {
            background: Some(Color::White),
            foreground: Color::Black,
            title: Color::Black,
            progress_start: Color::Blue,
            progress_end: Color::Blue,
            progress_empty: Color::Gray,
            progress_indicator: Color::Red,
            marker: Color::Black,
            marker_label: Color::Black,
            quote: Color::Black,
            legend_elapsed: Color::Blue,
            legend_remaining: Color::Gray,
        },
        dark: ColorScheme {
            background: Some(Color::Black),
            foreground: Color::White,
            title: Color::White,
            progress_start: Color::Cyan,
            progress_end: Color::Cyan,
            progress_empty: Color::DarkGray,
            progress_indicator: Color::Yellow,
            marker: Color::White,
            marker_label: Color::White,
            quote: Color::White,
            legend_elapsed: Color::Cyan,
            legend_remaining: Color::DarkGray,
        },
    }
}

pub fn get_all_themes() -> Vec<Theme> {
    vec![
        get_default_theme(),
        get_ocean_theme(),
        get_forest_theme(),
        get_sunset_theme(),
        get_monochrome_theme(),
        get_contrast_theme(),
    ]
}

pub fn get_theme_by_name(name: &str) -> Option<Theme> {
    get_all_themes().into_iter().find(|t| t.name == name)
}

pub fn get_theme_names() -> Vec<String> {
    get_all_themes().iter().map(|t| t.name.clone()).collect()
}
