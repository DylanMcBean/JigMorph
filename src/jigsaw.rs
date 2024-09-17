use rand::prelude::*;
use crate::settings::Settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Side(pub i8); // i8: 0 = edge, positive = "inny," negative = "outy"

impl Side {
    fn matches(&self, other: &Side) -> bool {
        self.0 + other.0 == 0
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub id: usize,
    pub sides: [Side; 4], // [top, right, bottom, left]
}


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PieceState {
    pub piece: Piece,
    rotation: Rotation,
}

impl AsRef<Piece> for PieceState {
    fn as_ref(&self) -> &Piece {
        &self.piece
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rotation {
    Degrees0,
    Degrees90,
    Degrees180,
    Degrees270,
}

#[derive(Debug, Clone)]
pub struct Jigsaw {
    grid: Vec<Vec<Option<PieceState>>>,
    pieces: Vec<PieceState>,
}

impl Jigsaw {
    pub fn generate(size: usize, settings: &Settings) -> Jigsaw {
        Self::generate_with_dimensions(size, size, settings)
    }

    fn generate_with_dimensions(width: usize, height: usize, settings: &Settings) -> Jigsaw {
        // Initialize the grid with None
        let mut grid: Vec<Vec<Option<PieceState>>> = vec![vec![None; width]; height];
        let mut pieces: Vec<PieceState> = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let sides = [
                    Side(if y == 0 { 0 } else { -grid[y - 1][x].as_ref().unwrap().piece.sides[2].0 }),        // Top
                    Side(if x == width - 1 { 0 } else { Self::pick_side(settings.side_types) }),  // Right
                    Side(if y == height - 1 { 0 } else { Self::pick_side(settings.side_types) }), // Bottom
                    Side(if x == 0 { 0 } else { -grid[y][x - 1].as_ref().unwrap().piece.sides[1].0 }),       // Left
                ];

                let id = y * width + x;
                let piece_state = PieceState {
                    piece: Piece { id, sides },
                    rotation: Rotation::Degrees0,
                };
                grid[y][x] = Some(piece_state.clone());
                if x != 0 || y != 0 {
                    pieces.push(piece_state);
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                if x != 0 || y != 0 {
                    grid[y][x] = None;
                }
            }
        }

        Jigsaw { grid, pieces }
    }

    fn pick_side(side_count: u8) -> i8 {
        let mut rng = thread_rng();
        let side = rng.gen_range(1..=side_count) as i8;
        let sign = if rng.gen_bool(0.5) { 1 } else { -1 };
        sign * side
    }

    fn find_next_free_space(&self) -> Option<(usize, usize)> {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if self.grid[y][x].is_none() {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn try_solve(&mut self, solutions: &mut Vec<Vec<Vec<Option<PieceState>>>>) {
        if let Some((x, y)) = self.find_next_free_space() {
            let pieces_left = self.pieces.clone();
            for piece in pieces_left {
                for &rotation in &[
                    Rotation::Degrees0,
                    Rotation::Degrees90,
                    Rotation::Degrees180,
                    Rotation::Degrees270,
                ] {
                    let rotated_sides = Self::rotate_sides(&piece.piece.sides, rotation);
                    let rotated_piece = PieceState {
                        piece: Piece { id: piece.piece.id, sides: rotated_sides },
                        rotation,
                    };

                    if self.can_place_piece_at(x, y, &rotated_piece) {
                        self.grid[y][x] = Some(rotated_piece.clone());
                        self.pieces.retain(|p| p.piece.id != rotated_piece.piece.id);
                        if self.is_solved() {
                            solutions.push(self.grid.clone());
                        } else {
                            self.try_solve(solutions);
                        }
                        // Backtrack
                        self.grid[y][x] = None;
                        self.pieces.push(piece.clone());
                    }
                }
            }
        }
    }

    fn rotate_sides(sides: &[Side; 4], rotation: Rotation) -> [Side; 4] {
        match rotation {
            Rotation::Degrees0 => *sides,
            Rotation::Degrees90 => [sides[3], sides[0], sides[1], sides[2]],
            Rotation::Degrees180 => [sides[2], sides[3], sides[0], sides[1]],
            Rotation::Degrees270 => [sides[1], sides[2], sides[3], sides[0]],
        }
    }

    fn can_place_piece_at(&self, x: usize, y: usize, piece: &PieceState) -> bool {
        // Check left side
        if x == 0 {
            if !piece.piece.sides[3].matches(&Side(0)) {
                return false;
            }
        } else if let Some(left_piece) = &self.grid[y][x - 1] {
            if !piece.piece.sides[3].matches(&left_piece.piece.sides[1]) {
                return false;
            }
        }

        // Check right side
        if x == self.grid[0].len() - 1 {
            if !piece.piece.sides[1].matches(&Side(0)) {
                return false;
            }
        }

        // Check top side
        if y == 0 {
            if !piece.piece.sides[0].matches(&Side(0)) {
                return false;
            }
        } else if let Some(top_piece) = &self.grid[y - 1][x] {
            if !piece.piece.sides[0].matches(&top_piece.piece.sides[2]) {
                return false;
            }
        }

        // Check bottom side
        if y == self.grid.len() - 1 {
            if !piece.piece.sides[2].matches(&Side(0)) {
                return false;
            }
        }

        true
    }

    fn is_solved(&self) -> bool {
        self.find_next_free_space().is_none()
    }
}
