use crate::include::*;
pub fn dibujar_popup_bios(sistema: &mut SystemTable<Boot>, titulo: &str, mensaje: &str) {
    let stdout = sistema.stdout();

    let (max_cols, max_rows) = if let Ok(Some(modo)) = stdout.current_mode() {
        (modo.columns(), modo.rows())
    } else {
        (80, 25)
    };

    let _ = stdout.set_color(Color::White, Color::Blue);
    let _ = stdout.clear();

    let ancho = 54;
    let alto = 10;
    let x = (max_cols.saturating_sub(ancho)) / 2;
    let y = (max_rows.saturating_sub(alto)) / 2;

    let _ = stdout.set_color(Color::Black, Color::Black);
    for i in (y + 1)..=(y + alto) {
        let _ = stdout.set_cursor_position(x + 2, i);
        for _ in 0..ancho {
            let _ = write!(stdout, " ");
        }
    }

    let _ = stdout.set_color(Color::Black, Color::LightGray);
    for i in y..(y + alto) {
        let _ = stdout.set_cursor_position(x, i);
        for _ in 0..ancho {
            let _ = write!(stdout, " ");
        }
    }

    let _ = stdout.set_cursor_position(x, y);
    let _ = write!(stdout, "╔");
    for _ in 0..(ancho.saturating_sub(2)) {
        let _ = write!(stdout, "═");
    }
    let _ = write!(stdout, "╗");

    for i in 1..(alto.saturating_sub(1)) {
        let _ = stdout.set_cursor_position(x, y + i);
        let _ = write!(stdout, "║");
        let _ = stdout.set_cursor_position(x + ancho - 1, y + i);
        let _ = write!(stdout, "║");
    }

    let _ = stdout.set_cursor_position(x, y + alto - 1);
    let _ = write!(stdout, "╚");
    for _ in 0..(ancho.saturating_sub(2)) {
        let _ = write!(stdout, "═");
    }
    let _ = write!(stdout, "╝");

    let t_len = titulo.len();
    let t_pos = x + (ancho / 2).saturating_sub((t_len + 4) / 2);
    let _ = stdout.set_cursor_position(t_pos, y);
    let _ = write!(stdout, "[ {} ]", titulo);

    let m_len = mensaje.len();
    let m_pos = x + (ancho / 2).saturating_sub(m_len / 2);
    let _ = stdout.set_cursor_position(m_pos, y + 4);
    let _ = write!(stdout, "{}", mensaje);

    let btn_txt = "< Aceptar >";
    let b_pos = x + (ancho / 2).saturating_sub(btn_txt.len() / 2);
    let _ = stdout.set_color(Color::White, Color::Red);
    let _ = stdout.set_cursor_position(b_pos, y + 7);
    let _ = write!(stdout, "{}", btn_txt);

    let _ = stdout.set_color(Color::Black, Color::LightGray);
}
