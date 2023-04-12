use crate::{
    error_handler::{GameError, Result, UiError},
    Event, MenuItem,
};
use crossterm::{
    event::{KeyCode, KeyEvent},
    terminal::disable_raw_mode,
};
use std::{io::Stdout, sync::mpsc::Receiver};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

pub fn build_ui(
    mut term: Terminal<CrosstermBackend<Stdout>>,
    rx: Receiver<Event<KeyEvent>>,
) -> Result<()> {
    let mut active_menu_item = MenuItem::Logs;
    loop {
        term.draw(|rect| {
            let size = rect.size();
            let chunks = get_chunks(size).expect("should return the chunks");
            build_menu(rect, chunks[0], active_menu_item)
                .expect("should build and place the menu item");
            build_info_menu(rect, chunks[2]).expect("should build and place the infor menu item");
            match active_menu_item {
                MenuItem::Logs => rect.render_widget(render_logs(), chunks[1]),
                _ => {}
            }
        })?;
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('e') => {
                    disable_raw_mode()?;
                    term.clear()?;
                    term.show_cursor()?;
                    break;
                }
                KeyCode::Char('l') => active_menu_item = MenuItem::Logs,
                KeyCode::Char('w') => active_menu_item = MenuItem::WorldMap,
                KeyCode::Char('c') => active_menu_item = MenuItem::Configurations,
                KeyCode::Char('q') => active_menu_item = MenuItem::Quests,
                _ => {}
            },
            Event::Tick => {}
        }
    }
    Ok(())
}

fn get_chunks(size: Rect) -> Result<Vec<Rect>> {
    if size.width > 100 && size.height > 20 {
        Ok(Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(size))
    } else {
        Err(GameError::UiBuildError(UiError::new(String::from(
            "the screen size must not be equal to or less than 300x250",
        ))))
    }
}

fn build_menu(
    rect: &mut Frame<CrosstermBackend<Stdout>>,
    chunk: Rect,
    active_menu_item: MenuItem,
) -> Result<()> {
    let menu_titles = vec!["Logs", "WorldMap", "Configurations", "Quests", "Exit"];

    let menu = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let tabs = Tabs::new(menu)
        .select(active_menu_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));
    rect.render_widget(tabs, chunk);
    Ok(())
}

fn build_info_menu(rect: &mut Frame<CrosstermBackend<Stdout>>, chunk: Rect) -> Result<()> {
    // TODO: Use the chunk passed as an argument to divide it in the middle with an horizontal layout
    // to have different types of informations in both sides
    let menu_titles = vec![
        "Poblation",
        "Food",
        "Water",
        "Wood",
        "Iron",
        "Gold",
        "InvPoints",
        "Army",
        "Diplomacy",
    ];
    let menu = menu_titles
        .iter()
        .map(|t| {
            Spans::from(vec![Span::styled(
                t.to_owned(),
                Style::default().fg(Color::White),
            )])
        })
        .collect();
    let tabs = Tabs::new(menu)
        .block(Block::default().title("Info").borders(Borders::ALL))
        .divider(Span::raw("|"))
        .style(Style::default().fg(Color::White));
    rect.render_widget(tabs, chunk);
    Ok(())
}

fn render_logs<'a>() -> Paragraph<'a> {
    let logs = Paragraph::new(vec![
        Spans::from(vec![Span::raw("[ LOGS ] : This is a test of a log")]),
        Spans::from(vec![Span::styled(
            "[ WARNING ] This is a test of a warning log",
            Style::default().fg(Color::Yellow),
        )]),
        Spans::from(vec![Span::styled(
            "[ IMPORTANT ] This is a test of a important log",
            Style::default().fg(Color::Red),
        )]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Logs")
            .border_type(BorderType::Plain),
    );
    logs
}
