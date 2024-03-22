use crate::{map::Map2D, utils};

pub trait Renderer {
    fn draw_map(&self, map: &Map2D);
    fn clean(&self);
}
pub struct TerminalRenderer;

impl Renderer for TerminalRenderer {
    fn draw_map(&self, map: &Map2D) {
        for row in map {
            for &c in row {
                print!("{}", c);
            }
            println!();
        }
    }

    fn clean(&self) {
        utils::clean_terminal();
    }
}
