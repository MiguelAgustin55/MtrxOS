use crate::include::*;
pub fn configurar_sistema(sistema: &mut SystemTable<Boot>, nombre_buf: &mut [u8; 32], nombre_l: &mut usize) {
    let (max_cols, max_rows) = if let Ok(Some(modo)) = sistema.stdout().current_mode() {
        (modo.columns(), modo.rows())
    } else {
        (80, 25)
    };

    let ancho = 54;
    let alto = 14;
    let x = (max_cols.saturating_sub(ancho)) / 2;
    let y = (max_rows.saturating_sub(alto)) / 2;

    let mut paso = 0;
    let mut seleccion = 0;
    let mut necesita_redibujar_fondo = true;
    let mut iniciar_en_lade = false;

    loop {
        if paso > 3 {
            break;
        }

        if necesita_redibujar_fondo {
            let _ = sistema.stdout().set_color(Color::White, Color::Blue);
            let _ = sistema.stdout().clear();

            let _ = sistema.stdout().set_color(Color::Black, Color::Black);
            for i in (y + 1)..=(y + alto) {
                let _ = sistema.stdout().set_cursor_position(x + 2, i);
                for _ in 0..ancho {
                    let _ = write!(sistema.stdout(), " ");
                }
            }

            let _ = sistema.stdout().set_color(Color::Black, Color::LightGray);
            for i in y..(y + alto) {
                let _ = sistema.stdout().set_cursor_position(x, i);
                for _ in 0..ancho {
                    let _ = write!(sistema.stdout(), " ");
                }
            }

            let _ = sistema.stdout().set_cursor_position(x, y);
            let _ = write!(sistema.stdout(), "╔");
            for _ in 0..(ancho - 2) {
                let _ = write!(sistema.stdout(), "═");
            }
            let _ = write!(sistema.stdout(), "╗");
            for i in 1..(alto - 1) {
                let _ = sistema.stdout().set_cursor_position(x, y + i);
                let _ = write!(sistema.stdout(), "║");
                let _ = sistema.stdout().set_cursor_position(x + ancho - 1, y + i);
                let _ = write!(sistema.stdout(), "║");
            }
            let _ = sistema.stdout().set_cursor_position(x, y + alto - 1);
            let _ = write!(sistema.stdout(), "╚");
            for _ in 0..(ancho - 2) {
                let _ = write!(sistema.stdout(), "═");
            }
            let _ = write!(sistema.stdout(), "╝");

            let titulo_texto = "MtrxOS Setup Utility";
            let largo_total = titulo_texto.len() + 4;
            let x_centrada = x + (ancho / 2) - (largo_total / 2);
            let _ = sistema.stdout().set_cursor_position(x_centrada, y);
            let _ = write!(sistema.stdout(), "[ {} ]", titulo_texto);

            necesita_redibujar_fondo = false;
        }

        let _ = sistema.stdout().set_color(Color::Black, Color::LightGray);
        for i in 2..12 {
            let _ = sistema.stdout().set_cursor_position(x + 1, y + i);
            for _ in 0..(ancho - 2) {
                let _ = write!(sistema.stdout(), " ");
            }
        }

        match paso {
            0 => {
                let msg = "Seleccione Nombre de Usuario:";
                let _ = sistema.stdout().set_cursor_position(x + (ancho / 2) - (msg.len() / 2), y + 3);
                let _ = write!(sistema.stdout(), "{}", msg);
                let opciones = ["< USER >", "< IDK >", "< DEFAULT (PC) >"];
                for (i, opt) in opciones.iter().enumerate() {
                    if seleccion == i {
                        let _ = sistema.stdout().set_color(Color::White, Color::Red);
                    } else {
                        let _ = sistema.stdout().set_color(Color::Black, Color::LightGray);
                    }
                    let _ = sistema.stdout().set_cursor_position(x + (ancho / 2) - (opt.len() / 2), y + 6 + i);
                    let _ = write!(sistema.stdout(), "{}", opt);
                }
            }
            1 => {
                let msg = "Aviso de Hardware:";
                let _ = sistema.stdout().set_cursor_position(x + (ancho / 2) - (msg.len() / 2), y + 3);
                let _ = write!(sistema.stdout(), "{}", msg);

                let _ = sistema.stdout().set_color(Color::Black, Color::LightGray);
                let info = "El sistema utiliza disposicion de";
                let info2 = "teclado USA / ENGLISH por defecto.";
                let _ = sistema.stdout().set_cursor_position(x + (ancho / 2) - (info.len() / 2), y + 6);
                let _ = write!(sistema.stdout(), "{}", info);
                let _ = sistema.stdout().set_cursor_position(x + (ancho / 2) - (info2.len() / 2), y + 7);
                let _ = write!(sistema.stdout(), "{}", info2);

                let _ = sistema.stdout().set_color(Color::White, Color::Red);
                let btn = "< Aceptar >";
                let _ = sistema.stdout().set_cursor_position(x + (ancho / 2) - (btn.len() / 2), y + 10);
                let _ = write!(sistema.stdout(), "{}", btn);
            }
            2 => {
                let msg = "Tipo de Sistema Detectado:";
                let _ = sistema.stdout().set_cursor_position(x + (ancho / 2) - (msg.len() / 2), y + 3);
                let _ = write!(sistema.stdout(), "{}", msg);
                let opciones = ["< DESKTOP PC >", "< LAPTOP / PORTABLE >", "< VM / VMPC >"];
                for (i, opt) in opciones.iter().enumerate() {
                    if seleccion == i {
                        let _ = sistema.stdout().set_color(Color::White, Color::Red);
                    } else {
                        let _ = sistema.stdout().set_color(Color::Black, Color::LightGray);
                    }
                    let _ = sistema.stdout().set_cursor_position(x + (ancho / 2) - (opt.len() / 2), y + 6 + i);
                    let _ = write!(sistema.stdout(), "{}", opt);
                }
            }
            3 => {
                let msg = "Seleccione Interfaz de Inicio:";
                let _ = sistema.stdout().set_cursor_position(x + (ancho / 2) - (msg.len() / 2), y + 3);
                let _ = write!(sistema.stdout(), "{}", msg);
                let opciones = ["< LADE (Grafico) >", "< KERNEL SHELL (Texto) >"];
                for (i, opt) in opciones.iter().enumerate() {
                    if seleccion == i {
                        let _ = sistema.stdout().set_color(Color::White, Color::Red);
                    } else {
                        let _ = sistema.stdout().set_color(Color::Black, Color::LightGray);
                    }
                    let _ = sistema.stdout().set_cursor_position(x + (ancho / 2) - (opt.len() / 2), y + 6 + i);
                    let _ = write!(sistema.stdout(), "{}", opt);
                }
            }
            _ => {}
        }

        let mut tecla_presionada = false;
        while !tecla_presionada {
            if let Ok(Some(evento)) = sistema.stdin().read_key() {
                match evento {
                    Key::Printable(tecla) => {
                        if u16::from(tecla) == 13 {
                            if paso == 0 {
                                let nombre = match seleccion {
                                    0 => "user",
                                    1 => "idk",
                                    _ => "pc",
                                };
                                nombre_buf[..nombre.len()].copy_from_slice(nombre.as_bytes());
                                *nombre_l = nombre.len();
                            }
                            if paso == 3 && seleccion == 0 {
                                iniciar_en_lade = true;
                            }
                            paso += 1;
                            seleccion = 0;
                            if paso <= 3 {
                                necesita_redibujar_fondo = true;
                            }
                            tecla_presionada = true;
                        }
                    }
                    Key::Special(uefi::proto::console::text::ScanCode::UP) => {
                        if seleccion > 0 {
                            seleccion -= 1;
                            tecla_presionada = true;
                        }
                    }
                    Key::Special(uefi::proto::console::text::ScanCode::DOWN) => {
                        let max = match paso {
                            0 => 2,
                            1 => 0,
                            2 => 2,
                            3 => 1,
                            _ => 0,
                        };
                        if seleccion < max {
                            seleccion += 1;
                            tecla_presionada = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if iniciar_en_lade {
        crate::lade::iniciar_lade(sistema);
    }
}
