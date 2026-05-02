#[link(wasm_import_module = "fledge")]
extern "C" {
    fn recv(ptr: *mut u8, max_len: i32) -> i32;
    fn send(ptr: *const u8, len: i32);
    fn exit(code: i32);
}

use std::thread;
use std::time::Duration;

fn send_msg(msg: &str) {
    unsafe { send(msg.as_ptr(), msg.len() as i32) }
}

fn output(text: &str) {
    let mut escaped = String::with_capacity(text.len() + 64);
    for ch in text.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\x1b' => escaped.push_str("\\u001b"),
            c if (c as u32) < 0x20 => {}
            c => escaped.push(c),
        }
    }
    send_msg(&format!(r#"{{"type":"output","text":"{escaped}"}}"#));
}

const W: usize = 50;
const H: usize = 25;
const GENS: usize = 80;
const FRAME_MS: u64 = 120;

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

fn render_frame(grid: &Grid, gen: usize) {
    let pop = population(grid);
    let top = "═".repeat(W);

    let mut frame = String::with_capacity((W + 20) * (H + 6));

    // Clear screen and cursor home
    frame.push_str("\x1b[2J\x1b[H\x1b[?25l");

    // Header
    frame.push_str("\n  \x1b[1;36m██████  Conway's Game of Life  ██████\x1b[0m\n\n");
    frame.push_str(&format!(
        "  Generation: \x1b[1;33m{:>3}\x1b[0m  │  Population: \x1b[1;32m{:>3}\x1b[0m  │  Board: {}×{}\n",
        gen, pop, W, H
    ));

    // Top border
    frame.push_str(&format!("  \x1b[90m╔{}╗\x1b[0m\n", top));

    // Grid rows
    for r in 0..H {
        frame.push_str("  \x1b[90m║\x1b[0m");
        for c in 0..W {
            if grid[r][c] {
                frame.push_str("\x1b[97m█\x1b[0m");
            } else {
                frame.push(' ');
            }
        }
        frame.push_str("\x1b[90m║\x1b[0m\n");
    }

    // Bottom border
    frame.push_str(&format!("  \x1b[90m╚{}╝\x1b[0m\n", top));

    frame.push_str("\n  \x1b[90mR-pentomino · 2 Gliders · LWSS\x1b[0m\n");

    output(&frame);
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

    // R-pentomino at center
    set(&mut grid, H / 2, W / 2, &[
        (-1, 0), (-1, 1), (0, -1), (0, 0), (1, 0),
    ]);

    // Glider heading SE from top-left
    set(&mut grid, 2, 2, &[
        (-1, 0), (0, 1), (1, -1), (1, 0), (1, 1),
    ]);

    // Glider heading SE from mid-top
    set(&mut grid, 2, 14, &[
        (-1, 0), (0, 1), (1, -1), (1, 0), (1, 1),
    ]);

    // LWSS heading east from left
    set(&mut grid, H - 6, 3, &[
        (0, 0), (0, 3), (1, 4), (2, 0), (2, 4), (3, 1), (3, 2), (3, 3), (3, 4),
    ]);

    // Animate
    for gen in 0..GENS {
        render_frame(&grid, gen);
        thread::sleep(Duration::from_millis(FRAME_MS));
        grid = step(&grid);
    }

    // Final frame + restore cursor
    render_frame(&grid, GENS);
    output("\x1b[?25h");
    output(&format!(
        "\n  \x1b[1;33m▸ Simulation complete — {} live cells remaining\x1b[0m\n\n",
        population(&grid)
    ));

    unsafe { exit(0) };
}
