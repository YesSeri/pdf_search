use std::{io, thread, time::Duration};
use std::time::Instant;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Widget, Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::widgets::{Paragraph, Wrap};
use crate::search_match::SearchMatch;

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
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
                        let selected_item = stateful_list.items[stateful_list.state.selected().unwrap()].clone();
                        return Ok(selected_item);
                    }
                    KeyCode::Char('q') => {
                        return Err(io::Error::new(io::ErrorKind::Other, "User quit"));
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


        // Iterate through all elements in the `items` app and append some debug text to it.
        let items: Vec<ListItem> = stateful_list.items
            .iter()
            .map(|search_match| {
                let mut lines = vec![Spans::from(search_match.content.clone())];
                lines.push(Spans::from(Span::styled(
                    search_match.path.to_str().unwrap(),
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
                lines.push(Spans::from(Span::styled(
                    format!("Page: {} Line: {}", search_match.page, search_match.line),
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        // We can now render the item list
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Search Results"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        // We can now render the item list
        f.render_stateful_widget(items, chunks[0], &mut stateful_list.state);

        let context = stateful_list.items[stateful_list.state.selected().unwrap()].context.clone();

        // let mut spans = context.lines().map(Spans::from).collect::<Vec<_>>();
        let mut spans = Vec::new();
        for line in context.lines() {
            let lowercase_line = line.to_lowercase();
            let lowercase_term = search_term.to_lowercase();
            if lowercase_line.contains(lowercase_term.as_str()) {
                spans.push(Spans::from(Span::styled(line, Style::default().add_modifier(Modifier::BOLD).bg(Color::LightYellow))));
                // let span = Spans::from(Span::styled(format!(), Style::default().bg(Color::LightBlue).fg(Color::Black)));
            } else {
                spans.push(Spans::from(line));
            }
        }
        let result_info_span = Spans::from(Span::styled(format!("Number of results: {}, below is preview.", stateful_list.items.len().to_string()), Style::default().bg(Color::LightBlue).fg(Color::Black)));
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