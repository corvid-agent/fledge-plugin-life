/// Grid dimensions.
pub const W: usize = 50;
pub const H: usize = 25;

/// A Game of Life board.
pub type Grid = [[bool; W]; H];

/// Count live neighbors of cell (r, c).
pub fn neighbors(grid: &Grid, r: usize, c: usize) -> u8 {
    let mut n = 0u8;
    for dr in [-1i32, 0, 1] {
        for dc in [-1i32, 0, 1] {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0
                && nr < H as i32
                && nc >= 0
                && nc < W as i32
                && grid[nr as usize][nc as usize]
            {
                n += 1;
            }
        }
    }
    n
}

/// Advance the grid by one generation using standard B3/S23 rules.
pub fn step(grid: &Grid) -> Grid {
    let mut next = [[false; W]; H];
    for r in 0..H {
        for c in 0..W {
            let n = neighbors(grid, r, c);
            next[r][c] = if grid[r][c] { n == 2 || n == 3 } else { n == 3 };
        }
    }
    next
}

/// Count live cells.
pub fn population(grid: &Grid) -> usize {
    grid.iter().flat_map(|r| r.iter()).filter(|&&c| c).count()
}

/// Place a pattern on the grid at offset (r, c).
pub fn set(grid: &mut Grid, r: usize, c: usize, cells: &[(i32, i32)]) {
    for &(dr, dc) in cells {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nr < H as i32 && nc >= 0 && nc < W as i32 {
            grid[nr as usize][nc as usize] = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_grid_stays_empty() {
        let grid: Grid = [[false; W]; H];
        let next = step(&grid);
        assert_eq!(population(&next), 0);
    }

    #[test]
    fn block_is_still_life() {
        // 2x2 block is a still life (stable pattern)
        let mut grid: Grid = [[false; W]; H];
        grid[5][5] = true;
        grid[5][6] = true;
        grid[6][5] = true;
        grid[6][6] = true;

        let next = step(&grid);
        assert_eq!(population(&next), 4);
        assert!(next[5][5] && next[5][6] && next[6][5] && next[6][6]);
    }

    #[test]
    fn blinker_oscillates() {
        // Horizontal blinker (period 2)
        let mut grid: Grid = [[false; W]; H];
        grid[10][10] = true;
        grid[10][11] = true;
        grid[10][12] = true;

        let gen1 = step(&grid);
        // Should become vertical
        assert!(gen1[9][11] && gen1[10][11] && gen1[11][11]);
        assert!(!gen1[10][10] && !gen1[10][12]);

        let gen2 = step(&gen1);
        // Should return to horizontal
        assert!(gen2[10][10] && gen2[10][11] && gen2[10][12]);
        assert!(!gen2[9][11] && !gen2[11][11]);
    }

    #[test]
    fn glider_moves() {
        // Standard glider heading SE
        let mut grid: Grid = [[false; W]; H];
        set(&mut grid, 2, 2, &[(-1, 0), (0, 1), (1, -1), (1, 0), (1, 1)]);

        let initial_pop = population(&grid);
        assert_eq!(initial_pop, 5);

        // After 4 generations, a glider translates one cell diagonally
        let mut g = grid;
        for _ in 0..4 {
            g = step(&g);
        }
        assert_eq!(population(&g), 5, "glider should preserve population");
    }

    #[test]
    fn lone_cell_dies() {
        let mut grid: Grid = [[false; W]; H];
        grid[10][10] = true;

        let next = step(&grid);
        assert_eq!(population(&next), 0);
    }

    #[test]
    fn three_in_corner_reproduce() {
        // Three cells with a shared neighbor create a new cell
        let mut grid: Grid = [[false; W]; H];
        grid[0][0] = true;
        grid[0][1] = true;
        grid[1][0] = true;

        let next = step(&grid);
        // (1,1) should be born (3 neighbors)
        assert!(next[1][1]);
        assert_eq!(population(&next), 4); // all original survive + new cell
    }

    #[test]
    fn neighbors_count_correct() {
        let mut grid: Grid = [[false; W]; H];
        grid[5][5] = true;
        grid[5][6] = true;
        grid[6][5] = true;

        assert_eq!(neighbors(&grid, 5, 5), 2);
        assert_eq!(neighbors(&grid, 6, 6), 3); // dead cell with 3 neighbors
        assert_eq!(neighbors(&grid, 4, 4), 1);
    }

    #[test]
    fn set_places_pattern() {
        let mut grid: Grid = [[false; W]; H];
        set(&mut grid, 5, 5, &[(0, 0), (0, 1), (1, 0)]);

        assert!(grid[5][5]);
        assert!(grid[5][6]);
        assert!(grid[6][5]);
        assert_eq!(population(&grid), 3);
    }

    #[test]
    fn set_clips_out_of_bounds() {
        let mut grid: Grid = [[false; W]; H];
        // Placing near edge should not panic
        set(&mut grid, 0, 0, &[(-1, -1), (0, 0), (1, 1)]);
        // Only (0,0) and (1,1) should be placed; (-1,-1) is out of bounds
        assert!(grid[0][0]);
        assert!(grid[1][1]);
        assert_eq!(population(&grid), 2);
    }
}
