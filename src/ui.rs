use crate::error_handler::{GameError, Result, UiError};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};

pub fn build_ui(mut term: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    term.draw(|rect| {
        let size = rect.size();
        let chunks = get_chunks(size).expect("should return the chunks");
        let copyright = Paragraph::new("Derechos reservados to me 2023")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Copyright")
                    .border_type(BorderType::Plain),
            );

        rect.render_widget(copyright, chunks[2]);
    })?;
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
