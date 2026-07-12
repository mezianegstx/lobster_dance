use crossterm::{self, event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};
use std::io::{self, Write, stdout};
use std::{thread, time::Duration};

use crate::{Mode, interpreter::InterpreterState};

pub struct CommandLineInterface {
    term: DefaultTerminal,
}

struct Areas {
    tape: Rect,
    editor: Rect,
    infos: Rect,
    output: Rect,
    input: Rect,
}

struct Area {
    rect: Rect,
    title: String,
    style: Style,
    active_mode: Mode,
}

impl CommandLineInterface {
    pub fn new() -> Self {
        Self {
            term: ratatui::init(),
        }
    }
    // pub fn print_step_by_step(tape: &[u8], action: char, erase: bool) {
    //     let mut top_row = String::new();
    //     let mut content_row = String::new();
    //     let mut bot_row = String::new();
    //     for (i, cell) in tape.iter().take(10).enumerate() {
    //         if i == 0 {
    //             top_row += "   ┌";
    //             content_row += &format!(" {} |", action);
    //             bot_row += "   └";
    //         }
    //         if i == 9 {
    //             top_row += "---┐\n";
    //             content_row += &format!("{:<3}|\n", cell);
    //             bot_row += "---┘\n";
    //         } else {
    //             top_row += "---┬";
    //             content_row += &format!("{:<3}|", cell);
    //             bot_row += "---┴";
    //         }
    //     }
    //     let mut out = stdout();
    //     execute!(
    //         out,
    //         MoveToColumn(0),
    //         SetForegroundColor(Color::DarkRed),
    //         Print(top_row),
    //         Print(content_row),
    //         Print(bot_row),
    //         Print("\n"),
    //         ResetColor,
    //     )
    //     .unwrap();
    //     out.flush().unwrap();
    //     if erase {
    //         // println!("erase on");
    //         execute!(stdout(), MoveUp(4)).unwrap();
    //     }
    // }

    pub fn render(&mut self, state: &InterpreterState, mode: Mode) {
        let result = self.term.draw(|frame| {
            let areas = CommandLineInterface::compute_layout(frame.area());
            let widget = Paragraph::new("text").block(
                Block::bordered()
                    .title("Memory")
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(Color::Red)),
            );
            frame.render_widget(widget, areas.tape);
            frame.render_widget(
                Paragraph::new("editor").block(
                    Block::bordered()
                        .title("Editor")
                        .border_type(BorderType::Rounded)
                        .border_style(Style::new().fg(Color::Red)),
                ),
                areas.editor,
            );
            frame.render_widget(
                Paragraph::new("infos").block(
                    Block::bordered()
                        .title("Commands")
                        .border_type(BorderType::Rounded)
                        .border_style(Style::new().fg(Color::Red)),
                ),
                areas.infos,
            );
            frame.render_widget(
                Paragraph::new("output").block(
                    Block::bordered()
                        .title("Output")
                        .border_type(BorderType::Rounded)
                        .border_style(Style::new().fg(Color::Red)),
                ),
                areas.output,
            );
            frame.render_widget(
                Paragraph::new("input").block(
                    Block::bordered()
                        .title("Input")
                        .border_type(BorderType::Rounded)
                        .border_style(Style::new().fg(Color::Red)),
                ),
                areas.input,
            );
        });
    }

    fn compute_layout(area: Rect) -> Areas {
        let global = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(3)])
            .split(area);

        let main_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(global[1]);

        let left_pannel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(4)])
            .split(main_chunk[0]);

        let right_pannel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(main_chunk[1]);

        Areas {
            tape: global[0],
            editor: left_pannel[0],
            infos: left_pannel[1],
            output: right_pannel[0],
            input: right_pannel[1],
        }
    }
}

impl Drop for CommandLineInterface {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
