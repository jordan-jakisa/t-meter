use std::{io, thread, time::Duration};
use chrono::{Local, Timelike};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let now = Local::now();
        let minutes_today = now.hour() as f64 * 60.0 + now.minute() as f64;
        let ratio = minutes_today / 1440.0;

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Length(1)].as_ref())
                .split(f.size());
            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title("t-meter"))
                .gauge_style(Style::default().fg(Color::Green))
                .ratio(ratio)
                .label(format!("{:02}:{:02}", now.hour(), now.minute()));
            let info = Paragraph::new(format!("Elapsed {:.1}% | Press q to quit"),
                ratio * 100.0);
            f.render_widget(info, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q'){
                    break;
                }
            }
        }
        thread::sleep(Duration::from_millis(500));
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
