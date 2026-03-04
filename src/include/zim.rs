use crate::include::*;
pub fn iniciar_ide(sistema: &mut SystemTable<Boot>) {

    let (max_cols, max_rows) = if let Some(modo) = sistema.stdout().current_mode().ok().and_then(|m| m) {
        (modo.columns(), modo.rows())
    } else {
        (80, 25)
    };

    let mut buffer = [0u8; 2048];
    let mut len = 0;
    let mut pos = 0;

    let _ = sistema.stdout().enable_cursor(true);

    loop {
        sistema.stdout().clear().unwrap();
        let _ = sistema.stdout().set_cursor_position(0, 0);
        let _ = write!(sistema.stdout(), "ZIM IDE | M Lang | F5: RUN | ESC: EXIT | FLECHAS: Mv Cursor\n");
        let _ = write!(sistema.stdout(), "-----------------------------------------------------------\n");

        let txt = core::str::from_utf8(&buffer[..len]).unwrap_or("");
        let _ = write!(sistema.stdout(), "{}", txt);

        let mut cx = 0;
        let mut cy = 2;
        for i in 0..pos {
            if buffer[i] == b'\n' {
                cx = 0;
                cy += 1;
            } else {
                cx += 1;
                if cx >= max_cols {
                    cx = 0;
                    cy += 1;
                }
            }
        }

        let cy_limitado = if cy >= max_rows { max_rows - 1 } else { cy };
        let _ = sistema.stdout().set_cursor_position(cx, cy_limitado);

        loop {
            if let Ok(Some(evento)) = sistema.stdin().read_key() {
                match evento {
                    Key::Printable(t) => {
                        if cy < max_rows - 1 {
                            let v = u16::from(t);
                            
                            if v == 8 {
                                if pos > 0 {
                                    for i in pos..len { buffer[i - 1] = buffer[i]; }
                                    pos -= 1;
                                    len -= 1;
                                }
                            } 
                            else if v == 13 {
                                if len < 2048 {
                                    for i in (pos..len).rev() { buffer[i + 1] = buffer[i]; }
                                    buffer[pos] = b'\n';
                                    pos += 1;
                                    len += 1;
                                }
                            } 
                            else if v >= 32 && v <= 126 {
                                if len < 2048 {
                                    for i in (pos..len).rev() { buffer[i + 1] = buffer[i]; }
                                    buffer[pos] = v as u8;
                                    pos += 1;
                                    len += 1;
                                }
                            }
                        }
                        break; 
                    }
                    Key::Special(s) => {
                        match s {
                            ScanCode::ESCAPE => {
                                sistema.stdout().clear().unwrap();
                                let _ = sistema.stdout().enable_cursor(true);
                                return;
                            }
                            ScanCode::FUNCTION_5 => {
                                let _ = sistema.stdout().enable_cursor(false);
                                ejecutar_codigo(sistema, &buffer[..len]);
                                let _ = sistema.stdout().enable_cursor(true);
                                break;
                            }
                            ScanCode::LEFT => { if pos > 0 { pos -= 1; } }
                            ScanCode::RIGHT => { if pos < len { pos += 1; } }
                            ScanCode::UP => {
                                let mut start = 0;
                                for i in (0..pos).rev() {
                                    if buffer[i] == b'\n' { start = i + 1; break; }
                                }
                                let col = pos - start;
                                if start > 1 {
                                    let mut prev_start = 0;
                                    for i in (0..start - 1).rev() {
                                        if buffer[i] == b'\n' { prev_start = i + 1; break; }
                                    }
                                    let prev_len = (start - 1) - prev_start;
                                    pos = prev_start + if col > prev_len { prev_len } else { col };
                                }
                            }
                            ScanCode::DOWN => {
                                let mut start = 0;
                                for i in (0..pos).rev() {
                                    if buffer[i] == b'\n' { start = i + 1; break; }
                                }
                                let col = pos - start;
                                let mut next_line = None;
                                for i in pos..len {
                                    if buffer[i] == b'\n' { next_line = Some(i + 1); break; }
                                }
                                if let Some(n_start) = next_line {
                                    let mut next_end = len;
                                    for i in n_start..len {
                                        if buffer[i] == b'\n' { next_end = i; break; }
                                    }
                                    let n_len = next_end - n_start;
                                    pos = n_start + if col > n_len { n_len } else { col };
                                }
                            }
                            _ => {}
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn ejecutar_codigo(sistema: &mut SystemTable<Boot>, codigo: &[u8]) {
    sistema.stdout().clear().unwrap();
    let mut vars_noms: [[u8; 8]; 20] = [[0; 8]; 20];
    let mut vars_vals: [i64; 20] = [0; 20];
    let mut v_count = 0;

    let txt = core::str::from_utf8(codigo).unwrap_or("");
    let mut lineas = [""; 100];
    for (i, l) in txt.lines().enumerate() {
        if i < 100 { lineas[i] = l; }
    }

    let mut pc = 0;
    while pc < 100 {
        let l = lineas[pc].trim();
        pc += 1;

        if l.is_empty() || l.starts_with(':') { continue; }

        let mut partes = l.splitn(2, ' ');
        let cmd = partes.next().unwrap_or("");
        let arg = partes.next().unwrap_or("").trim();

        match cmd {
            "rand" => {
                let mut p = arg.splitn(2, ' ');
                let var_nom = p.next().unwrap_or("");
                let max = p.next().unwrap_or("100").parse::<i64>().unwrap_or(100);
                
                let semillita = unsafe { core::arch::x86_64::_rdtsc() };
                let n_rand = (semillita % max as u64) as i64;
            
                for i in 0..v_count {
                    let len = vars_noms[i].iter().position(|&x| x == 0).unwrap_or(8);
                    if core::str::from_utf8(&vars_noms[i][..len]).unwrap() == var_nom {
                        vars_vals[i] = n_rand;
                        break;
                    }
                }
            }
            "reset" => {
                v_count = 0;
                vars_vals = [0; 20];
                vars_noms = [[0; 8]; 20];
                let _ = write!(sistema.stdout(), "Memoria limpia.\n");
            }
            "mul" | "div" => {
                let mut p = arg.splitn(2, ' ');
                let n = p.next().unwrap_or("");
                let v_str = p.next().unwrap_or("1");
                
                let mut v = v_str.parse::<i64>().unwrap_or(0);
                for i in 0..v_count {
                    let len = vars_noms[i].iter().position(|&x| x == 0).unwrap_or(8);
                    if core::str::from_utf8(&vars_noms[i][..len]).unwrap() == v_str {
                        v = vars_vals[i]; break;
                    }
                }
            
                for i in 0..v_count {
                    let len = vars_noms[i].iter().position(|&x| x == 0).unwrap_or(8);
                    if core::str::from_utf8(&vars_noms[i][..len]).unwrap() == n {
                        if cmd == "mul" { 
                            vars_vals[i] *= v; 
                        } else if v != 0 { 
                            vars_vals[i] /= v; 
                        } else {
                            let _ = write!(sistema.stdout(), "Err: Div por 0\n");
                        }
                        break;
                    }
                }
            }
            "goto" => {
                for i in 0..100 {
                    if lineas[i].trim().starts_with(':') && &lineas[i].trim()[1..] == arg {
                        pc = i;
                        break;
                    }
                }
            }
            "if" => {
                let mut cond = arg.split_whitespace();
                let v_name = cond.next().unwrap_or("");
                let op = cond.next().unwrap_or("");
                let v_target = cond.next().unwrap_or("0").parse::<i64>().unwrap_or(0);
                
                let mut actual_val = 0;
                for i in 0..v_count {
                    let len = vars_noms[i].iter().position(|&x| x == 0).unwrap_or(8);
                    if core::str::from_utf8(&vars_noms[i][..len]).unwrap() == v_name {
                        actual_val = vars_vals[i];
                        break;
                    }
                }
                if op == "==" && actual_val != v_target { pc += 1; }
                if op == ">" && actual_val <= v_target { pc += 1; }
                if op == "<" && actual_val >= v_target { pc += 1; }
            }
            "print" => {
                if arg.starts_with('"') && arg.ends_with('"') {
                    writeln!(sistema.stdout(), "{}", &arg[1..arg.len()-1]).unwrap();
                } else {
                    let mut f = false;
                    for i in 0..v_count {
                        let len = vars_noms[i].iter().position(|&x| x == 0).unwrap_or(8);
                        if core::str::from_utf8(&vars_noms[i][..len]).unwrap() == arg {
                            writeln!(sistema.stdout(), "{}", vars_vals[i]).unwrap();
                            f = true; break;
                        }
                    }
                    if !f { writeln!(sistema.stdout(), "Err: {}", arg).unwrap(); }
                }
            }
            "let" => {
                let mut asig = arg.splitn(2, '=');
                let n = asig.next().unwrap_or("").trim();
                let v_str = asig.next().unwrap_or("").trim();
                
                let mut v = v_str.parse::<i64>().unwrap_or(0);
                for i in 0..v_count {
                    let len = vars_noms[i].iter().position(|&x| x == 0).unwrap_or(8);
                    if core::str::from_utf8(&vars_noms[i][..len]).unwrap() == v_str {
                        v = vars_vals[i]; break;
                    }
                }

                let mut f = false;
                for i in 0..v_count {
                    let len = vars_noms[i].iter().position(|&x| x == 0).unwrap_or(8);
                    if core::str::from_utf8(&vars_noms[i][..len]).unwrap() == n {
                        vars_vals[i] = v; f = true; break;
                    }
                }
                if !f && v_count < 20 {
                    let b = n.as_bytes();
                    vars_noms[v_count][..b.len().min(8)].copy_from_slice(&b[..b.len().min(8)]);
                    vars_vals[v_count] = v; v_count += 1;
                }
            }
            "input" => {
                write!(sistema.stdout(), "? ").unwrap();
                let mut in_b = [0u8; 16];
                let mut in_idx = 0;
                loop {
                    if let Ok(Some(Key::Printable(t))) = sistema.stdin().read_key() {
                        let c = u16::from(t);
                        if c == 13 { writeln!(sistema.stdout()).unwrap(); break; }
                        if c == 8 && in_idx > 0 { in_idx -= 1; write!(sistema.stdout(), "\x08 \x08").unwrap(); }
                        if c >= 32 && c <= 126 && in_idx < 16 {
                            in_b[in_idx] = c as u8; in_idx += 1;
                            write!(sistema.stdout(), "{}", c as u8 as char).unwrap();
                        }
                    }
                }
                let val = core::str::from_utf8(&in_b[..in_idx]).unwrap_or("0").parse::<i64>().unwrap_or(0);
                let mut f = false;
                for i in 0..v_count {
                    let len = vars_noms[i].iter().position(|&x| x == 0).unwrap_or(8);
                    if core::str::from_utf8(&vars_noms[i][..len]).unwrap() == arg {
                        vars_vals[i] = val; f = true; break;
                    }
                }
                if !f && v_count < 20 {
                    let b = arg.as_bytes();
                    vars_noms[v_count][..b.len().min(8)].copy_from_slice(&b[..b.len().min(8)]);
                    vars_vals[v_count] = val; v_count += 1;
                }
            }
            "add" | "sub" => {
                let mut p = arg.splitn(2, ' ');
                let n = p.next().unwrap_or("");
                let v_str = p.next().unwrap_or("0");
                
                let mut v = v_str.parse::<i64>().unwrap_or(0);
                for i in 0..v_count {
                    let len = vars_noms[i].iter().position(|&x| x == 0).unwrap_or(8);
                    if core::str::from_utf8(&vars_noms[i][..len]).unwrap() == v_str {
                        v = vars_vals[i]; break;
                    }
                }

                for i in 0..v_count {
                    let len = vars_noms[i].iter().position(|&x| x == 0).unwrap_or(8);
                    if core::str::from_utf8(&vars_noms[i][..len]).unwrap() == n {
                        if cmd == "add" { vars_vals[i] += v; } else { vars_vals[i] -= v; }
                        break;
                    }
                }
            }
            "delay" => {
                let ms = arg.parse::<u64>().unwrap_or(0);
                sistema.boot_services().stall((ms * 1000) as usize);
            }
            "clear" => { sistema.stdout().clear().unwrap(); }
            _ => {}
        }
    }
    writeln!(sistema.stdout(), "\n[FIN - PULSA UNA TECLA]").unwrap();
    loop { if let Ok(Some(_)) = sistema.stdin().read_key() { break; } }
}