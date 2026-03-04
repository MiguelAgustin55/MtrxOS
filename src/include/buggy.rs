use crate::include::*;
pub fn iniciar_juego(sistema: &mut SystemTable<Boot>) {
    sistema.stdout().clear().unwrap();
    let _ = sistema.stdout().enable_cursor(false);
    let mut buggy_y: i32 = 20;
    let buggy_x: usize = 10;
    let mut salto_vel: i32 = 0;
    let mut saltando = false;

    let mut obstaculos_x: [usize; 5] = [0, 0, 0, 0, 0];
    let mut score: u32 = 0;
    let mut tick: u32 = 0;
    let mut vivo = true;

    let _ = sistema.stdout().set_cursor_position(0, 0);
    let _ = write!(sistema.stdout(), "MOON BUGGY - Score: 0 - 'W' o ESPACIO para saltar, 'Q' para salir");
    
    for x in 0..79 {
        let _ = sistema.stdout().set_cursor_position(x, 21);
        let _ = write!(sistema.stdout(), "-");
    }

    loop {
        if !vivo {
            let _ = sistema.stdout().set_cursor_position(30, 10);
            let _ = write!(sistema.stdout(), "GAME OVER! Score: {}", score);
            let _ = sistema.stdout().set_cursor_position(28, 12);
            let _ = write!(sistema.stdout(), "Presiona 'Q' para salir");
            
            loop {
                if let Ok(Some(evento)) = sistema.stdin().read_key() {
                    if let Key::Printable(t) = evento {
                        if u16::from(t) as u8 as char == 'q' {
                            sistema.stdout().clear().unwrap();
                            return;
                        }
                    }
                }
                sistema.boot_services().stall(20_000);
            }
        }

        if let Ok(Some(evento)) = sistema.stdin().read_key() {
            if let Key::Printable(t) = evento {
                let c = u16::from(t) as u8 as char;
                if c == 'q' { break; }
                if (c == 'w' || c == ' ') && !saltando {
                    saltando = true;
                    salto_vel = -3;
                }
            }
        }

        if tick % 3 == 0 {
            let _ = sistema.stdout().set_cursor_position(buggy_x, buggy_y as usize);
            let _ = write!(sistema.stdout(), " ");

            if saltando {
                buggy_y += salto_vel;
                salto_vel += 1;
                if buggy_y >= 20 {
                    buggy_y = 20;
                    saltando = false;
                    salto_vel = 0;
                }
            }

            let _ = sistema.stdout().set_cursor_position(buggy_x, buggy_y as usize);
            let _ = write!(sistema.stdout(), "B");

            for i in 0..5 {
                if obstaculos_x[i] > 0 {
                    let _ = sistema.stdout().set_cursor_position(obstaculos_x[i], 20);
                    let _ = write!(sistema.stdout(), " ");

                    obstaculos_x[i] -= 1;

                    if obstaculos_x[i] == buggy_x && buggy_y >= 20 {
                        vivo = false;
                    }

                    if obstaculos_x[i] > 0 {
                        let _ = sistema.stdout().set_cursor_position(obstaculos_x[i], 20);
                        let _ = write!(sistema.stdout(), "X");
                    } else {
                        score += 10;
                        let _ = sistema.stdout().set_cursor_position(20, 0);
                        let _ = write!(sistema.stdout(), "{}", score);
                    }
                }
            }

            if tick % 45 == 0 {
                if let Ok(tiempo) = sistema.runtime_services().get_time() {
                    if tiempo.nanosecond() % 2 == 0 {
                        for i in 0..5 {
                            if obstaculos_x[i] == 0 {
                                obstaculos_x[i] = 78;
                                break;
                            }
                        }
                    }
                }
            }
        }

        tick = tick.wrapping_add(1);
        sistema.boot_services().stall(20_000);
    }

    sistema.stdout().clear().unwrap();
    let _ = sistema.stdout().enable_cursor(true);
}