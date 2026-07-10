use crossterm::{
    cursor::{MoveToColumn, MoveUp},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{Write, stdout};

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
