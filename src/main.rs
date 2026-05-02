#[link(wasm_import_module = "fledge")]
extern "C" {
    fn recv(ptr: *mut u8, max_len: i32) -> i32;
    fn send(ptr: *const u8, len: i32);
    fn exit(code: i32);
}

fn send_msg(msg: &str) {
    unsafe { send(msg.as_ptr(), msg.len() as i32) }
}

fn output(text: &str) {
    let mut escaped = String::with_capacity(text.len() + 32);
    for ch in text.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            c if (c as u32) < 0x20 => {}
            c => escaped.push(c),
        }
    }
    send_msg(&format!(r#"{{"type":"output","text":"{escaped}"}}"#));
}

const W: usize = 40;
const H: usize = 20;
const GENS: usize = 30;

type Grid = [[bool; W]; H];

fn neighbors(grid: &Grid, r: usize, c: usize) -> u8 {
    let mut n = 0u8;
    for dr in [-1i32, 0, 1] {
        for dc in [-1i32, 0, 1] {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < H as i32 && nc >= 0 && nc < W as i32 && grid[nr as usize][nc as usize] {
                n += 1;
            }
        }
    }
    n
}

fn step(grid: &Grid) -> Grid {
    let mut next = [[false; W]; H];
    for r in 0..H {
        for c in 0..W {
            let n = neighbors(grid, r, c);
            next[r][c] = if grid[r][c] { n == 2 || n == 3 } else { n == 3 };
        }
    }
    next
}

fn population(grid: &Grid) -> usize {
    grid.iter().flat_map(|r| r.iter()).filter(|&&c| c).count()
}

fn render(grid: &Grid, gen: usize) {
    let sep = format!("  ╔{}╗", "═".repeat(W));
    let bot = format!("  ╚{}╝", "═".repeat(W));
    output(&format!("  ┌─── Generation {:>2} ─── Population: {:>3} ───┐", gen, population(grid)));
    output(&sep);
    for r in 0..H {
        let mut line = String::with_capacity(W * 3 + 6);
        line.push_str("  ║");
        for c in 0..W {
            line.push(if grid[r][c] { '█' } else { '·' });
        }
        line.push('║');
        output(&line);
    }
    output(&bot);
}

fn set(grid: &mut Grid, r: usize, c: usize, cells: &[(i32, i32)]) {
    for &(dr, dc) in cells {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nr < H as i32 && nc >= 0 && nc < W as i32 {
            grid[nr as usize][nc as usize] = true;
        }
    }
}

fn main() {
    let mut buf = [0u8; 8192];
    unsafe { recv(buf.as_mut_ptr(), buf.len() as i32) };

    let mut grid: Grid = [[false; W]; H];

    // R-pentomino at center — famous chaotic methuselah (stabilizes at gen 1103)
    //  .##
    //  ##.
    //  .#.
    set(&mut grid, H / 2, W / 2, &[
        (-1, 0), (-1, 1), (0, -1), (0, 0), (1, 0),
    ]);

    // Glider heading south-east from top-left
    set(&mut grid, 2, 2, &[
        (-1, 0), (0, 1), (1, -1), (1, 0), (1, 1),
    ]);

    // Glider heading south-east from top-right area
    set(&mut grid, 2, 14, &[
        (-1, 0), (0, 1), (1, -1), (1, 0), (1, 1),
    ]);

    // Lightweight spaceship (LWSS) heading east from left edge
    set(&mut grid, H - 6, 3, &[
        (0, 0), (0, 3), (1, 4), (2, 0), (2, 4), (3, 1), (3, 2), (3, 3), (3, 4),
    ]);

    output("");
    output("  ██████  Conway's Game of Life  ██████");
    output("");
    output(&format!("  Board: {}×{}  │  Generations: {}  │  Patterns: R-pentomino + 2 gliders + LWSS", W, H, GENS));
    output("");

    // Show 8 evenly-spaced snapshots so the evolution is clearly visible
    let frames: &[usize] = &[0, 3, 6, 10, 15, 20, 25, 29];
    for gen in 0..GENS {
        if frames.contains(&gen) {
            render(&grid, gen);
            output("");
        }
        grid = step(&grid);
    }

    output(&format!("  ▸ Final population: {} live cells", population(&grid)));
    output("");

    unsafe { exit(0) };
}
