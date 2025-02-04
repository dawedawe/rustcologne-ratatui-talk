use crossterm::event::{Event, KeyEventKind};
use ratatui::{
    crossterm::event::{self},
    widgets::Paragraph,
    DefaultTerminal,
};
use std::io;

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let some_widget = Paragraph::new("Hello rust.cologne");
            frame.render_widget(some_widget, frame.area());
        })?;

        if matches!(event::read()?, Event::Key(key) if key.kind == KeyEventKind::Press) {
            break Ok(());
        }
    }
}

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}
