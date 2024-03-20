use std::collections::HashMap;

use crate::Robot;

#[derive(Debug, Clone, Copy)]
pub enum CellType {
    Blank,
    Robot(u32),
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

pub fn initialize_positions(robots: &Vec<Robot>) -> HashMap<u32, Robot> {
    let mut positions = HashMap::new();
    for robot in robots {
        let guard = robot.lock().expect("No concurrence for now");
        let id = guard.id;
        positions.insert(id, robot.clone());
    }
    positions
}

pub fn update_positions_map(
    positions: &mut HashMap<u32, Robot>,
    map: &mut Map2D,
    id: u32,
    dx: i32,
    dy: i32,
) {
    update_position(positions, id, dx, dy);
    update_map(map, positions);
}

pub fn update_position(positions: &mut HashMap<u32, Robot>, id: u32, dx: i32, dy: i32) {
    if let Some(robot) = positions.get_mut(&id) {
        let mut inner = robot.lock().expect("no concurrence here");
        let (x, y) = inner.coords;
        inner.coords.0 = (x + dx).clamp(MIN_WEIGHT, MAX_WEIGHT - 1);
        inner.coords.1 = (y + dy).clamp(MIN_HEIGHT, MAX_HEIGHT - 1);
    }
}

fn update_map(map: &mut Map2D, positions: &mut HashMap<u32, Robot>) {
    clean_map(map);
    for (&id, robot) in positions.iter() {
        let inner = robot.lock().expect("no concurrence here");
        let (x, y) = inner.coords;
        map[y as usize][x as usize] = match id {
            0..=4 => CellType::Robot(id),
            _ => unimplemented!(),
        };
    }
}
