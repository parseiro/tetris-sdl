// use crate::tetris::Tetris;
use std::slice::Iter;

pub(crate) struct GameMap {
    pub(crate) map: Vec<Vec<u8>>,
}


pub(crate) const GAMEMAP_LINES: usize = 16;
pub(crate) const GAMEMAP_COLUMNS: usize = 10;

impl GameMap {
    #[inline]
    pub(crate) fn get_game_map_point(&self, line: usize, column: usize) -> u8 {
        self.map[line][column]
    }

    #[inline]
    pub(crate) fn set_gamemap_point(&mut self, line: usize, column: usize, new_value: u8) {
        self.map[line][column] = new_value;
    }

    pub(crate) fn new() -> GameMap {
        let mut game_map = Vec::<Vec<u8>>::new();

        for _ in 0..GAMEMAP_LINES {
            game_map.push(vec![0; GAMEMAP_COLUMNS]);
        }

        GameMap { map: game_map }
    }

    pub(crate) fn lines(&self) -> usize {
        self.map.len()
    }



    pub(crate) fn iter(&self) -> Iter<'_, Vec<u8>> {
        self.map.iter()
    }


}
