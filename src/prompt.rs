use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

/// Show a simple yes/no prompt and return true if user selects yes
pub fn prompt_yes_no(message: &str) -> Result<bool> {
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;

    let result = run_prompt(&mut terminal, message);

    // Cleanup
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    crossterm::terminal::disable_raw_mode()?;

    result
}

fn run_prompt(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, message: &str) -> Result<bool> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Min(7),
                    Constraint::Percentage(40),
                ])
                .split(f.area());

            let block = Block::default()
                .title("Download Sounds")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan));

            let text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    message,
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Press ", Style::default()),
                    Span::styled("y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled(" to download or ", Style::default()),
                    Span::styled("n", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                    Span::styled(" to skip", Style::default()),
                ]),
            ];

            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center);

            f.render_widget(paragraph, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => return Ok(true),
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => return Ok(false),
                _ => {}
            }
        }
    }
}
