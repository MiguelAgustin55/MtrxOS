use crate::include::*;
fn isin(a: i32) -> i32 {
    let angle = ((a % 360) + 360) % 360;
    let lut = [
        0, 4, 8, 13, 17, 22, 26, 31, 35, 40, 44, 48, 53, 57, 61, 65, 70, 74, 78, 83, 87, 91, 95, 99, 103, 107, 111,
        115, 119, 123, 127, 131, 135, 138, 142, 146, 149, 153, 156, 160, 163, 167, 170, 173, 177, 180, 183, 186, 189,
        192, 195, 198, 200, 203, 206, 208, 211, 213, 215, 218, 220, 222, 224, 226, 228, 229, 231, 233, 234, 236, 237,
        239, 240, 241, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 253, 254, 254, 255, 255, 255,
    ];
    if angle <= 90 {
        lut[angle as usize]
    } else if angle <= 180 {
        lut[(180 - angle) as usize]
    } else if angle <= 270 {
        -lut[(angle - 180) as usize]
    } else {
        -lut[(360 - angle) as usize]
    }
}

fn icos(a: i32) -> i32 {
    isin(a + 90)
}

pub fn iniciar_juego(sistema: &mut SystemTable<Boot>) {
    sistema.stdout().clear().unwrap();
    let _ = sistema.stdout().enable_cursor(false);
    let map = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 1, 1, 0, 1],
        [1, 0, 1, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1, 0, 0, 1],
        [1, 1, 1, 0, 1, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut px: i32 = 1536;
    let mut py: i32 = 1536;
    let mut pa: i32 = 90;
    let mut buffer = [b' '; 80 * 25];

    loop {
        if let Ok(Some(evento)) = sistema.stdin().read_key() {
            if let Key::Printable(t) = evento {
                let c = u16::from(t) as u8 as char;
                let dx = (icos(pa) * 256) / 256;
                let dy = (isin(pa) * 256) / 256;

                if c == 'q' {
                    break;
                } else if c == 'w' {
                    let nx = px + dx;
                    let ny = py + dy;
                    if map[(ny / 1024) as usize][(nx / 1024) as usize] == 0 {
                        px = nx;
                        py = ny;
                    }
                } else if c == 's' {
                    let nx = px - dx;
                    let ny = py - dy;
                    if map[(ny / 1024) as usize][(nx / 1024) as usize] == 0 {
                        px = nx;
                        py = ny;
                    }
                } else if c == 'a' {
                    pa = ((pa - 8) + 360) % 360;
                } else if c == 'd' {
                    pa = (pa + 8) % 360;
                }
            }
        }

        for x in 0..80 {
            let ra = ((pa - 30 + (x * 60) / 80) + 360) % 360;
            let step_x = (icos(ra) * 32) / 256;
            let step_y = (isin(ra) * 32) / 256;

            let mut rx = px;
            let mut ry = py;
            let mut step = 0;

            while step < 250 {
                rx += step_x;
                ry += step_y;
                let mx = rx / 1024;
                let my = ry / 1024;

                if mx < 0 || mx > 7 || my < 0 || my > 7 || map[my as usize][mx as usize] == 1 {
                    break;
                }
                step += 1;
            }

            let mut dist = step * 32;
            let fix = icos(((ra - pa) + 360) % 360);
            dist = (dist * fix) / 256;
            if dist <= 0 {
                dist = 1;
            }

            let mut h = (25 * 768) / dist;
            if h > 24 {
                h = 24;
            }

            let start_y = 12 - h / 2;
            let end_y = 12 + h / 2;

            let char_wall = if dist < 500 {
                b'#'
            } else if dist < 1000 {
                b'X'
            } else if dist < 1500 {
                b'='
            } else if dist < 2000 {
                b'-'
            } else {
                b':'
            };

            for y in 0..24 {
                let idx = (y * 80 + x) as usize;
                if y < start_y {
                    buffer[idx] = b' ';
                } else if y >= start_y && y <= end_y {
                    buffer[idx] = char_wall;
                } else {
                    buffer[idx] = b'.';
                }
            }
        }

        for y in 0..24 {
            let row = core::str::from_utf8(&buffer[(y * 80)..((y + 1) * 80)]).unwrap_or("");
            let _ = sistema.stdout().set_cursor_position(0, y as usize);
            let _ = write!(sistema.stdout(), "{}", row);
        }

        let _ = sistema.stdout().set_cursor_position(0, 24);
        let _ = write!(sistema.stdout(), "RAYCASTER | W/S: Mover | A/D: Girar | Q: Salir");

        sistema.boot_services().stall(30_000);
    }

    sistema.stdout().clear().unwrap();
    let _ = sistema.stdout().enable_cursor(true);
}