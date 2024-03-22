use std::{collections::HashMap, fmt::Display, sync::mpsc::Receiver};

use crate::{renderer::Renderer, Message, NB_ROBOTS};

#[derive(Debug, Clone, Copy)]
pub enum CellType {
    Blank,
    Robot(u32),
}

impl Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            CellType::Blank => ' ',
            CellType::Robot(id) => match id {
                0 => '@',
                1 => '%',
                2 => '#',
                3 => '*',
                4 => '+',
                _ => unimplemented!(),
            },
        };
        write!(f, "{}", c)
    }
}

pub type Map2D = Vec<Vec<CellType>>;
pub type Position = (i32, i32);

pub const INITIAL_POSITION: Position = (10, 10);
pub const MAX_HEIGHT: i32 = 20;
pub const MAX_WEIGHT: i32 = 20;
pub const MIN_HEIGHT: i32 = 0;
pub const MIN_WEIGHT: i32 = 0;

pub fn initialize_map() -> Map2D {
    vec![vec![CellType::Blank; MAX_WEIGHT as usize]; MAX_HEIGHT as usize]
}

pub fn clean_map(map: &mut Map2D) {
    map.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|c| *c = CellType::Blank));
}

pub fn initialize_positions() -> HashMap<u32, (i32, i32)> {
    let mut positions = HashMap::new();
    for id in 0..NB_ROBOTS {
        positions.insert(id, INITIAL_POSITION);
    }
    positions
}

pub fn update_and_draw_map(
    rx: &Receiver<Message>,
    positions: &mut HashMap<u32, Position>,
    map: &mut Map2D,
    renderer: &dyn Renderer,
) {
    if let Ok(Message::NewPosition { id, dx, dy }) = rx.recv() {
        update_positions_map(positions, map, id, dx, dy);
        renderer.clean();
        renderer.draw_map(map);
    }
}

pub fn update_positions_map(
    positions: &mut HashMap<u32, Position>,
    map: &mut Map2D,
    id: u32,
    dx: i32,
    dy: i32,
) {
    if let Some(position) = positions.get_mut(&id) {
        position.0 = (position.0 + dx).clamp(MIN_WEIGHT, MAX_WEIGHT - 1);
        position.1 = (position.1 + dy).clamp(MIN_HEIGHT, MAX_HEIGHT - 1);
    }

    clean_map(map);
    for (&id, &(x, y)) in positions.iter() {
        map[y as usize][x as usize] = match id {
            0..=4 => CellType::Robot(id),
            _ => unimplemented!(),
        };
    }
}
