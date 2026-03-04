use crate::include::*;
pub fn iniciar_lade(sistema: &mut SystemTable<Boot>) {
    let _ = sistema.stdout().enable_cursor(false);
    let (max_cols, max_rows) = if let Ok(Some(modo)) = sistema.stdout().current_mode() {
        (modo.columns(), modo.rows())
    } else {
        (80, 25)
    };

    let mut color_fondo = Color::Cyan;
    let mut menu_abierto = false;
    let mut ventana_activa = 1;
    let mut monitor_abierto = false;
    let mut calc_abierta = false;
    let mut c_n1: i64 = 0;
    let mut c_n2: i64 = 0;
    let mut c_op = ' ';
    let mut c_res: i64 = 0;
    let mut c_paso = 0;
    let mut sel_menu = 0;

    loop {
        sistema.stdout().set_color(Color::White, color_fondo).unwrap();
        sistema.stdout().clear().unwrap();

        if ventana_activa == 1 {
            dibujar_ventana(sistema, max_cols, max_rows, 50, 10, "LADE - LibreAr DE", "Bienvenido a LADE.\nPresiona [ESC] para ver el menu.\n\n[ENTER] Cerrar ventana", Color::White, Color::Blue);
        }

        if calc_abierta {
            dibujar_calculadora(sistema, max_cols, max_rows, c_n1, c_n2, c_op, c_res, c_paso);
        }

        if monitor_abierto {
            dibujar_monitor_sistema(sistema, max_cols, max_rows);
        }

        if menu_abierto {
            dibujar_ventana(sistema, max_cols, max_rows, 26, 12, "Menu MtrxOS", "", Color::Black, Color::LightGray);
            
            let mx = (max_cols.saturating_sub(26)) / 2 + 2;
            let my = (max_rows.saturating_sub(12)) / 2 + 2;
            
            let op1 = if sel_menu == 0 { "> 1. Fondo" } else { "  1. Fondo" };
            let op2 = if sel_menu == 1 { "> 2. Acerca de" } else { "  2. Acerca de" };
            let op3 = if sel_menu == 2 { "> 3. Monitor" } else { "  3. Monitor" };
            let op4 = if sel_menu == 3 { "> 4. Calculadora" } else { "  4. Calculadora" };
            let op5 = if sel_menu == 4 { "> 5. Reiniciar" } else { "  5. Reiniciar" };
            let op6 = if sel_menu == 5 { "> 6. Apagar" } else { "  6. Apagar" };
            let op7 = if sel_menu == 6 { "> 7. Salir" } else { "  7. Salir" };
        
            let _ = sistema.stdout().set_cursor_position(mx, my);
            let _ = write!(sistema.stdout(), "{}", op1);
            let _ = sistema.stdout().set_cursor_position(mx, my + 1);
            let _ = write!(sistema.stdout(), "{}", op2);
            let _ = sistema.stdout().set_cursor_position(mx, my + 2);
            let _ = write!(sistema.stdout(), "{}", op3);
            let _ = sistema.stdout().set_cursor_position(mx, my + 3);
            let _ = write!(sistema.stdout(), "{}", op4);
            let _ = sistema.stdout().set_cursor_position(mx, my + 4);
            let _ = write!(sistema.stdout(), "{}", op5);
            let _ = sistema.stdout().set_cursor_position(mx, my + 5);
            let _ = write!(sistema.stdout(), "{}", op6);
            let _ = sistema.stdout().set_cursor_position(mx, my + 6);
            let _ = write!(sistema.stdout(), "{}", op7);
        }

        loop {
            if let Ok(Some(evento)) = sistema.stdin().read_key() {
                match evento {
                    Key::Special(ScanCode::ESCAPE) => {
                        if menu_abierto {
                            menu_abierto = false;
                        } else if monitor_abierto {
                            monitor_abierto = false;
                        } else if calc_abierta {
                            calc_abierta = false;
                            c_n1 = 0; c_n2 = 0; c_op = ' '; c_paso = 0;
                        } else if ventana_activa != 0 {
                            ventana_activa = 0;
                        } else {
                            menu_abierto = true;
                            sel_menu = 0;
                        }
                        break;
                    }
                    Key::Special(ScanCode::UP) => {
                        if menu_abierto && sel_menu > 0 { sel_menu -= 1; }
                        break;
                    }
                    Key::Special(ScanCode::DOWN) => {
                        if menu_abierto && sel_menu < 6 { sel_menu += 1; }
                        break;
                    }
                    Key::Printable(t) => {
                        let v = u16::from(t);
                        let c = v as u8 as char;
                    
                        if calc_abierta {
                            match c {
                                '0'..='9' => {
                                    let num = (c as i64) - 48;
                                    if c_paso <= 1 { c_n1 = (c_n1 * 10) + num; c_paso = 1; }
                                    else if c_paso >= 2 && c_paso < 4 { c_n2 = (c_n2 * 10) + num; c_paso = 3; }
                                }
                                '+' | '-' | '*' | '/' => {
                                    if c_paso == 1 { c_op = c; c_paso = 2; }
                                }
                                _ if v == 13 => {
                                    if c_paso == 3 {
                                        c_res = match c_op {
                                            '+' => c_n1 + c_n2,
                                            '-' => c_n1 - c_n2,
                                            '*' => c_n1 * c_n2,
                                            '/' => if c_n2 != 0 { c_n1 / c_n2 } else { 0 },
                                            _ => 0,
                                        };
                                        c_paso = 4;
                                    } else if c_paso == 4 {
                                        c_n1 = 0; c_n2 = 0; c_op = ' '; c_res = 0; c_paso = 0;
                                    }
                                }
                                _ => {}
                            }
                        } else if v == 13 {
                            if menu_abierto {
                                match sel_menu {
                                    0 => {
                                        color_fondo = match color_fondo {
                                            Color::Cyan => Color::Red,
                                            Color::Red => Color::Green,
                                            Color::Green => Color::Black,
                                            _ => Color::Cyan,
                                        };
                                    }
                                    1 => {
                                        ventana_activa = 1;
                                        monitor_abierto = false;
                                        calc_abierta = false;
                                        menu_abierto = false;
                                    }
                                    2 => {
                                        monitor_abierto = true;
                                        ventana_activa = 0;
                                        calc_abierta = false;
                                        menu_abierto = false;
                                    }
                                    3 => {
                                        calc_abierta = true;
                                        monitor_abierto = false;
                                        ventana_activa = 0;
                                        menu_abierto = false;
                                        c_n1 = 0; c_n2 = 0; c_op = ' '; c_paso = 0;
                                    }
                                    4 => {
                                        sistema.runtime_services().reset(
                                            uefi::table::runtime::ResetType::COLD, 
                                            uefi::Status::SUCCESS, 
                                            None
                                        );
                                    }
                                    5 => {
                                        sistema.runtime_services().reset(
                                            uefi::table::runtime::ResetType::SHUTDOWN, 
                                            uefi::Status::SUCCESS, 
                                            None
                                        );
                                    }
                                    6 => {
                                        let _ = sistema.stdout().set_color(Color::White, Color::Black);
                                        let _ = sistema.stdout().clear();
                                        let _ = sistema.stdout().enable_cursor(true);
                                        return;
                                    }
                                    _ => {}
                                }
                            } else if ventana_activa == 1 {
                                ventana_activa = 0;
                            } else if monitor_abierto {
                                monitor_abierto = false;
                            } else if calc_abierta {
                                if c_paso == 4 { calc_abierta = false; }
                            }
                        }
                        break;
                    }
                    _ => { break; }
                }
            }
        }
    }
}

fn dibujar_ventana(sistema: &mut SystemTable<Boot>, max_cols: usize, max_rows: usize, ancho: usize, alto: usize, titulo: &str, texto: &str, fg: Color, bg: Color) {
    let x = (max_cols.saturating_sub(ancho)) / 2;
    let y = (max_rows.saturating_sub(alto)) / 2;

    sistema.stdout().set_color(Color::Black, Color::Black).unwrap();
    for i in (y + 1)..=(y + alto) {
        let _ = sistema.stdout().set_cursor_position(x + 2, i);
        for _ in 0..ancho { let _ = write!(sistema.stdout(), " "); }
    }

    sistema.stdout().set_color(fg, bg).unwrap();
    for i in y..(y + alto) {
        let _ = sistema.stdout().set_cursor_position(x, i);
        for _ in 0..ancho { let _ = write!(sistema.stdout(), " "); }
    }

    let _ = sistema.stdout().set_cursor_position(x, y);
    let _ = write!(sistema.stdout(), "╔");
    for _ in 0..(ancho.saturating_sub(2)) { let _ = write!(sistema.stdout(), "═"); }
    let _ = write!(sistema.stdout(), "╗");

    for i in 1..(alto.saturating_sub(1)) {
        let _ = sistema.stdout().set_cursor_position(x, y + i);
        let _ = write!(sistema.stdout(), "║");
        let _ = sistema.stdout().set_cursor_position(x + ancho - 1, y + i);
        let _ = write!(sistema.stdout(), "║");
    }

    let _ = sistema.stdout().set_cursor_position(x, y + alto - 1);
    let _ = write!(sistema.stdout(), "╚");
    for _ in 0..(ancho.saturating_sub(2)) { let _ = write!(sistema.stdout(), "═"); }
    let _ = write!(sistema.stdout(), "╝");

    let t_len = titulo.len();
    let t_pos = x + (ancho / 2).saturating_sub((t_len + 4) / 2);
    let _ = sistema.stdout().set_cursor_position(t_pos, y);
    let _ = write!(sistema.stdout(), "[ {} ]", titulo);

    let mut ty = y + 2;
    for linea in texto.split('\n') {
        let l_pos = x + 2;
        let _ = sistema.stdout().set_cursor_position(l_pos, ty);
        let _ = write!(sistema.stdout(), "{}", linea);
        ty += 1;
    }
}

fn dibujar_monitor_sistema(sistema: &mut SystemTable<Boot>, max_cols: usize, max_rows: usize) {
    dibujar_ventana(sistema, max_cols, max_rows, 46, 14, "Monitor de Sistema", "", Color::Black, Color::LightGray);
    
    let x = (max_cols.saturating_sub(46)) / 2 + 3;
    let y = (max_rows.saturating_sub(14)) / 2 + 3;

    let rdtsc = unsafe { core::arch::x86_64::_rdtsc() };
    let cpu_load = (rdtsc % 10) + 1;

    let mut mmap_buf = [0u8; 8192];
    let mut mem_bytes: u64 = 0;
    
    if let Ok(mmap) = sistema.boot_services().memory_map(&mut mmap_buf) {
        for entry in mmap.entries() {
            if entry.ty == uefi::table::boot::MemoryType::CONVENTIONAL {
                mem_bytes += entry.page_count * 4096;
            }
        }
    }
    
    let mem_mb = mem_bytes / (1024 * 1024);
    let mem_gb_int = mem_mb / 1024;
    let mem_gb_frac = (mem_mb % 1024) / 100;

    let _ = sistema.stdout().set_cursor_position(x, y);
    let _ = write!(sistema.stdout(), "CPU Ciclos: [");
    for i in 0..10 {
        if i < cpu_load as usize { let _ = write!(sistema.stdout(), "|"); }
        else { let _ = write!(sistema.stdout(), "."); }
    }
    let _ = write!(sistema.stdout(), "] {}0%", cpu_load);

    let _ = sistema.stdout().set_cursor_position(x, y + 2);
    let _ = write!(sistema.stdout(), "RAM Detectada: {}.{} GB", mem_gb_int, mem_gb_frac);

    let _ = sistema.stdout().set_cursor_position(x, y + 4);
    let _ = write!(sistema.stdout(), "RAM en MB: {} MB", mem_mb);

    let _ = sistema.stdout().set_cursor_position(x, y + 6);
    let _ = write!(sistema.stdout(), "Resolucion: {}x{}", max_cols, max_rows);

    let _ = sistema.stdout().set_cursor_position(x, y + 8);
    let _ = write!(sistema.stdout(), "Presiona [ENTER] o [ESC] para cerrar");
}

fn dibujar_calculadora(sistema: &mut SystemTable<Boot>, max_cols: usize, max_rows: usize, n1: i64, n2: i64, op: char, res: i64, paso: u8) {
    dibujar_ventana(sistema, max_cols, max_rows, 30, 12, "Calculadora Mtrx", "", Color::Black, Color::LightGray);
    let x = (max_cols.saturating_sub(30)) / 2 + 4;
    let y = (max_rows.saturating_sub(12)) / 2 + 3;

    let _ = sistema.stdout().set_cursor_position(x, y);
    if paso >= 1 { let _ = write!(sistema.stdout(), "Num 1: {}", n1); }
    else { let _ = write!(sistema.stdout(), "Num 1: _"); }
    
    let _ = sistema.stdout().set_cursor_position(x, y + 2);
    if paso >= 2 { let _ = write!(sistema.stdout(), "Oper:  {}", op); }
    else { let _ = write!(sistema.stdout(), "Oper:  _"); }

    let _ = sistema.stdout().set_cursor_position(x, y + 4);
    if paso >= 3 { let _ = write!(sistema.stdout(), "Num 2: {}", n2); }
    else { let _ = write!(sistema.stdout(), "Num 2: _"); }

    let _ = sistema.stdout().set_cursor_position(x, y + 6);
    let _ = write!(sistema.stdout(), "---------------");

    let _ = sistema.stdout().set_cursor_position(x, y + 7);
    if paso == 4 {
        let _ = write!(sistema.stdout(), "TOTAL: {}", res);
    } else {
        let _ = write!(sistema.stdout(), "TOTAL: ?");
    }
}
