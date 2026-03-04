use crate::include::*;
pub fn mostrar_intro(sistema: &mut SystemTable<Boot>) {
    let _ = sistema.stdout().enable_cursor(false);
    let _ = sistema.stdout().set_color(Color::White, Color::Black);
    let _ = sistema.stdout().clear();

    let (max_cols, max_rows) = if let Ok(Some(modo)) = sistema.stdout().current_mode() {
        (modo.columns(), modo.rows())
    } else {
        (80, 25)
    };

    let logo = [
        "             ██             ",
        "            ████            ",
        "           ██  ██           ",
        "          ████████          ",
        "         ██      ██         ",
        "        ██        ██        ",
    ];

    let col_inicio = (max_cols / 2).saturating_sub(14);
    let fila_inicio = (max_rows / 2).saturating_sub(3);

    let _ = sistema.stdout().set_color(Color::LightBlue, Color::Black);
    for (i, linea) in logo.iter().enumerate() {
        let _ = sistema.stdout().set_cursor_position(col_inicio, fila_inicio + i);
        let _ = write!(sistema.stdout(), "{}", linea);
    }

    sistema.boot_services().stall(5_000_000);

    while let Ok(Some(_)) = sistema.stdin().read_key() {}

    let _ = sistema.stdout().set_color(Color::White, Color::Black);
    let _ = sistema.stdout().enable_cursor(true);
    let _ = sistema.stdout().clear();
}
