use crate::include::*;
const COLS: usize = 120;
const ROWS: usize = 80;
const SIZE: usize = COLS * ROWS;

pub fn run(sistema: &mut SystemTable<Boot>, gl: &mut MtrxGl) {
    let mut grid = [0u8; SIZE];
    let mut next = [0u8; SIZE];
    let mut seed = 987654321u32;
    let mut color_tick = 0u8;

    for i in 0..SIZE {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        if (seed >> 24) % 6 == 0 {
            grid[i] = 1;
        }
    }

    let cell_w = gl.width / (COLS as i32);
    let cell_h = gl.height / (ROWS as i32);
    let offset_x = (gl.width - cell_w * (COLS as i32)) / 2;
    let offset_y = (gl.height - cell_h * (ROWS as i32)) / 2;

    loop {
        if let Ok(Some(evento)) = sistema.stdin().read_key() {
            match evento {
                Key::Special(ScanCode::ESCAPE) => {
                    sistema.runtime_services().reset(
                        uefi::table::runtime::ResetType::COLD,
                        uefi::Status::SUCCESS,
                        None,
                    );
                },
                Key::Printable(t) => {
                    if u16::from(t) as u8 as char == 'q' {
                        break;
                    }
                }
                _ => {}
            }
        }

        gl.clear(0x050505);
        color_tick = color_tick.wrapping_add(3);

        for y in 0..ROWS {
            for x in 0..COLS {
                let idx = y * COLS + x;
                let mut neighbors = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0 && nx < COLS as i32 && ny >= 0 && ny < ROWS as i32 {
                            let n_idx = (ny as usize) * COLS + (nx as usize);
                            if grid[n_idx] == 1 {
                                neighbors += 1;
                            }
                        }
                    }
                }

                if grid[idx] == 1 {
                    if neighbors == 2 || neighbors == 3 {
                        next[idx] = 1;
                    } else {
                        next[idx] = 0;
                    }
                } else {
                    if neighbors == 3 {
                        next[idx] = 1;
                    } else {
                        next[idx] = 0;
                    }
                }
            }
        }

        for y in 0..ROWS {
            for x in 0..COLS {
                let idx = y * COLS + x;
                grid[idx] = next[idx];
                if grid[idx] == 1 {
                    let px = offset_x + (x as i32) * cell_w;
                    let py = offset_y + (y as i32) * cell_h;

                    let r = 20u8.saturating_add(color_tick / 2);
                    let g = 255u8.saturating_sub(color_tick / 3);
                    let b = 150u8.saturating_add(color_tick / 4);

                    gl.fill_rect(px, py, cell_w - 1, cell_h - 1, MtrxGl::rgb(r, g, b));
                }
            }
        }

        sistema.boot_services().stall(50_000);
    }
}
