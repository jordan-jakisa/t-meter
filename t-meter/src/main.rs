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
use config::{Config, ProgressBarStyle};

struct AppState {
    current_theme_index: usize,
    theme_mode: ThemeMode,
    progress_bar_style: ProgressBarStyle,
    themes: Vec<Theme>,
    config: Config,
    input_mode: InputMode,
    input_buffer: String,
    error_message: Option<String>,
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    EditingWakeUp,
    EditingBedTime,
    Help,
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
        self.config.theme_name = self.themes[self.current_theme_index].name.clone();
        let _ = self.config.save();
    }
    
    fn toggle_mode(&mut self) {
        self.theme_mode = self.theme_mode.toggle();
        self.config.theme_mode = match self.theme_mode {
            ThemeMode::Light => "light".to_string(),
            ThemeMode::Dark => "dark".to_string(),
        };
        let _ = self.config.save();
    }

    fn cycle_style(&mut self) {
        self.progress_bar_style = self.progress_bar_style.cycle();
        self.config.progress_bar_style = self.progress_bar_style;
        let _ = self.config.save();
    }

    fn get_wake_up_seconds(&self) -> u32 {
        parse_time(&self.config.wake_up_time)
    }

    fn get_bed_seconds(&self) -> u32 {
        parse_time(&self.config.bed_time)
    }
}

fn parse_time(time_str: &str) -> u32 {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() == 2 {
        let h: u32 = parts[0].parse().unwrap_or(0);
        let m: u32 = parts[1].parse().unwrap_or(0);
        h * 3600 + m * 60
    } else {
        0
    }
}

fn validate_time(time_str: &str) -> Result<u32, String> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid format. Use HH:MM".to_string());
    }
    
    let h: u32 = parts[0].parse().map_err(|_| "Invalid hour".to_string())?;
    let m: u32 = parts[1].parse().map_err(|_| "Invalid minute".to_string())?;
    
    if h >= 24 {
        return Err("Hour must be 0-23".to_string());
    }
    if m >= 60 {
        return Err("Minute must be 0-59".to_string());
    }
    
    Ok(h * 3600 + m * 60)
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
        progress_bar_style: config.progress_bar_style,
        themes: all_themes,
        config,
        input_mode: InputMode::Normal,
        input_buffer: String::new(),
        error_message: None,
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
                    match app_state.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => return Ok(()),
                            KeyCode::Char('t') => app_state.cycle_theme(),
                            KeyCode::Char('d') => app_state.toggle_mode(),
                            KeyCode::Char('s') => app_state.cycle_style(),
                            KeyCode::Char('h') => {
                                app_state.input_mode = InputMode::Help;
                            },
                            KeyCode::Char('w') => {
                                app_state.input_mode = InputMode::EditingWakeUp;
                                app_state.input_buffer = app_state.config.wake_up_time.clone();
                                app_state.error_message = None;
                            },
                            KeyCode::Char('b') => {
                                app_state.input_mode = InputMode::EditingBedTime;
                                app_state.input_buffer = app_state.config.bed_time.clone();
                                app_state.error_message = None;
                            },
                            KeyCode::Char('?') => {
                                let _ = open::that("https://github.com/jordan-jakisa/t-meter/blob/main/docs.md");
                            },
                            _ => {}
                        },
                        InputMode::Help => match key.code {
                            KeyCode::Esc | KeyCode::Char('h') | KeyCode::Char('q') => {
                                app_state.input_mode = InputMode::Normal;
                            },
                            _ => {}
                        },
                        InputMode::EditingWakeUp | InputMode::EditingBedTime => match key.code {
                            KeyCode::Enter => {
                                match validate_time(&app_state.input_buffer) {
                                    Ok(_) => {
                                        match app_state.input_mode {
                                            InputMode::EditingWakeUp => app_state.config.wake_up_time = app_state.input_buffer.clone(),
                                            InputMode::EditingBedTime => app_state.config.bed_time = app_state.input_buffer.clone(),
                                            _ => {}
                                        }
                                        let _ = app_state.config.save();
                                        app_state.input_mode = InputMode::Normal;
                                        app_state.error_message = None;
                                    },
                                    Err(err) => {
                                        app_state.error_message = Some(err);
                                    }
                                }
                                app_state.input_buffer.clear();
                            },
                            KeyCode::Esc => {
                                app_state.input_mode = InputMode::Normal;
                                app_state.input_buffer.clear();
                                app_state.error_message = None;
                            },
                            KeyCode::Backspace => {
                                app_state.input_buffer.pop();
                            },
                            KeyCode::Char(c) => {
                                if c.is_digit(10) || c == ':' {
                                    app_state.input_buffer.push(c);
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

fn ui(frame: &mut Frame, app_state: &AppState) {
    let colors = app_state.get_colors();
    
    // Help Screen
    if app_state.input_mode == InputMode::Help {
        let area = frame.area();
        let help_text = vec![
            " ",
            "┌───────────────────── HELP ─────────────────────┐",
            "│                                                │",
            "│  [h]     Toggle this help screen               │",
            "│  [q]     Quit application                      │",
            "│                                                │",
            "│  [t]     Cycle themes                          │",
            "│  [d]     Toggle dark/light mode                │",
            "│  [s]     Cycle progress bar style              │",
            "│                                                │",
            "│  [w]     Edit wake up time                     │",
            "│  [b]     Edit bed time                         │",
            "│  [?]     Open documentation                    │",
            "│                                                │",
            "│  Press [h], [q], or [Esc] to close             │",
            "│                                                │",
            "└────────────────────────────────────────────────┘",
        ];
        
        let help_paragraph = Paragraph::new(help_text.join("\n"))
            .style(Style::default().fg(colors.foreground))
            .alignment(Alignment::Center);
        
        frame.render_widget(help_paragraph, area);
        return;
    }
    
    let now = Local::now();
    let seconds_since_midnight = now.num_seconds_from_midnight();
    let total_seconds = 24 * 60 * 60;
    let ratio = seconds_since_midnight as f64 / total_seconds as f64;
    
    let area = frame.area();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Top padding
            Constraint::Length(1), // Title
            Constraint::Length(2), // Spacer
            Constraint::Length(2), // Floating Time
            Constraint::Length(4), // Bar
            Constraint::Length(1), // Ticks
            Constraint::Length(1), // Time Labels
            Constraint::Length(1), // Marker Labels
            Constraint::Length(2), // Spacer
            Constraint::Length(4), // Zen Quote
            Constraint::Min(1),    // Legend
        ])
        .split(area);

    // Title
    let title = Paragraph::new("TIME IS FLEETING")
        .style(Style::default().add_modifier(Modifier::BOLD).fg(colors.title))
        .alignment(Alignment::Center);
    frame.render_widget(title, layout[1]);

    // Dimensions
    let width = layout[4].width as usize;
    if width < 2 { return; }
    
    // Floating Time
    let time_str = now.format("%H:%M").to_string();
    let time_pos = (ratio * width as f64).round() as usize;
    let time_pos = time_pos.min(width - 1);
    
    // Calculate safe position for time string to avoid clipping
    let time_len = time_str.len();
    let time_start = if time_pos >= time_len / 2 {
        time_pos - time_len / 2
    } else {
        0
    };
    let time_start = time_start.min(width.saturating_sub(time_len));
    
    let mut time_line = String::from(" ".repeat(width));
    if time_start < width {
        let end = (time_start + time_len).min(width);
        time_line.replace_range(time_start..end, &time_str);
    }
    
    let mut pointer_line = String::from(" ".repeat(width));
    if time_pos < width {
        pointer_line.replace_range(time_pos..time_pos+1, "▼");
    }

    let floating_time = Paragraph::new(format!("{}\n{}", time_line, pointer_line))
        .style(Style::default().fg(colors.foreground).add_modifier(Modifier::BOLD));
    frame.render_widget(floating_time, layout[3]);

    // Progress Bar
    let filled_width = (ratio * width as f64).round() as usize;
    
    let mut spans = Vec::with_capacity(width);
    
    // Helper to interpolate colors
    fn interpolate_color(start: Color, end: Color, t: f64) -> Color {
        if let (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) = (start, end) {
            let r = (r1 as f64 + (r2 as f64 - r1 as f64) * t) as u8;
            let g = (g1 as f64 + (g2 as f64 - g1 as f64) * t) as u8;
            let b = (b1 as f64 + (b2 as f64 - b1 as f64) * t) as u8;
            Color::Rgb(r, g, b)
        } else {
            start // Fallback if not RGB
        }
    }

    for i in 0..width {
        
        // Determine base style
        let (char_str, style) = match app_state.progress_bar_style {
            ProgressBarStyle::Gradient => {
                if i < filled_width {
                    let t = i as f64 / width as f64;
                    let color = interpolate_color(colors.progress_start, colors.progress_end, t);
                    ("█", Style::default().fg(color))
                } else {
                    ("█", Style::default().fg(colors.progress_empty))
                }
            },
            ProgressBarStyle::Grainy => {
                if i < filled_width {
                    ("▓", Style::default().fg(colors.progress_end))
                } else {
                    ("░", Style::default().fg(colors.progress_empty))
                }
            },
            ProgressBarStyle::Analog => {
                if i < filled_width {
                    ("║", Style::default().fg(colors.progress_end))
                } else {
                    ("│", Style::default().fg(colors.progress_empty))
                }
            }
        };

        // Calculate positions for wake and bed time
        let wake_pos = (app_state.get_wake_up_seconds() as f64 / total_seconds as f64 * width as f64).round() as usize;
        let bed_pos = (app_state.get_bed_seconds() as f64 / total_seconds as f64 * width as f64).round() as usize;

        if i == time_pos {
            spans.push(Span::styled("┃", Style::default().fg(colors.progress_indicator).add_modifier(Modifier::BOLD)));
        } else if i == wake_pos || i == bed_pos {
            spans.push(Span::styled("│", Style::default().fg(colors.marker).add_modifier(Modifier::BOLD)));
        } else {
            spans.push(Span::styled(char_str, style));
        }
    }
    
    let line = Line::from(spans);
    let bar_paragraph = Paragraph::new(vec![line.clone(), line.clone(), line.clone(), line]);
    frame.render_widget(bar_paragraph, layout[4]);

    // Markers
    let mut ticks_chars: Vec<char> = vec![' '; width];
    let mut times_chars: Vec<char> = vec![' '; width];
    let mut labels_chars: Vec<char> = vec![' '; width];

    // Determine styles for editable fields
    let wake_style = if app_state.input_mode == InputMode::EditingWakeUp {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(colors.marker)
    };

    let bed_style = if app_state.input_mode == InputMode::EditingBedTime {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(colors.marker)
    };

    let wake_time_display = if app_state.input_mode == InputMode::EditingWakeUp {
        app_state.input_buffer.clone()
    } else {
        format!("{:02}:{:02}", app_state.get_wake_up_seconds()/3600, (app_state.get_wake_up_seconds()%3600)/60)
    };

    let bed_time_display = if app_state.input_mode == InputMode::EditingBedTime {
        app_state.input_buffer.clone()
    } else {
        format!("{:02}:{:02}", app_state.get_bed_seconds()/3600, (app_state.get_bed_seconds()%3600)/60)
    };

    let markers = vec![
        (app_state.get_wake_up_seconds(), wake_time_display, "Wake Up [w]", wake_style),
        (12 * 3600, "12:00".to_string(), "Noon", Style::default().fg(colors.marker)),
        (app_state.get_bed_seconds(), bed_time_display, "Sleep [b]", bed_style),
    ];

    // We need to render markers manually to support different styles per marker
    // But since we are using a single string for the line, we can't easily mix styles in the Paragraph for a single line without using Spans.
    // However, the current implementation builds a String. We should switch to rendering Spans or just use the color for the whole line if we want simplicity, 
    // but the user wants intuitive editing, so highlighting just the time is better.
    // Let's stick to the current char-grid approach but we need to know WHICH style to apply to which char.
    // This is getting complicated for a simple char grid. 
    // Alternative: Render the editable fields separately? No, they need to be positioned correctly.
    
    // Let's use a parallel vector for styles!
    let mut times_styles: Vec<Style> = vec![Style::default().fg(colors.marker); width];

    for (seconds, time_text, label_text, style) in markers {
        let pos = (seconds as f64 / total_seconds as f64 * (width as f64 - 1.0)).round() as usize;
        
        if pos < width {
            // Tick
            ticks_chars[pos] = '│';
            
            // Time
            let t_len = time_text.chars().count();
            let t_start = if pos >= t_len / 2 { pos - t_len / 2 } else { 0 };
            let t_start = t_start.min(width.saturating_sub(t_len));
            if t_start < width {
                for (i, c) in time_text.chars().enumerate() {
                    if t_start + i < width {
                        times_chars[t_start + i] = c;
                        times_styles[t_start + i] = style;
                    }
                }
            }

            // Label
            let l_len = label_text.chars().count();
            let l_start = if pos >= l_len / 2 { pos - l_len / 2 } else { 0 };
            let l_start = l_start.min(width.saturating_sub(l_len));
            if l_start < width {
                for (i, c) in label_text.chars().enumerate() {
                    if l_start + i < width {
                        labels_chars[l_start + i] = c;
                    }
                }
            }
        }
    }

    let ticks_line: String = ticks_chars.into_iter().collect();
    // Construct times line with styles
    let mut times_spans = Vec::new();
    let mut current_style = times_styles[0];
    let mut current_text = String::new();

    for (i, c) in times_chars.iter().enumerate() {
        if times_styles[i] != current_style {
            times_spans.push(Span::styled(current_text.clone(), current_style));
            current_text.clear();
            current_style = times_styles[i];
        }
        current_text.push(*c);
    }
    times_spans.push(Span::styled(current_text, current_style));

    let labels_line: String = labels_chars.into_iter().collect();

    frame.render_widget(Paragraph::new(ticks_line).style(Style::default().fg(colors.marker)), layout[5]);
    frame.render_widget(Paragraph::new(Line::from(times_spans)), layout[6]);
    frame.render_widget(Paragraph::new(labels_line).style(Style::default().fg(colors.marker_label)), layout[7]);

    // Help Text and Error Messages
    if app_state.input_mode != InputMode::Normal && app_state.input_mode != InputMode::Help {
        let help_text = if let Some(ref error) = app_state.error_message {
            format!("❌ Error: {} | Esc to cancel", error)
        } else {
            "Enter time (HH:MM) | Enter to confirm | Esc to cancel".to_string()
        };
        
        let text_color = if app_state.error_message.is_some() {
            Color::Red
        } else {
            Color::Yellow
        };
        
        let help_paragraph = Paragraph::new(help_text)
            .style(Style::default().fg(text_color).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        let area = Rect::new(0, frame.area().height - 1, frame.area().width, 1);
        frame.render_widget(help_paragraph, area);
    }

    // Zen Quotes
    let quotes = quotes::get_quotes();
    let quote_index = (now.hour() as usize) % quotes.len();
    let quote = &quotes[quote_index];
    
    let quote_widget = Paragraph::new(format!("\"{}\"\n~ {}", quote.text, quote.author))
        .style(Style::default().add_modifier(Modifier::ITALIC).fg(colors.quote))
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(quote_widget, layout[9]);

    // Legend
    let elapsed_seconds = seconds_since_midnight;
    let remaining_seconds = total_seconds - elapsed_seconds;
    
    let elapsed_str = format!("{:02}:{:02}", elapsed_seconds / 3600, (elapsed_seconds % 3600) / 60);
    let remaining_str = format!("{:02}:{:02}", remaining_seconds / 3600, (remaining_seconds % 3600) / 60);

    let legend_text = vec![
        Line::from(vec![
            Span::styled("● Elapsed:   ", Style::default().fg(colors.legend_elapsed).add_modifier(Modifier::BOLD)),
            Span::raw(elapsed_str),
        ]),
        Line::from(vec![
            Span::styled("○ Remaining: ", Style::default().fg(colors.legend_remaining).add_modifier(Modifier::BOLD)),
            Span::raw(remaining_str),
        ]),
    ];
    
    let legend_widget = Paragraph::new(legend_text)
        .alignment(Alignment::Center);
    frame.render_widget(legend_widget, layout[10]);
}
