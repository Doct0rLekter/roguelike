#![warn(clippy::pedantic)]

use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

#[allow(clippy::module_name_repetitions, clippy::cast_sign_loss)] // Bounds checked
pub fn map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
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
                let index = map_index(x, y);
                match self.tiles.get(index) {
                    Some(TileType::Floor) => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    Some(TileType::Wall) => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                    None => panic!("Invalid index"),
                }
            }
        }
    }

    pub fn in_bounds(point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        Self::in_bounds(point) && self.tiles[map_index(point.x, point.y)] == TileType::Floor
    }

    #[allow(clippy::unused_self)]
    pub fn try_index(&self, point: Point) -> Option<usize> {
        if Self::in_bounds(point) {
            Some(map_index(point.x, point.y))
        } else {
            None
        }
    }
}
