use std::{io, thread, time::Duration};
use std::panic::panic_any;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::thread::sleep;
use std::time::Instant;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Widget, Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, terminal};
use crossterm::terminal::ClearType;
use tui::widgets::{Paragraph, Wrap};
use crate::pdf_opener;
use crate::search_match::SearchMatch;

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn get_selected_item(&self) -> &T {
        &self.items[self.state.selected().unwrap()]
    }
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut sl = StatefulList {
            state: ListState::default(),
            items,
        };
        sl.state.select(Some(0));
        sl
    }
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub fn run(items: Vec<SearchMatch>, search_term: &str) -> Result<SearchMatch, io::Error> {
    let mut stateful_list = StatefulList::with_items(items);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let selected_search_match = run_app(&mut terminal, &mut stateful_list, search_term)?;

    // draw_ui(&mut terminal, &mut stateful_list)?;
    // thread::sleep(Duration::from_millis(4000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    terminal.clear()?;
    Ok(selected_search_match)
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, stateful_list: &mut StatefulList<SearchMatch>, search_term: &str) -> Result<SearchMatch, io::Error> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    loop {
        draw_ui(terminal, stateful_list, search_term)?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        let selected_match = stateful_list.get_selected_item();
                        pdf_opener::open_pdf(selected_match);
                        // I want to redraw the screen after the pdf has opened so it doesnt look weird. We wait a little and then do it.
                        sleep(Duration::from_millis(500));
                        terminal.clear()?;
                    }
                    KeyCode::Char('q') => {
                        let selected_match = stateful_list.items[stateful_list.state.selected().unwrap()].clone();
                        return Ok(selected_match);
                    }
                    KeyCode::Down => stateful_list.next(),
                    KeyCode::Up => stateful_list.previous(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn draw_ui(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, stateful_list: &mut StatefulList<SearchMatch>, search_term: &str) -> Result<(), io::Error> {
    terminal.draw(|f| {
        // Create two chunks with equal horizontal screen space
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());
        let selected_match = &stateful_list.items[stateful_list.state.selected().unwrap()];
        // Iterate through all elements in the `items` app and append some debug text to it.
        let mut current_file_path: Option<PathBuf> = None;
        let items: Vec<ListItem> = stateful_list.items
            .iter()
            .map(|search_match| {
                let mut lines = if current_file_path.is_none() || *current_file_path.as_ref().unwrap() != search_match.path {
                    let mut vec = vec![Spans::from("-".repeat((f.size().width / 2) as usize))];
                    current_file_path = Some(search_match.path.clone());
                    vec.push(Spans::from(Span::styled(
                        search_match.path.to_str().unwrap(),
                        Style::default().add_modifier(Modifier::BOLD),
                    )));
                    vec
                } else {
                    vec![]
                };
                lines.push(Spans::from(search_match.content.to_string()));
                lines.push(Spans::from(Span::styled(
                    format!("Page: {} Line: {}", search_match.page, search_match.line),
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        // We can now render the item list
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(selected_match.path.to_str().unwrap_or("Unknown File")))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        // We can now render the item list
        f.render_stateful_widget(items, chunks[0], &mut stateful_list.state);


        // let mut spans = context.lines().map(Spans::from).collect::<Vec<_>>();
        let mut spans = Vec::new();
        for line in selected_match.context.lines() {
            let lowercase_line = line.to_lowercase();
            let lowercase_term = search_term.to_lowercase();
            if lowercase_line.contains(lowercase_term.as_str()) {
                spans.push(Spans::from(Span::styled(line, Style::default().add_modifier(Modifier::BOLD).bg(Color::LightYellow))));
                // let span = Spans::from(Span::styled(format!(), Style::default().bg(Color::LightBlue).fg(Color::Black)));
            } else {
                spans.push(Spans::from(line));
            }
        }
        let result_info_span = Spans::from(Span::styled(format!("Number of results: {}, below is preview.", stateful_list.items.len()), Style::default().bg(Color::LightBlue).fg(Color::Black)));
        spans.insert(0, result_info_span);

        let paragraph = Paragraph::new(spans).style(Style::default()).wrap(Wrap { trim: true })
            .block(Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::White).fg(Color::Black))
                .title("Preview"));

        f.render_widget(paragraph, chunks[1]);
    })?;
    Ok(())
}
