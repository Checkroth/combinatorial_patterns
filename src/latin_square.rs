//! Implementation for random generation of distributed latin squares at any size.
//! 
//! Uses the Jacobson Matthews approach of +/- 1 moves in an incidence cube.
//! Implementation is a very close re-implementation of Ignacio Sagastume's work in C++.
//! 
//! Given the nature of the algorithm, should not be used for large sizes if performance matters.
//! 
//! Sources:
//! 
//! - [Generating uniformly distributed random latin squares, Mark T. Jacobson, Peter Matthews](https://onlinelibrary.wiley.com/doi/10.1002/(SICI)1520-6610(1996)4:6%3C405::AID-JCD3%3E3.0.CO;2-J)
//! - [Generation of Random Latin Squares Step by Step and Graphically, Ignacio Gallego Sagastume](http://sedici.unlp.edu.ar/bitstream/handle/10915/42155/Documento_completo.pdf?sequence=1)


use rand::{thread_rng, Rng};
use std::fmt;

type Symbol = usize;

#[derive(Debug)]
enum CubeEntry {
    On,
    Off,
    Improper
}

impl CubeEntry {
    /// Returns one step lower of the current cube entry.
    /// 
    /// Direction is On -> Off -> Improper -> !panic
    /// Panics if Improper entry is turned off, as this should only occur due to a programming error.
    pub fn toggle_off(&self) -> CubeEntry {
        match self {
            CubeEntry::On => CubeEntry::Off,
            CubeEntry::Off => CubeEntry::Improper,
            CubeEntry::Improper => panic!("ProgrammingError: Cannot turn off improper cell.")
        }
    }
    /// Returns one step higher of the current cube entry.
    /// 
    /// Direction is Improper -> Off -> On -> !panic
    /// Panics if on entry is turned on, as this should only occur due to a programming error.
    pub fn toggle_on(&self) -> CubeEntry {
        match self {
            CubeEntry::Off => CubeEntry:: On,
            CubeEntry::Improper => CubeEntry::Off,
            CubeEntry::On => panic!("ProgrammingError: cannot turn on cell that is already on.")
        }
    }

    /// Helper to get cube entry as i32 value.
    /// Mark T. Jacobson, Peter Matthews approaches usually calculate by adding/substracting.
    /// This function allows the string output of a cube in a format more generally used with latin squares.
    /// 
    /// On = 1, Off = 0, Improper = -1.
    #[allow(dead_code)]
    pub fn as_int(&self) -> i32 {
        match self {
            CubeEntry::On => 1,
            CubeEntry::Off => 0,
            CubeEntry::Improper => -1
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SearchCoord {
    X,
    Y,
    Z
}

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl Coordinate {
    /// Mutates the incidence cube's specified search coordinate by one.
    /// Helper to allow incrementing an axis without knowing in advance which coordinate we want to increment.
    pub fn increment(&mut self, coord: SearchCoord) {
        match coord {
            SearchCoord::X => self.x += 1,
            SearchCoord::Y => self.y += 1,
            SearchCoord::Z => self.z += 1
        };
    }

    /// Helper to find the value of the coordinate we are searching for.
    /// Abstraction for operating withotu knowing what coordinate we are using in advance.
    pub fn search_axis(&self, coord: SearchCoord) -> usize {
        match coord {
            SearchCoord::X => self.x,
            SearchCoord::Y => self.y,
            SearchCoord::Z => self.z
        }
    }

    /// Creaets a new coordinate with th specified coordinates. The search coordinate is initialized
    ///     as Zero, as it will be iterated over to search for a specific value later.
    /// the usize specified that matches the search coordinate will be ignored.
    pub fn init_for_search(x: usize, y: usize, z: usize, search: SearchCoord) -> Coordinate {
        match search {
            SearchCoord::X => Coordinate {x: 0, y: y, z: z},
            SearchCoord::Y => Coordinate {x: x, y: 0, z: z},
            SearchCoord::Z => Coordinate {x: x, y: y, z: 0}
        }
    }
}

/// An n x n grid of n symbols, where each symbol appears in each row and column only once.
/// 
/// Can be used to create a latin square of any size.
/// 
/// To create a random valid latin square, simply call `LatinSquare::new_random(dimensions)`
/// 
/// An example rust main that would generate and output the resulting square:
///
/// ``` 
/// fn _main() {
///    println!("making cube...");
///    let args: Vec<String> = env::args().collect();
///    let size = args[1].parse().unwrap_or(4);
///    println!("{}", LatinSquare::new_random(size));
/// }
/// ```
pub struct LatinSquare {
    size: usize,
    pub square: Vec<Vec<Symbol>>
}

impl LatinSquare {
    fn new_square(dimensions: usize, value_initializer: fn(usize, usize, usize) -> usize) -> LatinSquare {
        let rows = (0..dimensions).map(|rownum| {
            (0..dimensions).map(|colnum| {
                value_initializer(dimensions, colnum, rownum)
            }).collect::<Vec<usize>>()
        }).collect::<Vec<Vec<usize>>>();
        LatinSquare {
            size: dimensions,
            square: rows
        }
    }

    /// Creates a new latin square where each row is a 1-cell shift.
    /// e.g. if `dimensions` is 3,
    /// 
    /// 1 2 3
    /// 2 3 1
    /// 3 1 2
    /// 
    /// Generally used as the starting point for a random latin square.
    pub fn new_cyclic(dimensions: usize) -> LatinSquare {
        LatinSquare::new_square(dimensions, |dimensions, colnum, rownum| {
            ((colnum + rownum) % (dimensions - 1)) + 1
        })
    }

    /// Creates a new randomized latin square using the Mark T. Jacobson, Peter Matthews approach.
    /// 
    /// TODO:: Add functionality here to add restrictions on structure/cyclcic nature.
    pub fn new_random(dimensions: usize) -> LatinSquare {
        let mut cube = IncidenceCube::new_cyclic(dimensions);
        cube.shuffle();
        cube.as_latin_square()
    }

    /// Creates a new latin square where every cell is 0.
    /// This isn't a valid latin square.
    /// In other words, just a Vec<Vec<usize>> of size `dimensions`, pre-populated with zeros.
    pub fn new_empty(dimensions: usize) -> LatinSquare {
        LatinSquare::new_square(dimensions, |_, _, _| 0)
    }
}

impl fmt::Display for LatinSquare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows: Vec<String> = self.square.iter().map(|row| {
            row.iter().map(|symbol| symbol.to_string()).collect::<Vec<String>>().join("   ")
        }).collect();
        let square: String = rows.join("\n\n");
        write!(f, "Latin square of size {}\n\n{}", self.size, square)
    }
}

/// A three-dimensional representation of a latin square.
/// 
/// the x and y axes are the same, where the enumeration of the possible values becomes the z axis.
/// In a simple example, given the latin square:
/// 0 1
/// 1 0
/// The incidence cube would become
/// (0, 0, 0), (0, 1, 1)
/// (1, 0, 1), (1, 1, 0)
/// Where each value is (x, y, z)
/// 
/// call `IncidenceCube::new_cyclic` to create a new incidence cube with the dimensions you want.
/// call `IncidenceCube::as_latin_square` to downgrade it to two dimensions, for general use and output.
pub struct IncidenceCube {
    size: usize,
    // xpos, ypos, zpos
    cube: Vec<Vec<Vec<CubeEntry>>>,
    improper_cell: Option<Coordinate>
}

impl IncidenceCube {
    pub fn new_cyclic(dimensions: usize) -> IncidenceCube {
        // starting_square = LatinSquare::new_cyclic(dimensions);
        let coords: Vec<Vec<Vec<CubeEntry>>> = (0..dimensions).map(|rownum| {
            (0..dimensions).map(|colnum| {
                let set_symbol = (colnum + rownum) % (dimensions);
                (0..dimensions).map(|symbolnum| {
                    if symbolnum == set_symbol {
                        CubeEntry::On
                    }
                    else {
                        CubeEntry::Off
                    }
                }).collect()
            }).collect()
        }).collect();
        IncidenceCube {
           size: dimensions,
           cube: coords,
           improper_cell: None
       }
    }

    /// Transform the incidence cube in to its 2-dimensional representation.
    pub fn as_latin_square(&self) -> LatinSquare {
        let mut square = LatinSquare::new_empty(self.size);

        for (rownum, row) in self.cube.iter().enumerate() {
            for (colnum, col) in row.iter().enumerate() {
                for (symbolposition, symbol) in col.iter().enumerate() {
                    if let CubeEntry::On = symbol {
                        square.square[rownum][colnum] = symbolposition;
                    }
                }
            }
        }
        square
    }

    /// Shuffles the incidence cube at least cube.size ^ 3 times.
    /// Will continue to shuffle until the cube is proper.
    /// 
    /// Optionally, will also continue to shuffle until the cube has no cyclical cells.
    /// This option is only viable if the cube size is an even number.
    /// Checking for cyclic cells is very slow, especially for large cubes. Avoid using if performance matters.
    pub fn shuffle(&mut self) {
        for _ in 0..i32::pow(self.size as i32, 3) {
            self.move_cell();
        }
        loop {
            if self.improper_cell.is_none() {
                break
            }
            self.move_cell();
        }
    }

    /// Moves a cell in the cube to another position. May resultin an improper cube.
    /// If the cube is already improper (i.e. self.improper_cell is Some), will move that cell.
    /// Otherwise, will randomly choose an origin Off cell and a target On cell to swap.
    /// 
    /// Logical reasoning here is too complex for documentation, but can be further explored in
    /// "Generating Uniformly Distributed Latin Squares" by  Mark T. Jacobson, Peter Matthews.
    fn move_cell(&mut self) {
        let &mut zero_cell;
        let (origin, use_first_occurence) = match &self.improper_cell {
            Some(cell) => (cell, None),
            None => {
                zero_cell = self.find_off_cell();
                (&zero_cell, Some(true))
            }
        };

        let new = Coordinate {
            x: self.pick_coordinate(0, origin.y, origin.z, SearchCoord::X, use_first_occurence),
            y: self.pick_coordinate(origin.x, 0, origin.z, SearchCoord::Y, use_first_occurence),
            z: self.pick_coordinate(origin.x, origin.y, 0, SearchCoord::Z, use_first_occurence)
        };

        // Switch new coords on
        for c in [
            Coordinate { x: origin.x, y: origin.y, z: origin.z }, // x1,y1,z1 -> x1,y1,z2
            Coordinate { x: origin.x, y: new.y, z: new.z }, // x1,y2,z2 -> x1,y2,z1
            Coordinate { x: new.x, y: new.y, z: origin.z }, // x2,y2,z1 -> x2,y1,z1
            Coordinate { x: new.x, y: origin.y, z: new.z } // x2,y1,z2 -> x2,y2,z2
        ] {
            self.cube[c.x][c.y][c.z] = self.cube[c.x][c.y][c.z].toggle_on();
        }

        for c in [
            Coordinate { x: origin.x, y: origin.y, z: new.z },
            Coordinate { x: origin.x, y: new.y, z: origin.z },
            Coordinate { x: new.x, y: origin.y, z: origin.z },
            Coordinate { x: new.x, y: new.y, z: new.z }
        ] {
            self.cube[c.x][c.y][c.z] = self.cube[c.x][c.y][c.z].toggle_off();
        }

        if let CubeEntry::Improper = self.cube[new.x][new.y][new.z] {
            self.improper_cell = Some(new);
        } else {
            self.improper_cell = None;
        }
    }

    /// Returns all cyclical cube cells. That is:
    /// given coordinates (x1, y1, z1) and (x2, y2, z2), if the following four positions are "On":
    /// - x1, y1, z1
    /// - x2, y2, z1
    /// - x1+n, y1, z2
    /// - x2+n, y2, z2
    /// The cell is cylcical.
    /// 
    /// In a Latin Square representation, we might have:
    /// 
    /// 1   0   2   3
    /// 2  [3]  0  [1]
    ///[3]  2  [1]  0
    /// 0   1   3   2
    /// 
    /// Corresponding to the coordinates above:
    /// x1=0, y1=2, x2=1, y2=1, z1=3, z2=1, n=2
    /// - (0, 2, 3)
    /// - (1, 1, 3)
    /// - (2, 2, 1)
    /// - (3, 1, 1)
    /// Would be cyclical cells.

    #[allow(dead_code)]
    fn find_cyclic_cell(&self) -> Option<Vec<Coordinate>> {
        let mut cyclic_cells: Vec<Coordinate> = Vec::new();
        for (rownum, row) in self.cube.iter().enumerate() {
            for (colnum, col) in row.iter().enumerate() {
                for (symbolposition, symbol) in col.iter().enumerate() {
                    if let CubeEntry::On = symbol {
                        cyclic_cells.push(Coordinate {x: rownum, y: colnum, z: symbolposition});
                    }
                }
            }
        }
        Some(cyclic_cells)
    }

    /// Returns the cube position of a random cell marked as "Off".
    ///
    /// Assumes that the cube has a reasonable number of Off cells, which it by definition does
    /// if it represents a valid latin square, or a mid-shuffle improper square.
    ///
    /// Danger: Will loop infinitely if there are no zero cells, and may be very slow if the cube is not
    ///     representative of an actual latin square.
    fn find_off_cell(&self) -> Coordinate {
        let mut x: usize;
        let mut y: usize;
        let mut z: usize;
        loop {
            x = thread_rng().gen_range(0..self.size);
            y = thread_rng().gen_range(0..self.size);
            z = thread_rng().gen_range(0..self.size);
            if let CubeEntry::Off = self.cube[x][y][z] {
                break;
            }
        }
        Coordinate {
            x: x,
            y: y,
            z: z
        }
    }

    /// Finds an "On" cell along the axis specified by the search position and the search coordinate.
    /// 
    /// Intended as an internal helper to reduce code duplication. To achieve the same functionality
    /// as calling this function directly, call pick_coordinate with take_first = Some(true).
    /// 
    /// # Arguments
    /// * `search_pos` - A coordinate that will search along two of x, y, and z. The position of the third
    ///     coordinate will be mutably incremented, so the value within the search_pos will be the same as the
    ///     return value.
    /// * `search_coord` - Whhich axis to increment. The value of this enum indicates which axis we
    ///     are looking for, leaving the other two as originally passed.
    fn find_on_cell_along_axis(&self, search_pos: &mut Coordinate, search_coord: SearchCoord) -> Option<usize> {
        loop {
            let cell = &self.cube[search_pos.x][search_pos.y][search_pos.z];
            if let CubeEntry::On = cell {
                break
            } else if search_pos.search_axis(search_coord) == self.size {
                return None
            } else {
                search_pos.increment(search_coord);
            }
        }
        Some(search_pos.search_axis(search_coord))
    }

    
    /// Finds an On coordinate along the specified axis. Almost identical to find_cell_along_axis.
    /// 
    /// # Arguments
    /// - `x` - The x position on which to start your search.
    /// - `y` - The y position on which to start your search.
    /// - `z` - The z position on which to start your search.
    /// - `search_coord` - The axis on which you are looking for an On value.
    /// - `take_first` - Allows for some degree of randomness. 
    ///     If Some, will take the first if true or the second if false.
    ///     If None, will take the first or second with a 50/50 probability.
    pub fn pick_coordinate(
        &self, 
        x: usize,
        y: usize,
        z: usize,
        search_coord: SearchCoord,
        take_first: Option<bool>,
    ) -> usize {
        let mut search_pos = Coordinate::init_for_search(x, y, z, search_coord);

        let take_first = take_first.unwrap_or_else(|| {
            thread_rng().gen_bool(0.5)
        });

        let first_result = &self.find_on_cell_along_axis(&mut search_pos, search_coord);
        match (first_result, take_first) {
            (Some(res), true) => *res,
            (_, false) => {
                search_pos.increment(search_coord); // Prevent finding the same coordinate we just found.
                *&self.find_on_cell_along_axis(&mut search_pos, search_coord).unwrap()
            },
            _ => panic!("Couldn't find 'On' point along cube axis x: {}, y: {}, z: {}", x, y, z)
        }
   }
}