use std::default;
use std::error::Error;
use std::fs::File;
use std::{ffi::OsStr, path::Path};

use ini::{Ini, Properties, SectionSetter};
use ratatui::style::{Color, Style};

#[derive(Default, Clone)]
pub struct Theme {
    pub title: TxtTheme,
    pub main: TxtTheme,
    pub highlight: TxtTheme,
    pub alert: TxtTheme,
    pub info: TxtTheme,
    pub border: Border,
}

#[derive(Default, Clone)]
pub struct TxtTheme {
    bg: Color,
    fg: Color,
    bold: bool,
}

#[derive(Default, Clone)]
pub struct Border {
    thickness: usize,
    color: Color,
}

fn get_txt_theme_sec(conf: &Ini, title: &str) -> TxtTheme {
    conf.section(Some(title))
        .map(TxtTheme::from)
        .unwrap_or_default()
}

fn get_border_sec(conf: &Ini, title: &str) -> Border {
    conf.section(Some(title))
        .map(Border::from)
        .unwrap_or_default()
}

fn theme_from_file<T: AsRef<Path>>(file: &T) -> Result<Theme, Box<dyn Error>> {
    let conf = Ini::load_from_file(file)?;

    Ok(Theme {
        title: get_txt_theme_sec(&conf, "Title"),
        alert: get_txt_theme_sec(&conf, "Alert"),
        highlight: get_txt_theme_sec(&conf, "Highlight"),
        main: get_txt_theme_sec(&conf, "Main"),
        info: get_txt_theme_sec(&conf, "Info"),
        border: get_border_sec(&conf, "Border"),
    })
}

impl From<&Properties> for TxtTheme {
    fn from(props: &Properties) -> Self {
        let bg = props.get("bg").map(str_to_color).unwrap_or_default();
        let fg = props.get("fg").map(str_to_color).unwrap_or_default();
        let bold = props
            .get("bold")
            .map(|x| x.to_lowercase() == "true")
            .unwrap_or_default();

        TxtTheme { bg, fg, bold }
    }
}

impl Into<Style> for TxtTheme {
    fn into(self) -> Style {
        let style = Style::new().bg(self.bg).fg(self.fg);

        if self.bold {
            style.bold()
        } else {
            style.not_bold()
        }
    }
}

impl Into<Style> for &TxtTheme {
    fn into(self) -> Style {
        let style = Style::new().bg(self.bg).fg(self.fg);

        if self.bold {
            style.bold()
        } else {
            style.not_bold()
        }
    }
}

impl Into<Style> for Border {
    fn into(self) -> Style {
        // TODO: Implement border thickness
        Style::new().fg(self.color)
    }
}

impl From<&Properties> for Border {
    fn from(props: &Properties) -> Self {
        let thickness = props
            .get("thickness")
            .map(|x| x.parse::<usize>())
            .unwrap_or(Ok(0))
            .unwrap_or_default();
        let color = props.get("color").map(str_to_color).unwrap_or_default();

        Border { thickness, color }
    }
}

fn str_to_color(str: &str) -> Color {
    let lower = str.to_lowercase();
    let s = lower.as_str();

    if let Some(inner) = s.strip_prefix("rgb(").and_then(|s| s.strip_suffix(')')) {
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 3 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                parts[0].trim().parse::<u8>(),
                parts[1].trim().parse::<u8>(),
                parts[2].trim().parse::<u8>(),
            ) {
                return Color::Rgb(r, g, b);
            }
        }
    }

    if let Some(inner) = s.strip_prefix("indexed(").and_then(|s| s.strip_suffix(')')) {
        if let Ok(n) = inner.trim().parse::<u8>() {
            return Color::Indexed(n);
        }
    }

    match s {
        "reset" => Color::Reset,
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "purple" | "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" => Color::DarkGray,
        "lightred" => Color::LightRed,
        "lightgreen" => Color::LightGreen,
        "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan" => Color::LightCyan,
        "white" => Color::White,
        _ => Color::default(),
    }
}

pub fn open_theme(str: &str) -> Theme {
    let path = Path::new("resources")
        .join("themes")
        .join(str.to_string() + ".ini");

    theme_from_file(&path).unwrap_or_default()
}
