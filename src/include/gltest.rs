use crate::include::*;
pub fn run(sistema: &mut SystemTable<Boot>, gl: &mut MtrxGl) {
    let mut angle: f32 = 0.0;
    let mut history = [0i32; 120];
    let mut dummy_img = [0u32; 1024];

    for y in 0..32 {
        for x in 0..32 {
            let r = (x * 8) as u8;
            let g = (y * 8) as u8;
            let b = ((x + y) * 4) as u8;
            dummy_img[(y * 32 + x) as usize] = MtrxGl::rgb(r, g, b);
        }
    }

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

        for i in 0..119 {
            history[i] = history[i + 1];
        }
        let perf = 40.0 + f_sin(angle * 3.0) * 20.0 + f_cos(angle * 7.0) * 10.0;
        history[119] = perf as i32;
        angle += 0.05;

        gl.clear(0x050510);

        for i in 0..10 {
            let y = i * (gl.height / 10);
            gl.draw_line(0, y, gl.width, y, 0x111122);
            let x = i * (gl.width / 10);
            gl.draw_line(x, 0, x, gl.height, 0x111122);
        }

        let cx = gl.width / 4;
        let cy = gl.height / 4;
        let r = 30 + (f_sin(angle * 2.0) * 15.0) as i32;
        
        gl.fill_circle(cx, cy, r, 0xAA2255);
        gl.draw_circle(cx, cy, r + 5, 0xFF5588);
        gl.draw_circle(cx, cy, r + 10, 0xFF88AA);

        let tx1 = cx + (f_cos(angle) * 80.0) as i32;
        let ty1 = cy + (f_sin(angle) * 80.0) as i32;
        let tx2 = cx + (f_cos(angle + 2.094) * 80.0) as i32;
        let ty2 = cy + (f_sin(angle + 2.094) * 80.0) as i32;
        let tx3 = cx + (f_cos(angle + 4.188) * 80.0) as i32;
        let ty3 = cy + (f_sin(angle + 4.188) * 80.0) as i32;
        
        gl.fill_triangle(tx1, ty1, tx2, ty2, tx3, ty3, 0x22AA55);
        gl.draw_triangle(tx1, ty1, tx2, ty2, tx3, ty3, 0x55FF88);

        let rx = (gl.width / 2) + (f_sin(angle * 1.5) * 100.0) as i32;
        let ry = (gl.height / 4) + (f_cos(angle * 1.2) * 50.0) as i32;
        gl.fill_rect(rx, ry, 80, 50, 0x2255AA);
        gl.draw_rect(rx - 2, ry - 2, 84, 54, 0x5588FF);

        let ix = (gl.width * 3 / 4) + (f_cos(angle * 2.5) * 80.0) as i32;
        let iy = (gl.height / 4) + (f_sin(angle * 2.0) * 80.0) as i32;
        gl.draw_image(ix, iy, 32, 32, &dummy_img);
        gl.draw_rect(ix - 1, iy - 1, 34, 34, 0xFFFFFF);

        let graph_w = 480;
        let graph_h = 100;
        let graph_x = (gl.width - graph_w) / 2;
        let graph_y = gl.height - graph_h - 20;

        gl.fill_rect(graph_x, graph_y, graph_w, graph_h, 0x0A0A1A);
        gl.draw_rect(graph_x, graph_y, graph_w, graph_h, 0x444466);

        for i in 0..119 {
            let x0 = graph_x + (i as i32 * 4);
            let y0 = graph_y + graph_h - history[i];
            let x1 = graph_x + ((i + 1) as i32 * 4);
            let y1 = graph_y + graph_h - history[i + 1];
            if x1 < graph_x + graph_w {
                gl.draw_line(x0, y0, x1, y1, 0x00FFFF);
            }
        }

        sistema.boot_services().stall(100_000);
    }
}

fn f_sin(x: f32) -> f32 {
    let mut x = x % 6.283185;
    if x < 0.0 { x += 6.283185; }
    if x < 3.141593 {
        4.0 * x * (3.141593 - x) / (9.869604 - 0.8 * x * (3.141593 - x))
    } else {
        x -= 3.141593;
        -4.0 * x * (3.141593 - x) / (9.869604 - 0.8 * x * (3.141593 - x))
    }
}

fn f_cos(x: f32) -> f32 {
    f_sin(x + 1.570796)
}