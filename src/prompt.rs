use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal,
};

pub enum PromptType {
    YesNo,
    Info,
    Error,
}

pub fn run_prompt(
    terminal: &mut DefaultTerminal,
    title: &str,
    message: &str,
    prompt_type: PromptType,
) -> Result<bool> {
    // Helper closure to draw the prompt
    let draw_prompt = |f: &mut ratatui::Frame| {
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
            .style(Style::default().fg(match prompt_type {
                PromptType::Error => Color::Red,
                _ => Color::Cyan,
            }));

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
                PromptType::Info => Line::from(""),
                PromptType::Error => Line::from(vec![
                    Span::styled("Press ", Style::default()),
                    Span::styled(
                        "enter",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(" to exit.", Style::default()),
                ]),
            },
        ];

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center);

        f.render_widget(paragraph, chunks[1]);
    };

    // For Info prompts, just draw once and return immediately (non-blocking)
    if matches!(prompt_type, PromptType::Info) {
        terminal.draw(draw_prompt)?;
        return Ok(true);
    }

    // For YesNo and Error prompts, wait for user input (blocking)
    loop {
        terminal.draw(draw_prompt)?;

        if let Event::Key(key) = event::read()? {
            match prompt_type {
                PromptType::YesNo => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => return Ok(true),
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => return Ok(false),
                    _ => {}
                },
                PromptType::Error => {
                    if KeyCode::Enter == key.code {
                        return Ok(false);
                    }
                }
                PromptType::Info => unreachable!("Info handled above"),
            }
        }
    }
}
