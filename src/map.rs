use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

/** To allow
 * Clone - mytile.clone() to make a deep clope of the tile
 * Copy - mytile = other_tile to make a shallow copy of the tile instead of moving the value with the borrow checker
 * PartialEq - mytile == other_tile to compare two tiles
 */
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_index(x, y);
                match self.tiles[idx] {
                    TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                    TileType::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437('#')),
                }
            }
        }
    }
}

pub fn map_index(x: i32, y: i32) -> usize {
    (y * SCREEN_WIDTH + x) as usize
}
