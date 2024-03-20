use error::SimulationError;
use map::{Position, INITIAL_POSITION};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tracing::trace;
use utils::configure_logger;

mod error;
#[cfg(feature = "term_ui")]
mod graphical_term_ui;
mod map;
mod renderer;
mod utils;

pub const NB_ROBOTS: u32 = 5;
const TICK_DURATION: Duration = Duration::from_millis(10);

#[derive(Debug)]
enum Command {
    Move,
}

#[derive(Debug)]
enum Message {
    NewPosition { id: u32, dx: i32, dy: i32 },
}

pub type Robot = Arc<Mutex<InnerRobot>>;

pub struct InnerRobot {
    pub id: u32,
    pub coords: Position,
    sender: Sender<Command>,
}

fn generate_position_variation(id: u32, command_rx: Receiver<Command>, tx: Sender<Message>) {
    let seed = [id as u8; 32];
    let mut rng = StdRng::from_seed(seed);
    while let Ok(command) = command_rx.recv() {
        match command {
            Command::Move => {
                let dx = rng.gen_range(-1..=1);
                let dy = rng.gen_range(-1..=1);
                trace!("dx {}, dy: {}", dx, dy);
                tx.send(Message::NewPosition { id, dx, dy })
                    .expect("Failed to send position");
            }
        }
    }
}

fn initialize_robots(tx: Sender<Message>) -> Vec<Robot> {
    let mut command_txs = vec![];
    for id in 0..NB_ROBOTS {
        let (command_tx, command_rx) = mpsc::channel::<Command>();
        let tx = tx.clone();
        thread::spawn(move || {
            generate_position_variation(id, command_rx, tx);
        });
        let robot = Arc::new(Mutex::new(InnerRobot {
            id,
            coords: INITIAL_POSITION,
            sender: command_tx.clone(),
        }));
        command_txs.push(robot);
    }
    command_txs
}

fn main() -> std::result::Result<(), SimulationError> {
    let _guard = configure_logger();
    #[cfg(feature = "term_ui")]
    {
        graphical_term_ui::run()
    }
    #[cfg(not(feature = "term_ui"))]
    {
        renderer::run()
    }
}
