use crate::{
    map::{CellType, Map2D},
    utils,
};

pub trait Renderer {
    fn draw_map(&self, map: &Map2D);
    fn clean(&self);
}
pub struct TerminalRenderer;

impl TerminalRenderer {
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
}

impl Renderer for TerminalRenderer {
    fn draw_map(&self, map: &Map2D) {
        for row in map {
            for &c in row {
                Self::print_cell(c);
            }
            println!();
        }
    }

    fn clean(&self) {
        utils::clean_terminal();
    }
}
