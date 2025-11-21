use anyhow::Result;
use chrono::{Local, Timelike};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::Paragraph,
};
use std::io;

mod quotes;
mod theme;
mod config;

use theme::{Theme, ThemeMode, ColorScheme};
use config::Config;

struct AppState {
    current_theme_index: usize,
    theme_mode: ThemeMode,
    themes: Vec<Theme>,
}

impl AppState {
    fn get_current_theme(&self) -> &Theme {
        &self.themes[self.current_theme_index]
    }
    
    fn get_colors(&self) -> &ColorScheme {
        self.get_current_theme().get_colors(self.theme_mode)
    }
    
    fn cycle_theme(&mut self) {
        self.current_theme_index = (self.current_theme_index + 1) % self.themes.len();
    }
    
    fn toggle_mode(&mut self) {
        self.theme_mode = self.theme_mode.toggle();
    }
}

fn main() -> Result<()> {
    // Load configuration
    let config = Config::load();
    let all_themes = theme::get_all_themes();
    let current_theme_index = all_themes
        .iter()
        .position(|t| t.name == config.theme_name)
        .unwrap_or(0);
    
    let mut app_state = AppState {
        current_theme_index,
        theme_mode: config.get_theme_mode(),
        themes: all_themes,
    };
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal, &mut app_state);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app_state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app_state))?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => return Ok(()),
                        KeyCode::Char('t') => app_state.cycle_theme(),
                        KeyCode::Char('d') => app_state.toggle_mode(),
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui(frame: &mut Frame, app_state: &AppState) {
    let colors = app_state.get_colors();
    let now = Local::now();
    let seconds_since_midnight = now.num_seconds_from_midnight();
    let total_seconds = 24 * 60 * 60;
    let ratio = seconds_since_midnight as f64 / total_seconds as f64;
    
    let area = frame.area();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Length(5), // Spacer
            Constraint::Length(2), // Floating Time
            Constraint::Length(4), // Bar (Thicker - 4 lines)
            Constraint::Length(2), // Markers
            Constraint::Length(2), // Spacer
            Constraint::Length(4), // Zen Quote (Increased height for author)
            Constraint::Min(1),    // Legend (Bottom)
        ])
        .split(area);

    let title = Paragraph::new("time is fleeting")
        .style(Style::default().add_modifier(Modifier::BOLD).fg(colors.title));
    frame.render_widget(title, layout[0]);

    // Calculate dimensions
    let width = layout[3].width as usize;
    if width < 2 { return; }
    
    // Floating Time
    let time_str = now.format("%H:%M").to_string();
    let time_pos = (ratio * width as f64).round() as usize;
    
    // Center the time string on the position
    let time_start = if time_pos > time_str.len() / 2 {
        time_pos - time_str.len() / 2
    } else {
        0
    };
    let time_start = time_start.min(width.saturating_sub(time_str.len()));
    
    let mut time_line = String::from(" ".repeat(width));
    if time_start < time_line.len() {
        let end = (time_start + time_str.len()).min(time_line.len());
        time_line.replace_range(time_start..end, &time_str);
    }
    
    // Vertical line below time (pointer)
    let mut pointer_line = String::from(" ".repeat(width));
    if time_pos < pointer_line.len() {
        pointer_line.replace_range(time_pos..time_pos+1, "|");
    }

    let floating_time = Paragraph::new(format!("{}\n{}", time_line, pointer_line))
        .style(Style::default().fg(colors.foreground));
    frame.render_widget(floating_time, layout[2]);

    // Progress Bar (Denser Segments)
    // Using ▊ (Left 3/4 Block) to simulate 8px segment + 2px gap
    let segment_char = "▊";
    
    let filled_width = (ratio * width as f64).round() as usize;
    
    // We need to build a Line with Spans to handle colors (White vs DarkGray)
    let mut spans = Vec::new();
    
    for i in 0..width {
        if i == time_pos {
            // Vertical line overlay
            spans.push(Span::styled("|", Style::default().fg(colors.progress_indicator)));
        } else if i < filled_width {
            spans.push(Span::styled(segment_char, Style::default().fg(colors.progress_filled)));
        } else {
            spans.push(Span::styled(segment_char, Style::default().fg(colors.progress_empty)));
        }
    }
    
    let line = Line::from(spans);
    // Repeat for 4 lines
    let bar_paragraph = Paragraph::new(vec![line.clone(), line.clone(), line.clone(), line]);
    frame.render_widget(bar_paragraph, layout[3]);

    // Markers
    let mut marker_line_1 = String::from(" ".repeat(width)); // Ticks
    let mut marker_line_2 = String::from(" ".repeat(width)); // Times
    let mut marker_line_3 = String::from(" ".repeat(width)); // Labels

    let mut add_marker = |seconds: u32, _label: &str, sublabel: &str| {
        let pos = (seconds as f64 / total_seconds as f64 * width as f64).round() as usize;
        if pos < width {
            // Tick
            marker_line_1.replace_range(pos..pos+1, "|");
            
            // Time
            let time_str = format!("{:02}:{:02}", seconds / 3600, (seconds % 3600) / 60);
            let t_start = if pos > time_str.len() / 2 { pos - time_str.len() / 2 } else { 0 };
            let t_start = t_start.min(width.saturating_sub(time_str.len()));
            if t_start < width {
                 let end = (t_start + time_str.len()).min(width);
                 marker_line_2.replace_range(t_start..end, &time_str);
            }

            // Label
            let l_start = if pos > sublabel.len() / 2 { pos - sublabel.len() / 2 } else { 0 };
            let l_start = l_start.min(width.saturating_sub(sublabel.len()));
            if l_start < width {
                let end = (l_start + sublabel.len()).min(width);
                marker_line_3.replace_range(l_start..end, sublabel);
            }
        }
    };

    add_marker(0, "00:00", "Wake Up");
    add_marker(7 * 3600, "07:00", "Sunrise"); // Hardcoded Sunrise
    add_marker(12 * 3600, "12:00", "Noon");
    add_marker(19 * 3600, "19:00", "Sunset"); // Hardcoded Sunset
    add_marker(24 * 3600, "24:00", "Sleep");

    let markers_widget = Paragraph::new(format!("{}\n{}\n{}", marker_line_1, marker_line_2, marker_line_3))
        .style(Style::default().fg(colors.marker));
    frame.render_widget(markers_widget, layout[4]);

    // Zen Quotes
    let quotes = quotes::get_quotes();
    let quote_index = (now.hour() as usize) % quotes.len();
    let quote = &quotes[quote_index];
    
    let quote_widget = Paragraph::new(format!("\"{}\"\n~ {}", quote.text, quote.author))
        .style(Style::default().add_modifier(Modifier::ITALIC).fg(colors.quote))
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(quote_widget, layout[6]);

    // Legend
    let elapsed_seconds = seconds_since_midnight;
    let remaining_seconds = total_seconds - elapsed_seconds;
    
    let elapsed_str = format!("{:02}:{:02}", elapsed_seconds / 3600, (elapsed_seconds % 3600) / 60);
    let remaining_str = format!("{:02}:{:02}", remaining_seconds / 3600, (remaining_seconds % 3600) / 60);

    let legend = Line::from(vec![
        Span::styled("█ Elapsed: ", Style::default().fg(colors.legend_elapsed)),
        Span::raw(format!("{}   ", elapsed_str)),
        Span::styled("▊ Remaining: ", Style::default().fg(colors.legend_remaining)),
        Span::raw(remaining_str),
    ]);
    
    let legend_widget = Paragraph::new(legend)
        .alignment(Alignment::Center);
    frame.render_widget(legend_widget, layout[7]);
}
