use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
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
        Self {
            term: ratatui::init(),
            input_char: '\0',
        }
    }

    pub fn render(&mut self, state: &InterpreterState, mode: Mode) {
        let result = self.term.draw(|frame| {
            let areas = CommandLineInterface::compute_layout(frame.area());

            CommandLineInterface::render_memory(frame, areas.memory, state.tape(), state.ptr, mode);
            CommandLineInterface::render_output(frame, areas.output, state.output(), mode);
            CommandLineInterface::render_input(frame, areas.input, self.input_char, mode);

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
            .constraints([Constraint::Min(20), Constraint::Length(7)])
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
            Paragraph::new(input_char.to_string()).block(
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
    }

    pub fn poll(&mut self, mode: Mode) -> FrontendEvent {
        if !event::poll(Duration::from_millis(0)).unwrap_or(false) {
            return FrontendEvent::None;
        }

        let event = match event::read() {
            Ok(e) => e,
            Err(_) => return FrontendEvent::None,
        };

        match event {
            Event::Resize(_, _) => FrontendEvent::Resized,
            Event::Key(key) => match mode {
                Mode::Execution(ExecutionState::AskingInput) => match key.code {
                    KeyCode::Char(c) => {
                        self.input_char = c;
                        FrontendEvent::None
                    }
                    KeyCode::Enter => {
                        let c = self.input_char;
                        self.input_char = '\0';
                        FrontendEvent::CharProvided(c)
                    }
                    _ => FrontendEvent::None,
                },

                Mode::Execution(_) => match key.code {
                    KeyCode::Esc => FrontendEvent::Quit,
                    _ => FrontendEvent::None,
                },

                Mode::Edition => match key.code {
                    KeyCode::Esc => FrontendEvent::Quit,
                    KeyCode::F(5) => FrontendEvent::Run,
                    KeyCode::Char(c) => FrontendEvent::CharTyped(c),
                    _ => FrontendEvent::None,
                },
            },
            _ => FrontendEvent::None,
        }
    }
}

impl Drop for CommandLineInterface {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
