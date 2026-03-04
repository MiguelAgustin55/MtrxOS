#![no_std]
#![no_main]

pub mod include;
pub use crate::include::*;

#[entry]
fn main(_manejador: Handle, mut sistema: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut sistema).unwrap();
    let _ = sistema.stdout().enable_cursor(false);

    let mut nombre_buffer = [0u8; 32];
    nombre_buffer[0..2].copy_from_slice(b"pc");
    let mut nombre_len = 2;

    let tiempo_inicio = sistema.runtime_services().get_time().unwrap();
    let ticks_inicio = (tiempo_inicio.hour() as u32 * 3600)
    + (tiempo_inicio.minute() as u32 * 60)
    + tiempo_inicio.second() as u32;

    let _ = sistema.stdout().set_color(Color::White, Color::Black);
    let _ = sistema.stdout().clear();

    popup::dibujar_popup_bios(&mut sistema, " MtrxOS ", "Bienvenido a MtrxOS.");

    loop {
        if let Ok(Some(_)) = sistema.stdin().read_key() {
            break;
        }
    }

    popup::dibujar_popup_bios(&mut sistema, " Acerca de ", "Para ver la ayuda escribe 'h'.");

    loop {
        if let Ok(Some(_)) = sistema.stdin().read_key() {
            break;
        }
    }

    let _ = sistema.stdout().set_color(Color::White, Color::Black);
    let _ = sistema.stdout().clear();

    sysconf::configurar_sistema(&mut sistema, &mut nombre_buffer, &mut nombre_len);

    let _ = sistema.stdout().set_color(Color::White, Color::Black);
    let _ = sistema.stdout().clear();

    let _ = sistema.stdout().enable_cursor(true);

    shell::run(&mut sistema, &mut nombre_buffer, &mut nombre_len, ticks_inicio);

    Status::SUCCESS
}
