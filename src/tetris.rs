// use core::option::Option::{None, Some};


use crate::tetrimino::{Tetrimino, PIECE_STATES_LINES, PIECE_STATES_COLUMNS};
use crate::game_map::{GameMap, GAMEMAP_COLUMNS, GAMEMAP_LINES};

pub(crate) const LEVEL_TIMES: [u32; 10] = [1000, 850, 700, 600, 500, 400, 300, 250, 221, 190];
pub(crate) const LEVEL_LINES: [u32; 10] = [20,   40,  60,  80,  100, 120, 140, 160, 180, 200];

pub struct Tetris {
    pub(crate) game_map: GameMap,
    current_level: u32,
    score: u32,
    nb_lines: u32,
    current_piece: Option<Tetrimino>,
}

impl Tetris {
    pub(crate) fn new() -> Tetris {


        Tetris {
            game_map: GameMap::new(),
            current_level: 1,
            score: 0,
            nb_lines: 0,
            current_piece: None,
        }
    }


    pub(crate) fn get_current_piece(&self) -> Option<&Tetrimino> {
        self.current_piece.as_ref()
    }



    pub(crate) fn get_current_level(&self) -> u32 {
        self.current_level
    }

    pub(crate) fn get_score(&self) -> u32 {
        self.score
    }




    pub(crate) fn make_permanent(&mut self) {
        let mut to_add = 0;
        if let Some(ref mut piece) = self.current_piece {
            let mut shift_y = 0;

            while shift_y < PIECE_STATES_LINES
                && piece.get_y() + shift_y < GAMEMAP_LINES {
                let mut shift_x = 0;

                while shift_x < PIECE_STATES_LINES
                    && piece.get_x() + (shift_x as isize) < GAMEMAP_COLUMNS as isize {
                    if piece.get_state_now(shift_y, shift_x) != 0 {
                        // if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {
                        self.game_map.set_gamemap_point(
                            piece.get_y() + shift_y,
                            piece.get_x() as usize + shift_x,
                            piece.get_state_now(shift_y, shift_x));
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
            to_add += self.get_current_level();
        }

        self.add_to_score(to_add);

        let score = self.game_map.check_lines(self.get_current_level());

        self.add_to_score(score);

        self.current_piece = None;
    }


    pub(crate) fn change_current_piece_position(&mut self, new_column: isize, new_line: usize) -> bool {
        if let Some(ref mut piece) = self.current_piece {
            let can_be_placed = piece.test_position(&self.game_map,
                                                    Some(piece.get_current_state() as usize),
                                                    new_column,
                                                    new_line);
            if can_be_placed == true {
                piece.set_column(new_column as isize);
                piece.set_line(new_line);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub(crate) fn rotate_current_piece(&mut self /*&mut piece: Tetrimino*/) {
        let piece = self.current_piece.as_mut().unwrap();

        piece
            .rotate(&mut self.game_map);
    }

    pub(crate) fn set_current_piece(&mut self, piece: Tetrimino) {
        self.current_piece = Some(piece);
    }

    pub(crate) fn add_to_score(&mut self, to_add: u32) {
        self.score += to_add;
    }

    fn increase_line(&mut self) {
        self.nb_lines += 1;
        if self.nb_lines > LEVEL_LINES[self.current_level as usize - 1] {
            self.current_level += 1;
        }
    }


}