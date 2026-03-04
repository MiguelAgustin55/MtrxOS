use crate::include::*;
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

pub struct MtrxGl {
    fb: *mut u32,
    pub width: i32,
    pub height: i32,
}

impl MtrxGl {
    pub fn init(sistema: &mut SystemTable<Boot>) -> Option<Self> {
        let bs = sistema.boot_services();
        if let Ok(handle) = bs.get_handle_for_protocol::<GraphicsOutput>() {
            if let Ok(mut gop) = bs.open_protocol_exclusive::<GraphicsOutput>(handle) {
                let info = gop.current_mode_info();
                let (w, h) = info.resolution();
                let fb_ptr = gop.frame_buffer().as_mut_ptr() as *mut u32;
                return Some(Self {
                    fb: fb_ptr,
                    width: w as i32,
                    height: h as i32,
                });
            }
        }
        None
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    pub fn clear(&mut self, color: u32) {
        let len = (self.width * self.height) as usize;
        unsafe {
            let s = slice::from_raw_parts_mut(self.fb, len);
            s.fill(color);
        }
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            unsafe {
                *self.fb.add((y * self.width + x) as usize) = color;
            }
        }
    }

    fn h_line(&mut self, x1: i32, x2: i32, y: i32, color: u32) {
        if y < 0 || y >= self.height { return; }
        let start_x = x1.max(0);
        let end_x = x2.min(self.width);
        if start_x >= end_x { return; }
        let offset = (y * self.width + start_x) as usize;
        let len = (end_x - start_x) as usize;
        unsafe {
            let s = slice::from_raw_parts_mut(self.fb.add(offset), len);
            s.fill(color);
        }
    }

    fn v_line(&mut self, x: i32, y1: i32, y2: i32, color: u32) {
        if x < 0 || x >= self.width { return; }
        let start_y = y1.max(0);
        let end_y = y2.min(self.height);
        for y in start_y..end_y {
            unsafe { *self.fb.add((y * self.width + x) as usize) = color; }
        }
    }

    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: u32) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.draw_pixel(x0, y0, color);
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x0 += sx; }
            if e2 <= dx { err += dx; y0 += sy; }
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        self.h_line(x, x + w, y, color);
        self.h_line(x, x + w, y + h - 1, color);
        self.v_line(x, y, y + h, color);
        self.v_line(x + w - 1, y, y + h, color);
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        let y1 = y.max(0);
        let y2 = (y + h).min(self.height);
        for cy in y1..y2 {
            self.h_line(x, x + w, cy, color);
        }
    }

    pub fn draw_circle(&mut self, cx: i32, cy: i32, r: i32, color: u32) {
        let mut x = 0;
        let mut y = r;
        let mut d = 3 - 2 * r;
        while y >= x {
            self.draw_pixel(cx + x, cy + y, color);
            self.draw_pixel(cx - x, cy + y, color);
            self.draw_pixel(cx + x, cy - y, color);
            self.draw_pixel(cx - x, cy - y, color);
            self.draw_pixel(cx + y, cy + x, color);
            self.draw_pixel(cx - y, cy + x, color);
            self.draw_pixel(cx + y, cy - x, color);
            self.draw_pixel(cx - y, cy - x, color);
            x += 1;
            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }
        }
    }

    pub fn fill_circle(&mut self, cx: i32, cy: i32, r: i32, color: u32) {
        let mut x = 0;
        let mut y = r;
        let mut d = 3 - 2 * r;
        while y >= x {
            self.h_line(cx - x, cx + x + 1, cy + y, color);
            self.h_line(cx - x, cx + x + 1, cy - y, color);
            self.h_line(cx - y, cx + y + 1, cy + x, color);
            self.h_line(cx - y, cx + y + 1, cy - x, color);
            x += 1;
            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }
        }
    }

    pub fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: u32) {
        self.draw_line(x1, y1, x2, y2, color);
        self.draw_line(x2, y2, x3, y3, color);
        self.draw_line(x3, y3, x1, y1, color);
    }

    pub fn fill_triangle(&mut self, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, mut x3: i32, mut y3: i32, color: u32) {
        if y1 > y2 { core::mem::swap(&mut x1, &mut x2); core::mem::swap(&mut y1, &mut y2); }
        if y1 > y3 { core::mem::swap(&mut x1, &mut x3); core::mem::swap(&mut y1, &mut y3); }
        if y2 > y3 { core::mem::swap(&mut x2, &mut x3); core::mem::swap(&mut y2, &mut y3); }
        let th = y3 - y1;
        if th == 0 { return; }
        for i in 0..th {
            let sh = i > y2 - y1 || y2 == y1;
            let sh_h = if sh { y3 - y2 } else { y2 - y1 };
            if sh_h == 0 { continue; }
            let mut a = x1 + (x3 - x1) * i / th;
            let mut b = if sh {
                x2 + (x3 - x2) * (i - (y2 - y1)) / sh_h
            } else {
                x1 + (x2 - x1) * i / sh_h
            };
            if a > b { core::mem::swap(&mut a, &mut b); }
            self.h_line(a, b + 1, y1 + i, color);
        }
    }

    pub fn draw_image(&mut self, dx: i32, dy: i32, img_w: i32, img_h: i32, data: &[u32]) {
        for y in 0..img_h {
            let sy = dy + y;
            if sy < 0 || sy >= self.height { continue; }
            let mut rs = 0;
            let mut re = img_w;
            if dx < 0 { rs = -dx; }
            if dx + img_w > self.width { re = self.width - dx; }
            if rs >= re { continue; }
            let so = (sy * self.width + dx + rs) as usize;
            let io = (y * img_w + rs) as usize;
            let len = (re - rs) as usize;
            unsafe {
                let dst = slice::from_raw_parts_mut(self.fb.add(so), len);
                let src = &data[io..io + len];
                dst.copy_from_slice(src);
            }
        }
    }
}