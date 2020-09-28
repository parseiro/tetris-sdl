use crate::tetris::Tetris;
use std::slice::Iter;

pub(crate) struct GameMap {
    map: Vec<Vec<u8>>,
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

    pub(crate) fn check_lines(&mut self, current_level: u32) -> u32 {
        let mut line = 0;
        let mut score_add = 0;

        while line < self.map.len() {
            let mut complete = true;

            for column in &self.map[line] {
                if *column == 0 {
                    complete = false;
                    break;
                }
            }

            if complete == true {
                score_add += current_level;
                self.map.remove(line);
                line -= 1;
                // increase the number of self.lines
            }
            line += 1;
        }

        if self.map.len() == 0 {
            // A tetris!
            score_add += 1000;
        }
        while self.map.len() < GAMEMAP_LINES {
            self.map.insert(0, vec![0; GAMEMAP_COLUMNS]);
        }

        score_add
    }

    pub(crate) fn iter(&self) -> Iter<'_, Vec<u8>> {
        self.map.iter()
    }
}
