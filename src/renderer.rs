use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver};
use std::{result, thread};

use crate::error::{Result, SimulationError};
use crate::map::{initialize_map, initialize_positions, update_positions_map};
use crate::{initialize_robots, Command, Message, Robot, TICK_DURATION};
use crate::{
    map::{CellType, Map2D},
    utils,
};

pub trait Renderer {
    fn update(&mut self) -> Result<Status>;
    fn clean(&self) {
        // Do nothing
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Finish,
    Continue,
}

pub struct TerminalRenderer {
    map: Map2D,
    positions: HashMap<u32, Robot>,
    robots: Vec<Robot>,
    rx: Receiver<Message>,
}

pub fn run() -> result::Result<(), SimulationError> {
    let map = initialize_map();
    let (tx, rx) = mpsc::channel::<Message>();
    let robots = initialize_robots(tx);
    let positions = initialize_positions(&robots);
    let mut environnement = TerminalRenderer::new(map, positions, robots, rx);
    loop {
        if Status::Finish == environnement.update()? {
            break;
        }
    }
    Ok(())
}

impl TerminalRenderer {
    fn new(
        map: Map2D,
        positions: HashMap<u32, Robot>,
        robots: Vec<Robot>,
        rx: Receiver<Message>,
    ) -> Self {
        TerminalRenderer {
            map,
            positions,
            robots,
            rx,
        }
    }

    fn print_cell(c: CellType) {
        match c {
            CellType::Blank => print!(" "),
            CellType::Robot(id) => {
                let c = match id {
                    0 => '@',
                    1 => '%',
                    2 => '#',
                    3 => '*',
                    4 => '+',
                    _ => unimplemented!(),
                };
                print!("{}", c);
            }
        };
    }

    fn draw_map(&self) {
        for row in &self.map {
            for c in row {
                Self::print_cell(*c);
            }
            println!();
        }
    }
}

impl Renderer for TerminalRenderer {
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
                update_positions_map(&mut self.positions, &mut self.map, id, dx, dy);
                self.clean();
                self.draw_map();
            }
        }

        thread::sleep(TICK_DURATION);

        Ok(Status::Continue)
    }

    fn clean(&self) {
        utils::clean_terminal();
    }
}
