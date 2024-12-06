// Strategy
// Make a matrix by parsing input into vector of vectors
// search each vector for first char X
// search adjacent for next char M
// continue searching in direction of found M
// Increase counter if XMAS is found!

struct IndexedChar {
    index: usize,
    character: char,
}

pub struct IndexedVec {
    index: usize,
    vector: Vec<IndexedChar>,
}

pub fn enter_the_matrix<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<IndexedVec> {
    lines
        .map(|x| {
            x.chars()
                .enumerate()
                .map(|(i, c)| IndexedChar {
                    index: i,
                    character: c,
                })
                .collect()
        })
        .enumerate()
        .map(|(i, v)| IndexedVec {
            index: i,
            vector: v,
        })
        .collect()
}

pub fn find_xmas(matrix: Vec<IndexedVec>) -> u32 {
    //    0123456789
    // [0[....XXMAS.]
    //  1[.SAMXMS...]
    //  2[...S..A...]
    //  3[..A.A.MS.X]
    //  4[XMASAMX.MM]
    //  5[X.....XA.A]
    //  6[S.S.S.S.SS]
    //  7[.A.A.A.A.A]
    //  8[..M.M.M.MM]
    //  9[.X.X.XMASX]]
    matrix.iter().fold(0, |mut acc, indexed_line| {
        let line = indexed_line.index;
        let vec = &indexed_line.vector;
        for item in vec {
            let position = item.index;
            let letter = item.character;
            if letter == 'X' {
                // println!("Found X at ({},{})", line, position);
                let coords = search_adjacent([line, position], 'M', &matrix);
                // println!("Found M at {:?}", coords);
                if coords.len() > 0 {
                    for coord in coords {
                        let [x, y] = [
                            coord[0] as isize - line as isize,
                            coord[1] as isize - position as isize,
                        ];
                        // println!("Checking in direction:({}, {})", x, y);
                        let hor = coord[0] as isize + x;
                        let vert = coord[1] as isize + y;
                        if !hor.is_negative() && !vert.is_negative() {
                            if matrix[hor as usize].vector[vert as usize].character == 'A' {
                                // println! {"Found A at ({}, {})", hor, vert};
                                let next_x = coord[0] as isize + 2 * x;
                                let next_y = coord[1] as isize + 2 * y;
                                if !next_x.is_negative() && !next_y.is_negative() {
                                    if matrix[next_x as usize].vector[next_y as usize].character
                                        == 'S'
                                    {
                                        // println! {"Found S at ({}, {})", next_x, next_y};
                                        acc += 1;
                                        // println! {"Current count is {acc}"};
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        acc
    })
}

fn search_adjacent(
    coord: [usize; 2],
    searchee: char,
    search_area: &Vec<IndexedVec>,
) -> Vec<[usize; 2]> {
    // An item is adjacent if it is one away, but it cannot cross boundaries
    let furthest_column = search_area[0].vector.len();
    let deepest_row = search_area.len();
    let [row, col] = coord;
    const NEIGHBORS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    NEIGHBORS.iter().fold(Vec::new(), |mut acc, (x, y)| {
        let loc = [x + row as isize, y + col as isize];
        if !loc[0].is_negative()
            && (loc[0] + 2 < furthest_column as isize || *x <= 0)
            && !loc[1].is_negative()
            && (loc[1] + 2 < deepest_row as isize || *y <= 0)
        {
            // println! {"Checking neighbor ({}, {})", loc[0], loc[1]};
            if search_area[loc[0] as usize].vector[loc[1] as usize].character == searchee {
                acc.push([loc[0] as usize, loc[1] as usize]);
            }
        }
        acc
    })
}

pub fn find_cross_mas(matrix: Vec<IndexedVec>) -> u32 {
    //    0123456789
    // [0[.M.S......]
    //  1[..A..MSMS.]
    //  2[.M.S.MAA..]
    //  3[..A.ASMSM.]
    //  4[.M.S.M....]
    //  5[..........]
    //  6[S.S.S.S.S.]
    //  7[.A.A.A.A..]
    //  8[M.M.M.M.M.]
    //  9[..........]]

    matrix.iter().fold(0, |mut acc, indexed_line| {
        let line = indexed_line.index;
        let vec = &indexed_line.vector;
        for item in vec {
            let position = item.index;
            let letter = item.character;
            if letter == 'A' {
                // println!("Found A at ({},{})", line, position);
                let coords = search_corners([line, position], 'M', &matrix);
                // println!("Found corner M at {:?}", coords);
                if coords.len() == 2 {
                    // println!("Checking other corners for S");
                    if coords.iter().all(|coord| {
                        let upper_bound = matrix.len() as isize;
                        let [delta_x, delta_y] = [
                            line as isize - coord[0] as isize,
                            position as isize - coord[1] as isize,
                        ];

                        let next_x = coord[0] as isize + 2 * delta_x;
                        let next_y = coord[1] as isize + 2 * delta_y;

                        // println!("S could be at ({}, {})", next_x, next_y);

                        !next_x.is_negative() && !next_y.is_negative() // Check lower bounds
                            && next_x < upper_bound && next_y < upper_bound // Check upper bounds
                            && matrix[next_x as usize].vector[next_y as usize].character == 'S'
                    }) {
                        // println!("  Increasing counter!");
                        acc += 1;
                    }
                }
            }
        }
        acc
    })
}

fn search_corners(
    coord: [usize; 2],
    searchee: char,
    search_area: &Vec<IndexedVec>,
) -> Vec<[usize; 2]> {
    // An item is adjacent if it is one away, but it cannot cross boundaries
    let furthest_column = search_area[0].vector.len();
    let deepest_row = search_area.len();
    let [row, col] = coord;
    const CORNERS: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    CORNERS.iter().fold(Vec::new(), |mut acc, (x, y)| {
        let loc = [x + row as isize, y + col as isize];
        if loc[0] >= 0
            && (loc[0] + 1 < furthest_column as isize)
            && loc[1] >= 0
            && (loc[1] + 1 < deepest_row as isize)
        {
            // println! {"Checking corner ({}, {})", loc[0], loc[1]};
            if search_area[loc[0] as usize].vector[loc[1] as usize].character == searchee {
                acc.push([loc[0] as usize, loc[1] as usize]);
            }
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn vector_of_str_becomes_matrix() {
    //     let input = "abc\ndef\nghi".split('\n');
    //     let result = enter_the_matrix(input);
    //     assert_eq!(
    //         result,
    //         [
    //             (0, [(0, 'a'), (1, 'b'), (2, 'c')]),
    //             (1, [(0, 'd'), (1, 'e'), (2, 'f')]),
    //             (2, [(0, 'g'), (1, 'h'), (2, 'i')])
    //         ]
    //     );
    // }
}
