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

pub enum PromptType {
    YesNo,
    Info,
}

pub fn run_prompt(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    title: &str,
    message: &str,
    prompt_type: PromptType,
) -> Result<bool> {
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
                .title(title)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan));

            let text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    message,
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                match prompt_type {
                    PromptType::YesNo => Line::from(vec![
                        Span::styled("Press ", Style::default()),
                        Span::styled(
                            "y",
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(" to download or ", Style::default()),
                        Span::styled(
                            "n",
                            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(" to skip", Style::default()),
                    ]),
                    PromptType::Info => Line::from(vec![
                        Span::styled("Press ", Style::default()),
                        Span::styled("enter", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(" to continue.", Style::default()),
                    ]),
                },
            ];

            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center);

            f.render_widget(paragraph, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match prompt_type {
                PromptType::YesNo => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => return Ok(true),
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => return Ok(false),
                    _ => {}
                },
                PromptType::Info => match key.code {
                    KeyCode::Enter => return Ok(true),
                    _ => {}
                },
            }
        }
    }
}
