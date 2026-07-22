use crossterm::event::{self, Event, KeyCode};
use crossterm::{cursor::SetCursorStyle, execute};

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph},
};
use std::io::{self, Write, stdout};
use std::{thread, time::Duration};

use crate::{
    ExecutionState, FrontendEvent, Mode,
    interpreter::{Effect, InterpreterState},
};

const UNACTIVE_COLOR: Color = Color::DarkGray;

pub struct CommandLineInterface {
    term: DefaultTerminal,
    input_char: char,
    cursor_offset: usize,
    cv: CodeVisualisation,
    displated_code_pos: (usize, usize),
}

impl CommandLineInterface {
    pub fn cursor_pos(&self) -> usize {
        self.cursor_offset
    }
}

struct Areas {
    memory: Rect,
    editor: Rect,
    infos: Rect,
    output: Rect,
    input: Rect,
}

struct Cursor {}

impl Cursor {
    // fn display_pos(&self, lines: Vec<Vec<cahr>>, offset: usize) -> (usize, usize) {}
}

pub struct CodeVisualisation {
    lines: Vec<Vec<char>>,
}

impl CodeVisualisation {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }
    pub fn indent(&mut self, code: &Vec<char>) -> &Vec<Vec<char>> {
        self.lines = Vec::new();
        let mut dept: usize = 0;
        let mut line: Vec<char> = Vec::new();
        for &c in code {
            if c == '[' {
                dept += 1;
                line.push(c);
                line.push(' ');
                self.lines.push(line);
                line = vec![' '; 2 * dept];
            } else if c == '\n' {
                line.push(' ');
                self.lines.push(line);
                line = vec![' '; 2 * dept];
            } else if c == ']' {
                line.push(' ');
                self.lines.push(line);
                dept -= 1;
                line = vec![' '; 2 * dept];
                line.push(c);
                line.push(' ');
                self.lines.push(line);
                line = vec![' '; 2 * dept];
            } else {
                line.push(c);
            }
        }
        &self.lines
    }

    pub fn render(
        &mut self,
        width: usize,
        height: usize,
        cursor_offest: usize,
        displayed_code_pos: &mut (usize, usize),
    ) -> (Vec<Line>, (usize, usize)) {
        let mut content: Vec<Line> = Vec::new();
        let (cy, cx) = self.cursor_loc(cursor_offest);
        *displayed_code_pos = (
            displayed_code_pos
                .0
                .min(cx)
                .max(cx.saturating_sub(width - 1)),
            displayed_code_pos
                .1
                .min(cy)
                .max(cy.saturating_sub(height - 1)),
        );
        for (i, line) in self.lines[displayed_code_pos.1
            ..(displayed_code_pos.1.min(self.lines.len()) + height).min(self.lines.len())]
            .iter()
            .enumerate()
        {
            content.push(CodeVisualisation::render_line(
                Vec::from(
                    &line[displayed_code_pos.0.min(line.len())
                        ..(displayed_code_pos.0 + width).min(line.len())],
                ),
                // if cy == i { Some(cx) } else { None },
            ))
        }
        (content, (cx, cy))
    }

    fn cursor_loc(&self, mut cursor_offest: usize) -> (usize, usize) {
        for (i, line) in self.lines.iter().enumerate() {
            if line.len() > cursor_offest {
                return (i, cursor_offest);
            }
            cursor_offest -= line.len();
        }
        (0, 0)
    }

    fn render_line(raw_line: Vec<char>) -> Line<'static> {
        // , cursor_pos: Option<usize>
        let mut line: Vec<Span> = Vec::new();
        for (i, &c) in raw_line.iter().enumerate() {
            line.push(Span::styled(
                c.to_string(),
                Style::default().fg(match c {
                    '[' | ']' => Color::Green,
                    ',' | '.' => Color::Red,
                    '+' | '-' => Color::Yellow,
                    '<' | '>' => Color::Blue,
                    _ => Color::DarkGray,
                }), // .bg(
                    //     if let Some(pos) = cursor_pos
                    //         && i == pos
                    //     {
                    //         Color::Red
                    //     } else {
                    //         Color::Reset
                    //     },
                    // ),
            ));
        }
        Line::from(line)
    }

    pub fn cursor_down(&self, cursor_offest: usize) -> usize {
        let (cy, cx) = self.cursor_loc(cursor_offest);
        if cy + 2 > self.lines.len() {
            cursor_offest
        } else {
            cursor_offest - cx + self.lines[cy].len() + cx.min(self.lines[cy + 1].len() - 1)
        }
    }

    pub fn cursor_up(&self, cursor_offest: usize) -> usize {
        let (cy, cx) = self.cursor_loc(cursor_offest);
        if cy < 1 {
            cursor_offest
        } else {
            cursor_offest + cx.min(self.lines[cy - 1].len() - 1) - cx - self.lines[cy - 1].len()
        }
    }
}

impl CommandLineInterface {
    pub fn new() -> Self {
        execute!(stdout(), SetCursorStyle::BlinkingBlock).ok();
        Self {
            term: ratatui::init(),
            input_char: '\0',
            cursor_offset: 0,
            cv: CodeVisualisation::new(),
            displated_code_pos: (0, 0),
        }
    }

    pub fn render(&mut self, state: &InterpreterState, mode: Mode) {
        let result = self.term.draw(|frame| {
            let areas = CommandLineInterface::compute_layout(frame.area());
            CommandLineInterface::render_memory(frame, areas.memory, state.tape(), state.ptr, mode);
            CommandLineInterface::render_editor(
                frame,
                areas.editor,
                state.code(),
                state.step,
                self.cursor_offset,
                &mut self.cv,
                &mut self.displated_code_pos,
                mode,
            );
            CommandLineInterface::render_output(frame, areas.output, state.output(), mode);
            CommandLineInterface::render_input(frame, areas.input, self.input_char, mode);
            CommandLineInterface::render_commands(
                frame,
                areas.infos,
                "test.bf".to_string(),
                mode,
                &self.cv.cursor_loc(self.cursor_offset),
                self.cursor_offset,
            );
        });
    }

    fn compute_layout(area: Rect) -> Areas {
        let chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(3),
            ])
            .split(area);

        let main_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunk[1]);

        let bot_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Length(20)])
            .split(chunk[2]);

        Areas {
            memory: chunk[0],
            editor: main_chunk[0],
            output: main_chunk[1],
            infos: bot_chunk[0],
            input: bot_chunk[1],
        }
    }

    fn render_memory(frame: &mut Frame, area: Rect, tape: &Vec<u8>, ptr: usize, mode: Mode) {
        let raw_content = tape[..(tape.len() - 1).min(frame.area().width.div_ceil(3) as usize)]
            .iter()
            .map(|v| format!("{:02X}", v))
            .collect::<Vec<String>>()
            .join(" ");
        let content = if matches!(mode, Mode::Execution(_))
            && ptr < frame.area().width.div_ceil(3) as usize
        {
            Line::from(vec![
                Span::raw(&raw_content[..3 * ptr]),
                Span::styled(
                    &raw_content[3 * ptr..3 * ptr + 2],
                    Style::default().fg(Color::LightRed),
                ),
                Span::raw(&raw_content[3 * ptr + 2..]),
            ])
        } else {
            Line::from(raw_content)
        };

        frame.render_widget(
            Paragraph::new(content)
                .block(
                    Block::bordered()
                        .title("Memory")
                        .border_type(BorderType::Rounded)
                        .border_style(if matches!(mode, Mode::Execution(_)) {
                            Style::new().fg(Color::White)
                        } else {
                            Style::new().fg(UNACTIVE_COLOR)
                        }),
                )
                .style(Style::default().fg(if matches!(mode, Mode::Execution(_)) {
                    Color::White
                } else {
                    UNACTIVE_COLOR
                })),
            area,
        );
    }

    fn render_editor(
        frame: &mut Frame,
        area: Rect,
        code: &Vec<char>,
        step: usize,
        cursor_offest: usize,
        cv: &mut CodeVisualisation,
        displayed_code_pos: &mut (usize, usize),
        mode: Mode,
    ) {
        cv.indent(code);
        let (content, (cx, cy)) = cv.render(
            area.width.saturating_sub(2) as usize,
            area.height.saturating_sub(2) as usize,
            cursor_offest,
            displayed_code_pos,
        );
        frame.render_widget(
            Paragraph::new(content).block(
                Block::bordered()
                    .title("Editor")
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(if matches!(mode, Mode::Edition) {
                        Color::White
                    } else {
                        UNACTIVE_COLOR
                    })),
            ),
            area,
        );
        if mode == Mode::Edition {
            frame.set_cursor_position((
                (area.x + 1 + (cx - displayed_code_pos.0) as u16).min(area.width - 2), // area.x + 1 + (cursor_offest as u16) % area.width.saturating_sub(2),
                (area.y + 1 + (cy - displayed_code_pos.1) as u16).min(area.height + 1), // area.y + 1 + (cursor_offest as u16) / area.width.saturating_sub(2),
            ));
        }
    }

    fn render_output(frame: &mut Frame, area: Rect, output: &Vec<u8>, mode: Mode) {
        let content: String = output.iter().map(|&v| v as char).collect();
        frame.render_widget(
            Paragraph::new(content)
                .block(
                    Block::bordered()
                        .title("Output")
                        .border_type(BorderType::Rounded)
                        .border_style(Style::new().fg(if matches!(mode, Mode::Execution(_)) {
                            Color::White
                        } else {
                            UNACTIVE_COLOR
                        })),
                )
                .style(Style::default().fg(if matches!(mode, Mode::Execution(_)) {
                    Color::White
                } else {
                    UNACTIVE_COLOR
                })),
            area,
        );
    }

    fn render_input(frame: &mut Frame, area: Rect, input_char: char, mode: Mode) {
        let active = mode == Mode::Execution(ExecutionState::AskingInput);
        frame.render_widget(
            Paragraph::new(if active && input_char == '\0' {
                Span::styled("Waiting for input", Style::default().fg(UNACTIVE_COLOR))
            } else {
                Span::from(input_char.to_string())
            })
            .block(
                Block::bordered()
                    .title("Input")
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(if active {
                        Color::Red
                    } else {
                        UNACTIVE_COLOR
                    })),
            ),
            area,
        );
        if active {
            frame.set_cursor_position((
                area.x + if input_char == '\0' { 1 } else { 2 }, // colonne absolue
                area.y + 1,                                      // ligne absolue
            ));
        }
    }

    fn render_commands(
        frame: &mut Frame,
        area: Rect,
        filename: String,
        mode: Mode,
        dcp: &(usize, usize),
        offest: usize,
    ) {
        let content = if mode == Mode::Edition {
            format!(
                "FILE: {} | Run: F5, Quit : Esc | {:?} | {}",
                filename, dcp, offest
            )
        } else {
            format!("FILE: {} | Play/Pause : Space, Stop : Esc", filename)
        };
        frame.render_widget(
            Paragraph::new(content)
                .block(Block::default().padding(Padding::uniform(1)))
                .style(Style::default().fg(Color::DarkGray)),
            area,
        );
    }

    fn cursor_right(&mut self, code: &Vec<char>) {
        self.cursor_offset = (self.cursor_offset + 1).min(code.len() - 1);
    }

    fn cursor_left(&mut self) {
        self.cursor_offset = self.cursor_offset.saturating_sub(1);
    }

    pub fn poll(&mut self, state: &InterpreterState, mode: Mode) -> FrontendEvent {
        if !event::poll(Duration::from_millis(0)).unwrap_or(false) {
            return FrontendEvent::None;
        }

        let event = match event::read() {
            Ok(e) => e,
            Err(_) => return FrontendEvent::None,
        };

        let mut fevent = FrontendEvent::None;
        match event {
            Event::Resize(_, _) => return FrontendEvent::Resized,
            Event::Key(key) => match mode {
                Mode::Execution(ExecutionState::AskingInput) => match key.code {
                    KeyCode::Char(c) => {
                        self.input_char = c;
                    }
                    KeyCode::Enter => {
                        fevent = FrontendEvent::CharProvided(self.input_char);
                        self.input_char = '\0';
                    }
                    KeyCode::Backspace => {
                        self.input_char = '\0';
                    }
                    KeyCode::Esc => return FrontendEvent::Stop,
                    _ => {}
                },

                Mode::Execution(_) => match key.code {
                    KeyCode::Esc => return FrontendEvent::Stop,
                    KeyCode::Char(' ') => match mode {
                        Mode::Execution(ExecutionState::Running) => return FrontendEvent::Pause,
                        Mode::Execution(ExecutionState::Paused) => return FrontendEvent::Play,
                        _ => {}
                    },
                    _ => {}
                },

                Mode::Edition => match key.code {
                    KeyCode::Esc => return FrontendEvent::Quit,
                    KeyCode::F(5) => return FrontendEvent::Run,
                    KeyCode::Char(c) => {
                        fevent = FrontendEvent::CharTyped(self.cursor_offset, c);
                        self.cursor_offset += 1;
                    }
                    KeyCode::Left => {
                        self.cursor_left();
                    }
                    KeyCode::Right => {
                        self.cursor_right(state.code());
                    }
                    KeyCode::Down => self.cursor_offset = self.cv.cursor_down(self.cursor_offset),
                    KeyCode::Up => self.cursor_offset = self.cv.cursor_up(self.cursor_offset),
                    KeyCode::Backspace => {
                        if self.cursor_offset > 0 {
                            fevent = FrontendEvent::CharErased(self.cursor_offset - 1);
                            self.cursor_left();
                        } else {
                        }
                    }
                    KeyCode::Delete => return FrontendEvent::CharErased(self.cursor_offset),
                    _ => {}
                },
            },
            _ => {}
        };
        fevent
    }
}

impl Drop for CommandLineInterface {
    fn drop(&mut self) {
        execute!(stdout(), SetCursorStyle::DefaultUserShape).ok();
        ratatui::restore();
    }
}
