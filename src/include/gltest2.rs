use crate::include::*;
pub fn run(sistema: &mut SystemTable<Boot>, gl: &mut MtrxGl) {
    let mut angle: f32 = 0.0;
    let mut bx: i32 = gl.width / 2;
    let mut by: i32 = gl.height / 2;
    let mut bdx: i32 = 5;
    let mut bdy: i32 = 4;
    let mut color_shift: u8 = 0;

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

        gl.clear(0x050515);

        angle += 0.04;
        color_shift = color_shift.wrapping_add(2);

        bx += bdx;
        by += bdy;
        if bx <= 0 || bx + 60 >= gl.width { bdx = -bdx; }
        if by <= 0 || by + 60 >= gl.height { bdy = -bdy; }

        let cx = gl.width / 2;
        let cy = gl.height / 2;

        for i in 1..8 {
            let r = 30 * i as i32 + (f_sin(angle + i as f32) * 15.0) as i32;
            if r > 0 {
                gl.draw_circle(cx, cy, r, MtrxGl::rgb(50, (i * 30) as u8, 255 - (i * 10) as u8));
            }
        }

        let tx1 = cx + (f_cos(angle) * 120.0) as i32;
        let ty1 = cy + (f_sin(angle) * 120.0) as i32;
        let tx2 = cx + (f_cos(angle + 2.094) * 120.0) as i32;
        let ty2 = cy + (f_sin(angle + 2.094) * 120.0) as i32;
        let tx3 = cx + (f_cos(angle + 4.188) * 120.0) as i32;
        let ty3 = cy + (f_sin(angle + 4.188) * 120.0) as i32;

        gl.fill_triangle(tx1, ty1, tx2, ty2, tx3, ty3, 0x1188AA);
        gl.draw_triangle(tx1, ty1, tx2, ty2, tx3, ty3, 0x44EEFF);

        gl.fill_rect(bx, by, 60, 60, MtrxGl::rgb(color_shift, 255 - color_shift, 150));
        gl.draw_rect(bx - 2, by - 2, 64, 64, 0xFFFFFF);

        sistema.boot_services().stall(30_000);
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
