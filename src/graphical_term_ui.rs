use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;
use std::{io, result};

use crate::error::{Result, SimulationError};
use crate::map::{initialize_positions, update_position, Position};
use crate::{initialize_robots, Command, Message, Robot};
use crossterm::event::{self, DisableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::{Block, Borders};
use ratatui::{symbols, Frame, Terminal};

use crate::renderer::{Renderer, Status};

type Tick = u64;

pub struct GraphicalTermRenderer {
    tui: Tui,
    robots: Vec<Robot>,
    positions: HashMap<u32, Robot>,
    rx: Receiver<Message>,
    past_position: HashMap<Position, Tick>,
    current_tick: Tick,
}

#[derive(Default, Eq, PartialEq)]
pub enum KeyHandleResult {
    #[default]
    Continue,
    Exit,
}

impl KeyHandleResult {
    pub fn is_exit(&self) -> bool {
        *self == KeyHandleResult::Exit
    }
}

pub fn run() -> result::Result<(), SimulationError> {
    let (tx, rx) = mpsc::channel::<Message>();
    let robots = initialize_robots(tx);
    let positions = initialize_positions(&robots);
    let mut environnement = GraphicalTermRenderer::new(robots, positions, rx)?;
    loop {
        if Status::Finish == environnement.update()? {
            break;
        }
    }
    Ok(())
}

impl GraphicalTermRenderer {
    fn new(
        robots: Vec<Robot>,
        positions: HashMap<u32, Robot>,
        rx: Receiver<Message>,
    ) -> Result<Self> {
        let tui = Tui::init()?;
        Ok(GraphicalTermRenderer {
            tui,
            robots,
            positions,
            rx,
            current_tick: 0,
            past_position: HashMap::new(),
        })
    }
}

impl Renderer for GraphicalTermRenderer {
    fn update(&mut self) -> Result<Status> {
        for robot in &self.robots {
            let guard = robot.lock().expect("No concurrent call here");
            guard
                .sender
                .send(Command::Move)
                .expect("Failed to send move command");
        }

        for _ in 0..self.robots.len() {
            if let Ok(Message::NewPosition { id, dx, dy }) = self.rx.recv() {
                update_position(&mut self.positions, id, dx, dy);
                for robot in self.positions.values() {
                    let inner = robot.lock().unwrap();
                    self.past_position.insert(inner.coords, self.current_tick);
                }
            }
        }
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(event) = event::read()? {
                if handle_key(self, event).is_exit() {
                    return Ok(Status::Finish);
                }
            }
        }

        self.current_tick += 1;
        self.tui
            .terminal
            .draw(|f| ui(f, &self.positions, &self.past_position, self.current_tick))?;
        Ok(Status::Continue)
    }
}

fn handle_key(_app: &mut GraphicalTermRenderer, event: event::KeyEvent) -> KeyHandleResult {
    use KeyCode::*;
    // TODO
    if event.kind == KeyEventKind::Release {
        return KeyHandleResult::Continue;
    }

    match event.code {
        Char('q') => return KeyHandleResult::Exit,
        Enter | Esc => return KeyHandleResult::Exit,
        _ => {}
    };
    KeyHandleResult::Continue
}

fn ui(
    f: &mut Frame,
    positions: &HashMap<u32, Robot>,
    past_positions: &HashMap<Position, Tick>,
    current_tick: Tick,
) {
    let area = f.size();
    let map = Canvas::default()
        .block(Block::default().title("Mars Map").borders(Borders::ALL))
        .paint(|ctx| {
            ctx.layer();
            for (position, tick) in past_positions {
                let color = match current_tick - tick {
                    0..=5 => Color::Red,
                    6..=10 => Color::LightRed,
                    11..=15 => Color::LightYellow,
                    16..=25 => Color::White,
                    26..=35 => Color::LightCyan,
                    36..=50 => Color::LightBlue,
                    _ => Color::Blue,
                };
                ctx.print(
                    position.1.into(),
                    position.0.into(),
                    Span::styled("•", Style::default().fg(color)),
                );
            }
            for robot in positions.values() {
                let color = Color::Green;
                let current_robot = robot.lock().expect("No concurrent call");
                ctx.print(
                    current_robot.coords.1.into(),
                    current_robot.coords.0.into(),
                    Span::styled("X", Style::default().fg(color)),
                );
            }
        })
        .marker(symbols::Marker::Braille)
        .x_bounds([-200.0, 200.0])
        .y_bounds([-100.0, 100.0]);
    f.render_widget(map, area);
}

struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Tui {
    fn init() -> Result<Self> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        Ok(Self {
            terminal: Terminal::new(backend)?,
        })
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        // restore terminal
        if crossterm::terminal::is_raw_mode_enabled().unwrap() {
            let _ = execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            );
            let _ = disable_raw_mode();
            let _ = self.terminal.show_cursor();
        }
    }
}
