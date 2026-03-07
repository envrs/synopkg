use crate::context::Context;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
  backend::CrosstermBackend,
  layout::{Constraint, Direction, Layout},
  style::{Color, Style},
  widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
  Frame, Terminal,
};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputMode {
  Normal,
  Viewing,
}

pub fn run(ctx: Context) -> i32 {
  if ctx.instances.is_empty() {
    eprintln!("No dependencies found");
    return 0;
  }

  let result = run_tui(ctx);

  match result {
    Ok(()) => 0,
    Err(e) => {
      eprintln!("TUI Error: {}", e);
      1
    }
  }
}

fn run_tui(ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
  let stdout = io::stdout();
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut state = TuiState {
    input_mode: InputMode::Normal,
    selected: 0,
    items: collect_items(&ctx),
  };

  loop {
    terminal.draw(|f| ui(f, &mut state, &ctx))?;

    if event::poll(std::time::Duration::from_millis(100))? {
      if let Event::Key(key) = event::read()? {
        match state.input_mode {
          InputMode::Normal => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
              break;
            }
            KeyCode::Char('j') | KeyCode::Down => {
              state.next();
            }
            KeyCode::Char('k') | KeyCode::Up => {
              state.previous();
            }
            KeyCode::Enter => {
              state.input_mode = InputMode::Viewing;
            }
            _ => {}
          },
          InputMode::Viewing => match key.code {
            KeyCode::Char('e') | KeyCode::Esc => {
              state.input_mode = InputMode::Normal;
            }
            KeyCode::Char('q') => {
              break;
            }
            _ => {}
          },
        }
      }
    }
  }

  Ok(())
}

struct TuiState {
  input_mode: InputMode,
  selected: usize,
  items: Vec<DependencyItem>,
}

impl TuiState {
  fn next(&mut self) {
    if self.selected < self.items.len().saturating_sub(1) {
      self.selected += 1;
    }
  }

  fn previous(&mut self) {
    if self.selected > 0 {
      self.selected -= 1;
    }
  }
}

#[derive(Debug, Clone)]
struct DependencyItem {
  name: String,
  current: String,
  expected: String,
  status: String,
  package: String,
}

fn collect_items(ctx: &Context) -> Vec<DependencyItem> {
  let mut items = Vec::new();

  for instance in &ctx.instances {
    if instance.is_invalid() || instance.is_suspect() {
      let current = instance.descriptor.specifier.get_raw().to_string();

      let expected = instance
        .expected_specifier
        .borrow()
        .as_ref()
        .map(|s| s.get_raw().to_string())
        .unwrap_or_else(|| "N/A".to_string());

      let package = instance
        .descriptor
        .package
        .borrow()
        .file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

      let status = format!("{:?}", instance.state.borrow());

      items.push(DependencyItem {
        name: instance.descriptor.name.clone(),
        current,
        expected,
        status,
        package,
      });
    }
  }

  items
}

fn ui(frame: &mut Frame, state: &mut TuiState, _ctx: &Context) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
    .split(frame.area());

  let title_block = Block::default()
    .borders(Borders::ALL)
    .style(Style::default().fg(Color::Cyan))
    .title(" Synopkg Interactive ");

  frame.render_widget(title_block, chunks[0]);

  let stats = format!(
    " Total Issues: {} | Use Up/Down to navigate, Enter for details, q to quit ",
    state.items.len()
  );

  let stats_block = Paragraph::new(stats)
    .style(Style::default().fg(Color::Yellow))
    .alignment(ratatui::layout::Alignment::Center);

  frame.render_widget(stats_block, chunks[2]);

  let list_items: Vec<ListItem> = state
    .items
    .iter()
    .enumerate()
    .map(|(i, item)| {
      let selected_style = if i == state.selected {
        Style::default().bg(Color::Blue).fg(Color::White)
      } else {
        Style::default()
      };

      let line = format!(" {} -> {} ({}) in {} ", item.name, item.expected, item.status, item.package);

      ListItem::new(line).style(selected_style)
    })
    .collect();

  let list = List::new(list_items)
    .block(Block::default().borders(Borders::ALL).title(" Issues "))
    .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

  frame.render_widget(list, chunks[1]);

  if state.input_mode == InputMode::Viewing {
    if let Some(item) = state.items.get(state.selected) {
      let details = format!(
        "Dependency: {}\nCurrent: {}\nExpected: {}\nStatus: {}\nPackage: {}\n\n[e] Back to list  [q] Quit",
        item.name, item.current, item.expected, item.status, item.package,
      );

      let details_block = Block::default()
        .borders(Borders::ALL)
        .title(" Details ")
        .style(Style::default().bg(Color::DarkGray));

      let details_text = Paragraph::new(details).block(details_block).wrap(Wrap { trim: true });

      let details_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(0)])
        .split(chunks[1]);

      frame.render_widget(details_text, details_chunks[0]);
    }
  }
}
