use crate::app::{App, Channel};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

/// Render the entire UI
pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main area
            Constraint::Length(3), // Help text
        ])
        .split(f.area());

    render_channels(f, app, chunks[0]);
    render_help(f, chunks[1]);
}

/// Render the channel volume bars (alsamixer style)
fn render_channels(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Relax Player ")
        .borders(Borders::ALL);

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Split into 4 columns (one for each channel)
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(inner);

    // Render each channel
    for (i, channel) in Channel::all().iter().enumerate() {
        render_channel_bar(f, app, *channel, columns[i]);
    }
}

/// Render a single channel's volume bar
fn render_channel_bar(f: &mut Frame, app: &App, channel: Channel, area: Rect) {
    let is_selected = app.selected_channel == channel;
    let volume = app.get_volume(channel);
    let is_muted = app.is_muted(channel);

    // Split area into: title, bar, volume text
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Min(0),    // Bar
            Constraint::Length(1), // Volume %
        ])
        .split(area);

    // Render title
    let title_style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let title = Paragraph::new(channel.name())
        .style(title_style)
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Render volume bar
    let bar = VolumeBar {
        volume,
        is_selected,
        is_muted,
    };
    f.render_widget(bar, chunks[1]);

    // Render volume percentage and mute indicator
    let mute_indicator = if is_muted { " 🔇" } else { "" };
    let vol_text = format!("[{}%]{}", volume, mute_indicator);
    let vol_paragraph =
        Paragraph::new(vol_text)
            .alignment(Alignment::Center)
            .style(if is_selected {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });
    f.render_widget(vol_paragraph, chunks[2]);
}

/// Custom widget for rendering a vertical volume bar
struct VolumeBar {
    volume: u8,
    is_selected: bool,
    is_muted: bool,
}

impl Widget for VolumeBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let height = area.height as usize;
        if height == 0 {
            return;
        }

        // Calculate how many cells should be filled
        let filled_height = ((self.volume as f32 / 100.0) * height as f32).round() as usize;
        let filled_height = filled_height.min(height);

        // Determine the fill character and color
        let (fill_char, empty_char) = if self.is_muted {
            ('░', '░')
        } else {
            ('▓', '┃')
        };

        let fill_color = if self.is_muted {
            Color::DarkGray
        } else if self.is_selected {
            Color::Green
        } else {
            Color::Cyan
        };

        // Center the bar in the available width
        let bar_width = 3;
        let x_offset = (area.width.saturating_sub(bar_width)) / 2;

        // Draw the bar from bottom to top
        for row in 0..height {
            let y = area.y + row as u16;
            let cells_from_bottom = height - row - 1;

            let (ch, style) = if cells_from_bottom < filled_height {
                (fill_char, Style::default().fg(fill_color))
            } else {
                (empty_char, Style::default().fg(Color::DarkGray))
            };

            // Draw the bar character(s)
            for dx in 0..bar_width.min(area.width) {
                let x = area.x + x_offset + dx;
                if x < area.x + area.width {
                    buf[(x, y)].set_char(ch).set_style(style);
                }
            }
        }
    }
}

/// Render help text
fn render_help(f: &mut Frame, area: Rect) {
    let help_lines = vec![Line::from(vec![
        Span::raw("←/→ "),
        Span::styled("h/l", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": Select  ↑/↓ "),
        Span::styled("j/k", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": Volume  "),
        Span::styled("m", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": Mute  "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": Quit"),
    ])];

    let help = Paragraph::new(help_lines)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    f.render_widget(help, area);
}
