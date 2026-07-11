use crossterm::{
    cursor::{MoveToColumn, MoveUp},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io::{self, Write, stdout};
use std::{thread, time::Duration};
pub struct CommandLineInterface {}

impl CommandLineInterface {
    pub fn print_step_by_step(tape: &[u8], action: char, erase: bool) {
        let mut top_row = String::new();
        let mut content_row = String::new();
        let mut bot_row = String::new();
        for (i, cell) in tape.iter().take(10).enumerate() {
            if i == 0 {
                top_row += "   ┌";
                content_row += &format!(" {} |", action);
                bot_row += "   └";
            }
            if i == 9 {
                top_row += "---┐\n";
                content_row += &format!("{:<3}|\n", cell);
                bot_row += "---┘\n";
            } else {
                top_row += "---┬";
                content_row += &format!("{:<3}|", cell);
                bot_row += "---┴";
            }
        }
        let mut out = stdout();
        execute!(
            out,
            MoveToColumn(0),
            SetForegroundColor(Color::DarkRed),
            Print(top_row),
            Print(content_row),
            Print(bot_row),
            Print("\n"),
            ResetColor,
        )
        .unwrap();
        out.flush().unwrap();
        if erase {
            // println!("erase on");
            execute!(stdout(), MoveUp(4)).unwrap();
        }
    }
}

use crate::interpreter::InterpreterState;

pub fn render(state: &InterpreterState) -> io::Result<()> {
    let mut term = ratatui::init();
    let result = term.draw(|frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(3)])
            .split(frame.area());
        let widget = Paragraph::new("text").block(Block::default().borders(Borders::ALL));
        frame.render_widget(widget, chunks[0]);
    });
    thread::sleep(Duration::from_millis(3000));
    ratatui::restore();
    result.map(|_| ())
}
