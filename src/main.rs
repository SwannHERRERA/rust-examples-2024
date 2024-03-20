use map::{initialize_map, initialize_positions, update_and_draw_map};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use renderer::TerminalRenderer;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use tracing::trace;
use utils::configure_logger;

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

fn update_position(id: u32, command_rx: Receiver<Command>, tx: Sender<Message>) {
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

fn initialize_robots(tx: Sender<Message>) -> Vec<Sender<Command>> {
    let mut command_txs = vec![];
    for id in 0..NB_ROBOTS {
        let (command_tx, command_rx) = mpsc::channel::<Command>();
        command_txs.push(command_tx.clone());
        let tx = tx.clone();
        thread::spawn(move || {
            update_position(id, command_rx, tx);
        });
    }
    command_txs
}

fn main() {
    // Startup
    let _guard = configure_logger();
    let (tx, rx) = mpsc::channel::<Message>();
    let command_txs = initialize_robots(tx);
    let mut positions = initialize_positions();
    let mut map = initialize_map();

    let renderer = TerminalRenderer;

    loop {
        for command_tx in &command_txs {
            command_tx
                .send(Command::Move)
                .expect("Failed to send move command");
        }

        for _ in 0..NB_ROBOTS {
            update_and_draw_map(&rx, &mut positions, &mut map, &renderer);
        }

        thread::sleep(TICK_DURATION);
    }
}
