#![allow(dead_code, unused)]

use std::{fmt::Formatter, io::Write, thread, time::Duration};

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell {
    state: State,
}

impl Cell {
    fn new(state: State) -> Self {
        Cell { state }
    }

    fn next_state(&self, neighbors: Vec<Cell>) -> Cell {
        let alive_neighbors: Vec<Cell> = neighbors
            .iter()
            .filter(|x| x.state == State::Alive)
            .map(|x| *x)
            .collect();
        let dead_neighbors: Vec<Cell> = neighbors
            .iter()
            .filter(|x| x.state == State::Dead)
            .map(|x| *x)
            .collect();
        assert!(alive_neighbors.len() + dead_neighbors.len() == neighbors.len());
        match self.state {
            State::Alive => {
                if alive_neighbors.len() < 2 || alive_neighbors.len() > 3 {
                    Cell { state: State::Dead }
                } else {
                    *self
                }
            }
            State::Dead => {
                if alive_neighbors.len() == 3 {
                    Cell {
                        state: State::Alive,
                    }
                } else {
                    *self
                }
            }
        }
    }
}

fn get_neighbors(coords: Coords, game: &GameOfLife) -> Vec<Cell> {
    let mut cells: Vec<Cell> = Vec::new();
    let x = coords.x;
    let y = coords.y;
    let x_not_low = coords.x != 0;
    let y_not_low = coords.y != 0;
    let x_not_high = coords.x != game.board[0].len() - 1;
    let y_not_high = coords.y != game.board.len() - 1;
    if x_not_low {
        if y_not_low {
            cells.push(game.get_cell_at(Coords { x: x - 1, y: y - 1 }));
        }
        if y_not_high {
            cells.push(game.get_cell_at(Coords { x: x - 1, y: y + 1 }));
        }
        cells.push(game.get_cell_at(Coords { x: x - 1, y }));
    }
    if y_not_low {
        if x_not_high {
            cells.push(game.get_cell_at(Coords { x: x + 1, y: y - 1 }));
        }
        cells.push(game.get_cell_at(Coords { x, y: y - 1 }));
    }
    if x_not_high {
        if y_not_high {
            cells.push(game.get_cell_at(Coords { x: x + 1, y: y + 1 }));
        }
        cells.push(game.get_cell_at(Coords { x: x + 1, y }));
    }
    if y_not_high {
        cells.push(game.get_cell_at(Coords { x, y: y + 1 }));
    }
    cells
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct GameOfLife {
    board: [[Cell; 50]; 25],
}

impl GameOfLife {
    fn new(live_cells: Vec<Coords>) -> Self {
        let mut new_frame: [[Cell; 50]; 25] = [[Cell::new(State::Dead); 50]; 25];
        for cell in live_cells {
            new_frame[cell.y][cell.x] = Cell {
                state: State::Alive,
            };
        }

        GameOfLife { board: new_frame }
    }

    fn next_frame(&self) -> GameOfLife {
        let mut new_frame: [[Cell; 50]; 25] = [[Cell::new(State::Dead); 50]; 25];
        let mut y = 0;
        while y < 25 {
            let mut x = 0;
            while x < 50 {
                let prev_cell = self.board[y][x];
                new_frame[y][x] = prev_cell.next_state(get_neighbors(Coords { x, y }, self));
                x += 1;
            }
            y += 1;
        }

        GameOfLife { board: new_frame }
    }

    fn get_cell_at(&self, coords: Coords) -> Cell {
        self.board[coords.y][coords.x]
    }
}

impl std::fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut y = 0;
        while y < self.board.len() {
            let mut x = 0;
            writeln!(f, "");
            while x < self.board[0].len() {
                write!(
                    f,
                    "{}",
                    match self.board[y][x].state {
                        State::Alive => 'o',
                        State::Dead => '.',
                    }
                );
                x += 1;
            }
            y += 1;
        }
        Ok(())
    }
}

fn main() {
    let millis = 250;
    let max_cycles = 100;
    let mut cycle = 0;

    let start_frame = GameOfLife::new(vec![
        // Blinker
        Coords { x: 0, y: 1 },
        Coords { x: 1, y: 1 },
        Coords { x: 2, y: 1 },
        //Glider
        Coords { x: 6, y: 0 },
        Coords { x: 6, y: 1 },
        Coords { x: 6, y: 2 },
        Coords { x: 5, y: 2 },
        Coords { x: 4, y: 1 },
        //Pulsar
        Coords { x: 5, y: 13 },
        Coords { x: 5, y: 14 },
        Coords { x: 5, y: 15 },
        Coords { x: 5, y: 19 },
        Coords { x: 5, y: 20 },
        Coords { x: 5, y: 21 },
        Coords { x: 10, y: 13 },
        Coords { x: 10, y: 14 },
        Coords { x: 10, y: 15 },
        Coords { x: 10, y: 19 },
        Coords { x: 10, y: 20 },
        Coords { x: 10, y: 21 },
        Coords { x: 12, y: 13 },
        Coords { x: 12, y: 14 },
        Coords { x: 12, y: 15 },
        Coords { x: 12, y: 19 },
        Coords { x: 12, y: 20 },
        Coords { x: 12, y: 21 },
        Coords { x: 17, y: 13 },
        Coords { x: 17, y: 14 },
        Coords { x: 17, y: 15 },
        Coords { x: 17, y: 19 },
        Coords { x: 17, y: 20 },
        Coords { x: 17, y: 21 },
        Coords { x: 7, y: 11 },
        Coords { x: 8, y: 11 },
        Coords { x: 9, y: 11 },
        Coords { x: 13, y: 11 },
        Coords { x: 14, y: 11 },
        Coords { x: 15, y: 11 },
        Coords { x: 7, y: 16 },
        Coords { x: 8, y: 16 },
        Coords { x: 9, y: 16 },
        Coords { x: 13, y: 16 },
        Coords { x: 14, y: 16 },
        Coords { x: 15, y: 16 },
        Coords { x: 7, y: 18 },
        Coords { x: 8, y: 18 },
        Coords { x: 9, y: 18 },
        Coords { x: 13, y: 18 },
        Coords { x: 14, y: 18 },
        Coords { x: 15, y: 18 },
        Coords { x: 7, y: 23 },
        Coords { x: 8, y: 23 },
        Coords { x: 9, y: 23 },
        Coords { x: 13, y: 23 },
        Coords { x: 14, y: 23 },
        Coords { x: 15, y: 23 },
    ]);
    let mut curr_frame = start_frame;
    let duration = Duration::from_millis(millis);
    println!("Starting Conway's game of life!");
    println!("Here is our starting frame: ");
    println!("{}", curr_frame);
    thread::sleep(duration);

    while cycle < max_cycles {
        cycle += 1;
        curr_frame = curr_frame.next_frame();
        println!("{} cycles:", cycle);
        println!("{}", curr_frame);
        thread::sleep(duration);
    }
}

#[test]
fn test_alternator() {
    let gol = GameOfLife::new(vec![
        Coords { x: 0, y: 1 },
        Coords { x: 1, y: 1 },
        Coords { x: 2, y: 1 },
    ]);
    let second_frame = GameOfLife::new(vec![
        Coords { x: 1, y: 0 },
        Coords { x: 1, y: 1 },
        Coords { x: 1, y: 2 },
    ]);

    assert_eq!(gol.next_frame(), second_frame)
}
