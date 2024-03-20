use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use tracing::{info, trace, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug)]
enum Command {
    Move,
}

#[derive(Debug)]
enum Message {
    NewPosition { id: u32, dx: i32, dy: i32 },
}

fn clean_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn configure_logger() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("./logs", "prefix.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(non_blocking)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Impossible de configurer le subscriber global de tracing");
    guard
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

fn draw_map(map: &Vec<Vec<char>>) {
    for row in map {
        for &c in row {
            print!("{}", c);
        }
        println!("");
    }
}

const NB_ROBOTS: u32 = 5;
const INITAL_POSITION: (i32, i32) = (10, 10);
const MAX_HEIGHT: usize = 20;
const MAX_WEIGTH: usize = 20;
const TICK_DURATION: Duration = Duration::from_millis(10);

fn main() {
    let _guard = configure_logger();
    info!("DEBuG");
    let (tx, rx) = mpsc::channel::<Message>();
    let mut command_txs = vec![];

    for id in 0..NB_ROBOTS {
        let (command_tx, command_rx) = mpsc::channel::<Command>();
        command_txs.push(command_tx);
        let tx = tx.clone();
        thread::spawn(move || {
            update_position(id, command_rx, tx);
        });
    }

    let mut positions: HashMap<u32, (i32, i32)> = HashMap::new();
    for id in 0..NB_ROBOTS {
        positions.insert(id, INITAL_POSITION);
    }

    let mut map: Vec<Vec<char>> = vec![vec![' '; MAX_WEIGTH]; MAX_HEIGHT];
    loop {
        for command_tx in &command_txs {
            command_tx
                .send(Command::Move)
                .expect("Failed to send move command");
        }

        for _ in 0..NB_ROBOTS {
            if let Ok(Message::NewPosition { id, dx, dy }) = rx.recv() {
                if let Some(position) = positions.get_mut(&id) {
                    position.0 = (position.0 + dx).clamp(0, MAX_WEIGTH as i32 - 1);
                    position.1 = (position.1 + dy).clamp(0, MAX_HEIGHT as i32 - 1);
                }
                map.iter_mut()
                    .for_each(|row| row.iter_mut().for_each(|c| *c = ' '));

                for (&id, &(x, y)) in &positions {
                    map[y as usize][x as usize] = match id {
                        0 => '@',
                        1 => '%',
                        2 => '#',
                        3 => '*',
                        4 => '+',
                        _ => unimplemented!(),
                    }
                }
            }
        }

        clean_terminal();
        draw_map(&map);
        thread::sleep(TICK_DURATION);
    }
}
