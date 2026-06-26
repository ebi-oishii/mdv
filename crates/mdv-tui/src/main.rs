use std::io;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use mdv_core::DocState;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;

#[derive(Parser, Debug)]
#[command(name = "mdv-tui", version, about = "Terminal UI for mdv")]
struct Args {
    /// Markdown file to open.
    file: Option<PathBuf>,
    /// Initial mode.
    #[arg(long, value_enum, default_value = "source")]
    mode: Mode,
    /// Open as read-only.
    #[arg(long)]
    read_only: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    Source,
    Preview,
    Diff,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let doc = match &args.file {
        Some(p) => DocState::from_text(std::fs::read_to_string(p)?, Some(p.clone())),
        None => DocState::empty(),
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &doc, &args);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn run<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    doc: &DocState,
    args: &Args,
) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(area);

            let title = match &doc.path {
                Some(p) => format!(" mdv-tui  {} ", p.display()),
                None => " mdv-tui ".to_string(),
            };
            let header = Paragraph::new(title).style(Style::default().fg(Color::Cyan));
            frame.render_widget(header, chunks[0]);

            let body_text = if doc.text.is_empty() {
                "(no file loaded — Phase 0.5 scaffold)\n\nPress q to quit.".to_string()
            } else {
                let preview: String = doc.text.lines().take(20).collect::<Vec<_>>().join("\n");
                format!("{preview}\n\n…\n\nPress q to quit.")
            };
            let body = Paragraph::new(body_text)
                .block(Block::default().borders(Borders::ALL).title(" Source "));
            frame.render_widget(body, chunks[1]);

            let footer = format!(
                " mode: {:?} | {} ",
                args.mode,
                if args.read_only { "RO" } else { "RW" }
            );
            frame.render_widget(
                Paragraph::new(footer).style(Style::default().fg(Color::DarkGray)),
                chunks[2],
            );
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
            {
                break;
            }
        }
    }
    Ok(())
}
