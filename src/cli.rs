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
    cursor_pos: usize,
}

impl CommandLineInterface {
    pub fn cursor_pos(&self) -> usize {
        self.cursor_pos
    }
}

struct Areas {
    memory: Rect,
    editor: Rect,
    infos: Rect,
    output: Rect,
    input: Rect,
}

impl CommandLineInterface {
    pub fn new() -> Self {
        execute!(stdout(), SetCursorStyle::BlinkingBlock).ok();
        Self {
            term: ratatui::init(),
            input_char: '\0',
            cursor_pos: 0,
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
                self.cursor_pos,
                mode,
            );
            CommandLineInterface::render_output(frame, areas.output, state.output(), mode);
            CommandLineInterface::render_input(frame, areas.input, self.input_char, mode);
            CommandLineInterface::render_commands(frame, areas.infos, "test.bf".to_string(), mode);
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
        cursor_pos: usize,
        mode: Mode,
    ) {
        let mut content: Vec<Line> = Vec::new();
        let mut line: Vec<Span> = Vec::new();
        for (i, &c) in code.iter().enumerate() {
            line.push(Span::styled(
                c.to_string(),
                Style::default()
                    .fg(match c {
                        '[' | ']' => Color::Green,
                        ',' | '.' => Color::Red,
                        '+' | '-' => Color::Yellow,
                        '<' | '>' => Color::Blue,
                        _ => Color::DarkGray,
                    })
                    .bg(if matches!(mode, Mode::Execution(_)) && i == step {
                        Color::Red
                    } else {
                        Color::Reset
                    }),
            ));
            if line.len() % (area.width.saturating_sub(2) as usize) == 0 {
                content.push(Line::from(line.clone()));
                line.clear()
            }
        }
        content.push(Line::from(line));
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
                area.x + 1 + (cursor_pos as u16) % area.width.saturating_sub(2),
                area.y + 1 + (cursor_pos as u16) / area.width.saturating_sub(2),
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

    fn render_commands(frame: &mut Frame, area: Rect, filename: String, mode: Mode) {
        let content = if mode == Mode::Edition {
            format!("FILE: {} | Run: F5, Quit : Esc", filename)
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
        self.cursor_pos = (self.cursor_pos + 1).min(code.len() - 1);
    }

    fn cursor_left(&mut self) {
        self.cursor_pos = (self.cursor_pos - 1).max(0);
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
                        fevent = FrontendEvent::CharTyped(self.cursor_pos, c);
                        self.cursor_pos += 1;
                    }
                    KeyCode::Left => {
                        self.cursor_left();
                    }
                    KeyCode::Right => {
                        self.cursor_right(state.code());
                    }
                    KeyCode::Backspace => {
                        if self.cursor_pos > 0 {
                            fevent = FrontendEvent::CharErased(self.cursor_pos - 1);
                            self.cursor_left();
                        } else {
                        }
                    }
                    KeyCode::Delete => return FrontendEvent::CharErased(self.cursor_pos),
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
