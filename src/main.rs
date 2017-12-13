#[derive(PartialEq, Debug)]
enum TerrainGround {
    Soil,
    Stone
} 

#[derive(PartialEq, Debug)]
enum TerrainBlock {
    Tree,
    Soil,
    Stone
}

#[derive(PartialEq, Debug)]
enum Being {
    Orc,
    Human
}

struct Square {
    ground: TerrainGround,
    block: Option<TerrainBlock>,
    being: Option<Being> 
}

struct Grid {
    size: (usize,usize),
    squares: Vec<Square>
}

enum Direction {
    West,
    East,
    North,
    South
}

#[derive(PartialEq,Debug)]
enum MovementError {
    NoBeingInSquare,
    SquareOffGrid,
    SquareOccupied,
    StoneTerrain
}

impl Grid {
    fn move_being_in_coord(&self, coord: (usize,usize), dir: Direction) -> Result<(usize,usize), MovementError> {
        let square = self.squares.get(coord.0 * self.size.0 + coord.1)
            .expect("Index out of bounds trying to get being");

        let destcoords = match dir {
            Direction::West => (coord.0 - 1, coord.1),
            Direction::East => (coord.0 + 1, coord.1),
            Direction::North => (coord.0, coord.1 - 1),
            Direction::South => (coord.0, coord.1 + 1)
        };
        
        match square.being {
            Some(_) => match self.squares.get(destcoords.0 * self.size.0 + destcoords.1) {
                Some(s) if s.ground == TerrainGround::Stone => Err(MovementError::StoneTerrain),
                Some(s) if s.being != None => Err(MovementError::SquareOccupied),
                Some(_) => Ok(destcoords),
                None => Err(MovementError::SquareOffGrid)
            },
            None => Err(MovementError::NoBeingInSquare)
        }
    }

    fn generate_empty(size_x: usize, size_y: usize) -> Grid {
        let number_of_squares = size_x * size_y;
        let mut squares: Vec<Square> = Vec::with_capacity(number_of_squares);

        for _ in 0..number_of_squares {
            squares.push( Square {
                ground: TerrainGround::Soil,  
                block: None, 
                being: None });
        }

        Grid {
            size: (size_x,size_y),
            squares: squares
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_grid() {
        let grid = ::Grid::generate_empty(5,13);
        assert_eq!(grid.size, (5,13));
        let mut number_of_squares = 0;

        for square in &grid.squares {
            assert_eq!(square.ground, ::TerrainGround::Soil);
            assert_eq!(square.block, None);
            assert_eq!(square.being, None);
            number_of_squares += 1;
        }

        assert_eq!(grid.squares.len(), 5*13);
        assert_eq!(number_of_squares, 5*13);
    }

    #[test]
    fn test_move_being_not_in_square() {
        let grid = ::Grid::generate_empty(3, 3);
        assert_eq!(grid.move_being_in_coord((0, 0), ::Direction::West), Err(::MovementError::NoBeingInSquare));
    }   

    #[test]
    fn test_move_being_to_stone_square() {
        let grid = ::Grid::generate_empty(3, 3);
        
        let playerSquare = ::Square {
            ground: ::TerrainGround::Soil,  
            block: Some(::TerrainBlock::Soil),
            being: Some(::Being::Human)
        };
        let stoneSqure = ::Square { 
            ground: ::TerrainGround::Stone,  
            block: None,
            being: None
        };

        grid.squares[0] = stoneSqure;
        grid.squares[1] = playerSquare;

        assert_eq!(grid.move_being_in_coord((0, 1), ::Direction::East), Err(::MovementError::StoneTerrain));
    }   
}

fn main() {
    println!("Hello, world!");
}
