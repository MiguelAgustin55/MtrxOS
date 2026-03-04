use crate::include::*;
pub fn run(
    sistema: &mut SystemTable<Boot>,
    nombre_buffer: &mut [u8; 32],
    nombre_len: &mut usize,
    ticks_inicio: u32,
) {
    let mut es_root = false;
    let mut tabla_alias_corto: [[u8; 8]; 5] = [[0u8; 8]; 5];
    let mut tabla_alias_largo: [[u8; 16]; 5] = [[0u8; 16]; 5];
    let mut num_alias = 0;
    let mut microsegundos_frame = 40_000;

    loop {
        let nombre_actual = core::str::from_utf8(&nombre_buffer[..*nombre_len]).unwrap_or("pc");

        if es_root {
            write!(sistema.stdout(), "[root@mtrx-os /]# ").unwrap();
        } else {
            write!(sistema.stdout(), "[{}@mtrx-os ~]$ ", nombre_actual).unwrap();
        }

        let mut buffer = [0u8; 128];
        let mut indice = 0;

        loop {        
            if let Ok(Some(evento)) = sistema.stdin().read_key() {
                match evento {
                    Key::Printable(tecla) => {
                        let valor = u16::from(tecla);
                        if valor == 13 {
                            writeln!(sistema.stdout(), "").unwrap();
                            break; 
                        } else if valor == 8 {
                            if indice > 0 {
                                indice -= 1;
                                write!(sistema.stdout(), "\x08 \x08").unwrap();
                            }
                        } else if valor >= 32 && valor <= 126 {
                            if indice < buffer.len() {
                                buffer[indice] = valor as u8;
                                indice += 1;
                                write!(sistema.stdout(), "{}", valor as u8 as char).unwrap();
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let entrada_cruda = core::str::from_utf8(&buffer[..indice]).unwrap_or("");
        let entrada = entrada_cruda.trim();
        if entrada.is_empty() { continue; }

        for sub_entrada in entrada.split("&&") {
            let sub_trim = sub_entrada.trim();
            if sub_trim.is_empty() { continue; }

            let mut partes = sub_trim.splitn(2, ' ');
            let mut comando = partes.next().unwrap_or("");
            let argumentos = partes.next().unwrap_or("");

            for i in 0..num_alias {
                let len_c = tabla_alias_corto[i].iter().position(|&x| x == 0).unwrap_or(8);
                let corto_guardado = core::str::from_utf8(&tabla_alias_corto[i][..len_c]).unwrap_or("");
                
                if comando == corto_guardado {
                    let len_l = tabla_alias_largo[i].iter().position(|&x| x == 0).unwrap_or(16);
                    comando = core::str::from_utf8(&tabla_alias_largo[i][..len_l]).unwrap_or("");
                    break;
                }
            }

            if comando == "m" { comando = "matrix"; }
            if comando == "f" { comando = "fetch"; }
            if comando == "c" { comando = "clear"; }
            if comando == "h" { comando = "help"; }

            match comando {
                "intro" => {
                    crate::intro::mostrar_intro(sistema);
                }
                "gltest" => {
                    if let Some(mut gl) = crate::mtrx_gl::MtrxGl::init(sistema) {
                        crate::gltest::run(sistema, &mut gl);
                    } else {
                        writeln!(sistema.stdout(), "Error: No se pudo inicializar MtrxGL.").unwrap();
                    }
                }
                "gdt" => {
                    if !es_root {
                        let _ = writeln!(sistema.stdout(), "Null");
                    } else {
                        unsafe {
                            let mut gdt_ptr = [0u8; 10];
                            core::arch::asm!("sgdt [{}]", in(reg) &mut gdt_ptr);
                            
                            let limite = u16::from_le_bytes([gdt_ptr[0], gdt_ptr[1]]);
                            let base = u64::from_le_bytes([
                                gdt_ptr[2], gdt_ptr[3], gdt_ptr[4], gdt_ptr[5],
                                gdt_ptr[6], gdt_ptr[7], gdt_ptr[8], gdt_ptr[9]
                            ]);
                
                            let _ = writeln!(sistema.stdout(), "--- GLOBAL DESCRIPTOR TABLE (GDT) ---");
                            let _ = writeln!(sistema.stdout(), "Direccion Base: 0x{:016X}", base);
                            let _ = writeln!(sistema.stdout(), "Limite: {} bytes", limite);
                            let _ = writeln!(sistema.stdout(), "Registros de Segmento: CS=0x08, DS=0x10 (Estimado)");
                            let _ = writeln!(sistema.stdout(), "-------------------------------------");
                        }
                    }
                }
                "smbios" => {
                    if !es_root {
                        let _ = writeln!(sistema.stdout(), "Null");
                    } else {
                        let mut direccion: Option<usize> = None;
                        for table in sistema.config_table() {
                            if table.guid == uefi::table::cfg::SMBIOS_GUID || table.guid == uefi::table::cfg::SMBIOS3_GUID {
                                direccion = Some(table.address as usize);
                                break;
                            }
                        }
                        
                        match direccion {
                            Some(addr) => {
                                let stdout = sistema.stdout();
                                let _ = writeln!(stdout, "--- TABLA SMBIOS DETECTADA ---");
                                let _ = writeln!(stdout, "Direccion Fisica: 0x{:X}", addr);
                                let _ = writeln!(stdout, "Estado: Mapeada en el espacio de configuracion.");
                            }
                            None => {
                                let _ = writeln!(sistema.stdout(), "Error: No se encontro SMBIOS.");
                            }
                        }
                    }
                }
                "msr" => {
                    if !es_root {
                        let _ = writeln!(sistema.stdout(), "Null");
                    } else {
                        unsafe {
                            let low: u32;
                            let high: u32;
                            let msr_addr: u32 = 0xC0000080;
                            core::arch::asm!("rdmsr", in("ecx") msr_addr, out("eax") low, out("edx") high);
                
                            let _ = writeln!(sistema.stdout(), "--- IA32_EFER (Extended Feature Enable) ---");
                            let _ = writeln!(sistema.stdout(), "Valor Hex: 0x{:08X}{:08X}", high, low);
                            if (low & (1 << 8)) != 0 { let _ = writeln!(sistema.stdout(), "- LME (Long Mode Enable): SI"); }
                            if (low & (1 << 11)) != 0 { let _ = writeln!(sistema.stdout(), "- NXE (No-Execute Bit): SI"); }
                        }
                    }
                }
                "cregs" => {
                    if !es_root {
                        let _ = writeln!(sistema.stdout(), "Null");
                    } else {
                        unsafe {
                            let cr0: u64;
                            let cr3: u64;
                            let cr4: u64;
                            core::arch::asm!("mov {}, cr0", out(reg) cr0);
                            core::arch::asm!("mov {}, cr3", out(reg) cr3);
                            core::arch::asm!("mov {}, cr4", out(reg) cr4);
                
                            let _ = writeln!(sistema.stdout(), "--- REGISTROS DE CONTROL ---");
                            let _ = writeln!(sistema.stdout(), "CR0: 0x{:016X} (System Flags)", cr0);
                            let _ = writeln!(sistema.stdout(), "CR3: 0x{:016X} (Page Directory Base)", cr3);
                            let _ = writeln!(sistema.stdout(), "CR4: 0x{:016X} (Arch Extensions)", cr4);
                            
                            if (cr0 & 1) != 0 { let _ = writeln!(sistema.stdout(), "- Modo Protegido: ACTIVO"); }
                            if (cr0 & 0x80000000) != 0 { let _ = writeln!(sistema.stdout(), "- Paginacion: ACTIVA"); }
                        }
                    }
                }
                "memmap" => {
                    if !es_root {
                        let _ = writeln!(sistema.stdout(), "Null");
                    } else {
                        let mut mmap_buf = [0u8; 16384];
                        if let Ok(mmap) = sistema.boot_services().memory_map(&mut mmap_buf) {
                            let _ = writeln!(sistema.stdout(), "TIPO             | PAGINAS    | DIRECCION INICIAL");
                            let _ = writeln!(sistema.stdout(), "--------------------------------------------------");
                            for entry in mmap.entries().take(10) {
                                let _ = writeln!(sistema.stdout(), "{:X?} \t | {:10} | 0x{:012X}", 
                                    entry.ty, entry.page_count, entry.phys_start);
                            }
                            let _ = writeln!(sistema.stdout(), "... (mostrando primeras 10 regiones)");
                        }
                    }
                }
                "cpuinfo" => {
                    let r = unsafe { core::arch::x86_64::__cpuid(1) };
                    let family = (r.eax >> 8) & 0xF;
                    let model = (r.eax >> 4) & 0xF;
                    
                    let _ = writeln!(sistema.stdout(), "--- DETALLES DEL PROCESADOR ---");
                    let _ = writeln!(sistema.stdout(), "Fabricante: {}", obtener_nombre_cpu());
                    let _ = writeln!(sistema.stdout(), "Familia: {:X} | Modelo: {:X}", family, model);
                    
                    let _ = write!(sistema.stdout(), "Features: ");
                    if (r.ecx & (1 << 31)) != 0 { let _ = write!(sistema.stdout(), "Hypervisor "); }
                    if (r.edx & (1 << 25)) != 0 { let _ = write!(sistema.stdout(), "SSE "); }
                    if (r.edx & (1 << 26)) != 0 { let _ = write!(sistema.stdout(), "SSE2 "); }
                    if (r.ecx & (1 << 0)) != 0 { let _ = write!(sistema.stdout(), "SSE3 "); }
                    
                    let _ = writeln!(sistema.stdout(), "\n-------------------------------");
                }
                "peek" => {
                    if !es_root {
                        let _ = writeln!(sistema.stdout(), "Null");
                    } else {
                        let addr_str = argumentos.trim();
                        if addr_str.is_empty() {
                            let _ = writeln!(sistema.stdout(), "Uso: peek [direccion_hex]");
                        } else {
                            let addr = if addr_str.starts_with("0x") {
                                usize::from_str_radix(&addr_str[2..], 16).unwrap_or(0)
                            } else {
                                addr_str.parse::<usize>().unwrap_or(0)
                            };
                
                            let ptr = addr as *const u8;
                            let _ = writeln!(sistema.stdout(), "Leyendo memoria en: 0x{:X}", addr);
                            let _ = writeln!(sistema.stdout(), "-----------------------------------");
                            
                            for i in 0..4 {
                                let _ = write!(sistema.stdout(), "0x{:X}: ", addr + (i * 4));
                                for j in 0..4 {
                                    unsafe {
                                        let val = *ptr.add(i * 4 + j);
                                        let _ = write!(sistema.stdout(), "{:02X} ", val);
                                    }
                                }
                                let _ = writeln!(sistema.stdout(), "");
                            }
                            let _ = writeln!(sistema.stdout(), "-----------------------------------");
                        }
                    }
                }
                "uptime" => {
                    if let Ok(ahora) = sistema.runtime_services().get_time() {
                        let ticks_ahora = (ahora.hour() as u32 * 3600) + (ahora.minute() as u32 * 60) + ahora.second() as u32;
                        
                        let diff = if ticks_ahora >= ticks_inicio { ticks_ahora - ticks_inicio } else { 0 };
                        
                        let h = diff / 3600;
                        let m = (diff % 3600) / 60;
                        let s = diff % 60;
                
                        let _ = writeln!(sistema.stdout(), "MtrxOS lleva encendido: {:02}:{:02}:{:02}", h, m, s);
                    } else {
                        let _ = writeln!(sistema.stdout(), "Error: No se puede leer el reloj del hardware.");
                    }
                }
                "lspci" => {
                    if !es_root {
                        let _ = writeln!(sistema.stdout(), "Null");
                    } else {
                        let _ = writeln!(sistema.stdout(), "--- ESCANEANDO BUS PCI (MtrxOS) ---");
                        let _ = writeln!(sistema.stdout(), "BUS | DEV | FUN | VENDOR | DEVICE");
                        let _ = writeln!(sistema.stdout(), "---------------------------------");
                        
                        for bus in 0..8 {
                            for dev in 0..32 {
                                let address = (bus << 16) | (dev << 11) | (1 << 31);
                                
                                unsafe {
                                    core::arch::asm!("out dx, eax", in("dx") 0xCF8u16, in("eax") address);
                                    let id: u32;
                                    core::arch::asm!("in eax, dx", out("eax") id, in("dx") 0xCFCu16);
                
                                    if id != 0xFFFFFFFF {
                                        let vendor = (id & 0xFFFF) as u16;
                                        let device = (id >> 16) as u16;
                                        let _ = writeln!(sistema.stdout(), "{:02x}  | {:02x}  | 00  | {:04x}   | {:04x}", bus, dev, vendor, device);
                                    }
                                }
                            }
                        }
                        let _ = writeln!(sistema.stdout(), "---------------------------------");
                    }
                }
                "tskm" => {
                    if !es_root {
                        let _ = writeln!(sistema.stdout(), "Error: Solo root puede ver el monitor de hardware.");
                    } else {
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
                
                        let (max_cols, max_rows) = if let Ok(Some(modo)) = sistema.stdout().current_mode() {
                            (modo.columns(), modo.rows())
                        } else {
                            (80, 25)
                        };
                
                        let stdout = sistema.stdout();
                        let _ = writeln!(stdout, "--- Monitor de Sistema Real ---");
                        let _ = write!(stdout, "CPU Ciclos: [");
                        for i in 0..10 {
                            if i < cpu_load as usize { let _ = write!(stdout, "|"); }
                            else { let _ = write!(stdout, "."); }
                        }
                        let _ = writeln!(stdout, "] {}0%", cpu_load);
                        let _ = writeln!(stdout, "RAM Detectada: {}.{} GB", mem_gb_int, mem_gb_frac);
                        let _ = writeln!(stdout, "RAM en MB: {} MB", mem_mb);
                        let _ = writeln!(stdout, "Resolucion: {}x{}", max_cols, max_rows);
                        let _ = writeln!(stdout, "-------------------------------");
                    }
                }
                "panic" => {
                    if !es_root {
                        let _ = writeln!(sistema.stdout(), "Null");
                    } else {
                        ejecutar_panic(sistema, argumentos);
                    }
                }
                "beep" => {
                    let mut partes = argumentos.split_whitespace();
                    let freq = partes.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(750);
                    let ms = partes.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(200);

                    emitir_beep(freq);
                    sistema.boot_services().stall((ms * 1000) as usize);
                    silenciar();
                }
                "lade" => {
                    lade::iniciar_lade(sistema);
                }
                "reslist" => {
                    writeln!(sistema.stdout(), "Modos de texto soportados:").unwrap();
                    
                    let num_modos = sistema.stdout().modes().count();
                    
                    for i in 0..num_modos {
                        if let Some(modo) = sistema.stdout().modes().nth(i) {
                            writeln!(
                                sistema.stdout(), 
                                "- {}x{} (Indice: {})", 
                                modo.columns(), 
                                modo.rows(), 
                                modo.index()
                            ).unwrap();
                        }
                    }
                }
                "res" => {
                    if !es_root {
                        writeln!(sistema.stdout(), "Error: Solo root puede cambiar la resolucion global.").unwrap();
                    } else {
                        let mut args = argumentos.splitn(2, 'x');
                        let arg_w = args.next().unwrap_or("");
                        let arg_h = args.next().unwrap_or("");
                
                        if arg_w.is_empty() || arg_h.is_empty() {
                            writeln!(sistema.stdout(), "Uso: res [ancho]x[alto] (Ej: res 1280x720)").unwrap();
                        } else {
                            let target_w = arg_w.parse::<usize>().unwrap_or(0);
                            let target_h = arg_h.parse::<usize>().unwrap_or(0);
                
                            let mut modo_encontrado = None;
                
                            for modo in sistema.stdout().modes() {
                                if modo.columns() == target_w && modo.rows() == target_h {
                                    modo_encontrado = Some(modo);
                                    break;
                                }
                            }
                
                            if let Some(m) = modo_encontrado {
                                if sistema.stdout().set_mode(m).is_ok() {
                                    sistema.stdout().clear().unwrap();
                                    writeln!(sistema.stdout(), "Resolucion establecida: {}x{}", target_w, target_h).unwrap();
                                } else {
                                    writeln!(sistema.stdout(), "Error critico al cambiar el modo.").unwrap();
                                }
                            } else {
                                writeln!(sistema.stdout(), "Error: La resolucion {}x{} no es soportada por este monitor.", target_w, target_h).unwrap();
                                writeln!(sistema.stdout(), "Tip: Prueba con resoluciones estandar (80x25, 100x31).").unwrap();
                            }
                        }
                    }
                }
                "app" => {
                    match argumentos {
                        "zim" => zim::iniciar_ide(sistema),
                        "" => writeln!(sistema.stdout(), "Uso: app [zim,]").unwrap(),
                        _ => writeln!(sistema.stdout(), "La app '{}' no esta instalada.", argumentos).unwrap(),
                    }
                }
                "ascii" => {
                    if !es_root {
                        let _ = writeln!(sistema.stdout(), "Error: Solo root puede ejecutar el volcado de tabla ASCII.");
                    } else {
                        for i in 32..127 {
                            let _ = write!(sistema.stdout(), "{}:{} ", i, i as u8 as char);
                            if i % 8 == 0 { 
                                let _ = writeln!(sistema.stdout(), ""); 
                            }
                        }
                        let _ = writeln!(sistema.stdout(), "");
                    }
                }
                "game" => {
                    match argumentos {
                        "buggy" => buggy::iniciar_juego(sistema),
                        "raycaster" => raycaster::iniciar_juego(sistema),
                        "" => writeln!(sistema.stdout(), "Uso: game [buggy, raycaster]").unwrap(),
                        _ => writeln!(sistema.stdout(), "El juego '{}' no esta instalado.", argumentos).unwrap(),
                    }
                }
                "wait" => {
                    let segundos = argumentos.parse::<u64>().unwrap_or(0);
                    if segundos > 0 {
                        writeln!(sistema.stdout(), "Esperando {} segundos...", segundos).unwrap();
                        sistema.boot_services().stall((segundos * 1_000_000) as usize);
                    } else {
                        writeln!(sistema.stdout(), "Uso: wait [segundos]").unwrap();
                    }
                }
                "alias" => {
                    let mut partes = argumentos.splitn(2, ':');
                    let corto_raw = partes.next().unwrap_or("").trim();
                    let largo_raw = partes.next().unwrap_or("").trim();
                
                    if corto_raw.is_empty() || largo_raw.is_empty() {
                        writeln!(sistema.stdout(), "Uso: alias [corto] : [comando]").unwrap();
                    } else if num_alias < 5 {
                        tabla_alias_corto[num_alias] = [0u8; 8];
                        tabla_alias_largo[num_alias] = [0u8; 16];

                        let b_corto = corto_raw.as_bytes();
                        let len_c = b_corto.len().min(8);
                        tabla_alias_corto[num_alias][..len_c].copy_from_slice(&b_corto[..len_c]);
                        
                        let b_largo = largo_raw.as_bytes();
                        let len_l = b_largo.len().min(16);
                        tabla_alias_largo[num_alias][..len_l].copy_from_slice(&b_largo[..len_l]);
                        
                        num_alias += 1;
                        writeln!(sistema.stdout(), "Alias OK: {} -> {}", corto_raw, largo_raw).unwrap();
                    } else {
                        writeln!(sistema.stdout(), "Error: Tabla llena.").unwrap();
                    }
                }
                "calc" => {
                    let mut partes = argumentos.split_whitespace();
                    let n1_str = partes.next().unwrap_or("");
                    let op = partes.next().unwrap_or("");
                    let n2_str = partes.next().unwrap_or("");

                    if n1_str.is_empty() || op.is_empty() || n2_str.is_empty() {
                        writeln!(sistema.stdout(), "Uso: calc [num1] [op] [num2] (Ej: calc 10 + 5)").unwrap();
                    } else {
                        let n1 = n1_str.parse::<i64>().unwrap_or(0);
                        let n2 = n2_str.parse::<i64>().unwrap_or(0);

                        match op {
                            "+" => writeln!(sistema.stdout(), "Resultado: {}", n1 + n2).unwrap(),
                            "-" => writeln!(sistema.stdout(), "Resultado: {}", n1 - n2).unwrap(),
                            "*" => writeln!(sistema.stdout(), "Resultado: {}", n1 * n2).unwrap(),
                            "/" => {
                                if n2 != 0 {
                                    writeln!(sistema.stdout(), "Resultado: {}", n1 / n2).unwrap();
                                } else {
                                    writeln!(sistema.stdout(), "Error: Division por cero.").unwrap();
                                }
                            }
                            _ => writeln!(sistema.stdout(), "Operador no soportado: {}", op).unwrap(),
                        }
                    }
                }
                "matrix" => {
                    sistema.stdout().set_color(uefi::proto::console::text::Color::LightGreen, uefi::proto::console::text::Color::Black).unwrap();
                    for _ in 0..1000 {
                        let semilla = sistema.runtime_services().get_time().unwrap().nanosecond();
                        let char_bit = if semilla % 2 == 0 { "0 " } else { "1 " };
                        
                        write!(sistema.stdout(), "{}", char_bit).unwrap();
                        
                        sistema.boot_services().stall(2_000); 
                    }
                    writeln!(sistema.stdout(), "").unwrap();
                    sistema.stdout().set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black).unwrap();
                }
                "date" => {
                    if let Ok(tiempo) = sistema.runtime_services().get_time() {
                        writeln!(sistema.stdout(), "Fecha: {:02}/{:02}/{}", tiempo.day(), tiempo.month(), tiempo.year()).unwrap();
                        writeln!(sistema.stdout(), "Hora:  {:02}:{:02}:{:02}", tiempo.hour(), tiempo.minute(), tiempo.second()).unwrap();
                    } else {
                        writeln!(sistema.stdout(), "Error al obtener la hora del hardware.").unwrap();
                    }
                }
                "tfps" => {
                    if !es_root {
                        writeln!(sistema.stdout(), "Solo el usuario root puede cambiar el tps del sistema.").unwrap();
                    } else {
                        match argumentos {
                            "10" => { microsegundos_frame = 100_000; writeln!(sistema.stdout(), "FPS: 10").unwrap(); }
                            "15" => { microsegundos_frame = 66_666;  writeln!(sistema.stdout(), "FPS: 15").unwrap(); }
                            "20" => { microsegundos_frame = 50_000;  writeln!(sistema.stdout(), "FPS: 20").unwrap(); }
                            "25" => { microsegundos_frame = 40_000;  writeln!(sistema.stdout(), "FPS: 25").unwrap(); }
                            "30" => { microsegundos_frame = 33_333;  writeln!(sistema.stdout(), "FPS: 30").unwrap(); }
                            _ => writeln!(sistema.stdout(), "Uso: tfps [10, 15, 20, 25, 30]").unwrap(),
                        }
                    }
                }
                "color" => {
                    match argumentos {
                        "def" => {
                            sistema.stdout().set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black).unwrap();
                        }
                        "0a" => {
                            sistema.stdout().set_color(uefi::proto::console::text::Color::LightGreen, uefi::proto::console::text::Color::Black).unwrap();
                        }
                        "0b" => {
                            sistema.stdout().set_color(uefi::proto::console::text::Color::LightCyan, uefi::proto::console::text::Color::Black).unwrap();
                        }
                        "0c" => {
                            sistema.stdout().set_color(uefi::proto::console::text::Color::LightRed, uefi::proto::console::text::Color::Black).unwrap();
                        }
                        _ => writeln!(sistema.stdout(), "Uso: color [0a, 0b, 0c, def]").unwrap(),
                    }
                }
                "echo" => {
                    writeln!(sistema.stdout(), "{}", argumentos).unwrap();
                }
                "shutdown" => {
                    if es_root {
                        writeln!(sistema.stdout(), "Apagando el hardware...").unwrap();
                        sistema.runtime_services().reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
                    } else {
                        writeln!(sistema.stdout(), "Solo el usuario root puede apagar el sistema.").unwrap();
                    }
                }
                "reboot" => {
                    if es_root {
                        writeln!(sistema.stdout(), "Reiniciando el sistema...").unwrap();
                        sistema.runtime_services().reset(ResetType::COLD, Status::SUCCESS, None);
                    } else {
                        writeln!(sistema.stdout(), "Solo el usuario root puede reiniciar el sistema.").unwrap();
                    }
                }
                "whoami" => {
                    let nombre_actual = core::str::from_utf8(&nombre_buffer[..*nombre_len]).unwrap_or("pc");
                    if es_root {
                        writeln!(sistema.stdout(), "root").unwrap();
                    } else {
                        writeln!(sistema.stdout(), "{}", nombre_actual).unwrap();
                    }
                }
                "help" => {
                    writeln!(sistema.stdout(), "------ Available MtrxOS commands ------").unwrap();
                    writeln!(sistema.stdout(), "help         : Muestra esta lista de comandos").unwrap();
                    writeln!(sistema.stdout(), "clear        : Limpia la pantalla").unwrap();
                    writeln!(sistema.stdout(), "fetch        : Muestra info del sistema y logo").unwrap();
                    writeln!(sistema.stdout(), "echo [txt]   : Imprime un texto en pantalla").unwrap();

                    loop {
                        if let Ok(Some(_)) = sistema.stdin().read_key() {
                            break;
                        }
                        sistema.boot_services().stall(10_000);
                    }

                    writeln!(sistema.stdout(), "color [cod]  : Cambia color (0a, 0b, 0c, def)").unwrap();
                    writeln!(sistema.stdout(), "res [WxH]    : Cambia a una res especifica (Ej: res 100x31)").unwrap();
                    writeln!(sistema.stdout(), "tfps [num]   : Ajusta los FPS (10, 15, 20, 25, 30)").unwrap();
                    writeln!(sistema.stdout(), "whoami       : Muestra el usuario actual").unwrap();
                    writeln!(sistema.stdout(), "date         : Muestra la fecha y hora real del hardware").unwrap();
                    
                    loop {
                        if let Ok(Some(_)) = sistema.stdin().read_key() {
                            break;
                        }
                        sistema.boot_services().stall(10_000);
                    }

                    writeln!(sistema.stdout(), "matrix       : Ejecuta el protector de pantalla digital").unwrap();
                    writeln!(sistema.stdout(), "sudo         : Cambia a modo superusuario (root)").unwrap();
                    writeln!(sistema.stdout(), "uname -u [n] : Cambia tu nombre de usuario").unwrap();
                    writeln!(sistema.stdout(), "clcache      : Elimina la cache").unwrap();
                    writeln!(sistema.stdout(), "optz         : Optimiza el sistema").unwrap();

                    loop {
                        if let Ok(Some(_)) = sistema.stdin().read_key() {
                            break;
                        }
                        sistema.boot_services().stall(10_000);
                    }

                    writeln!(sistema.stdout(), "calc [n1] [op] [n2] : Realiza operaciones matematicas (+ - * /)").unwrap();
                    writeln!(sistema.stdout(), "wait [seg]          : Pausa la ejecucion por N segundos").unwrap();
                    writeln!(sistema.stdout(), "alias [a] [c]       : Crea un atajo para un comando").unwrap();
                    writeln!(sistema.stdout(), "game [g]            : Ejecuta un juego (Ej: game buggy)").unwrap();
                    writeln!(sistema.stdout(), "ascii               : Muestra la tabla de caracteres ASCII").unwrap();

                    loop {
                        if let Ok(Some(_)) = sistema.stdin().read_key() {
                            break;
                        }
                        sistema.boot_services().stall(10_000);
                    }

                    writeln!(sistema.stdout(), "app [a]      : Ejecuta una app (Ej: app zim)").unwrap();
                    writeln!(sistema.stdout(), "reslist      : Lista resoluciones soportadas").unwrap();
                    writeln!(sistema.stdout(), "lade         : Inicia el entorno de escritorio LADE").unwrap();
                    writeln!(sistema.stdout(), "beep [f] [m] : Emite un sonido (Hz) por N milisegundos").unwrap();
                    writeln!(sistema.stdout(), "tskm         : Monitor de Sistema").unwrap();

                    loop {
                        if let Ok(Some(_)) = sistema.stdin().read_key() {
                            break;
                        }
                        sistema.boot_services().stall(10_000);
                    }

                    writeln!(sistema.stdout(), "exit         : Sale de root o del shell").unwrap();
                    writeln!(sistema.stdout(), "reboot       : Reinicia la computadora").unwrap();
                    writeln!(sistema.stdout(), "shutdown     : Apaga la computadora").unwrap();
                    writeln!(sistema.stdout(), "---------------------------------------").unwrap();
                    writeln!(sistema.stdout(), "Tip: Puedes usar '&&' para encadenar comandos.").unwrap();
                    writeln!(sistema.stdout(), "---------------------------------------").unwrap();
                }
                "optz" => {
                    writeln!(sistema.stdout(), "Analizando y Optimizando bloques de memoria RAM...").unwrap();
                    
                    let pasos = 50;
                    for i in 0..=pasos {
                        let porcentaje = (i * 100) / pasos;
                        
                        write!(sistema.stdout(), "\rProgreso: [").unwrap();
                        
                        for _ in 0..i {
                            write!(sistema.stdout(), "#").unwrap();
                        }
                        
                        for _ in i..pasos {
                            write!(sistema.stdout(), ".").unwrap();
                        }
                        
                        write!(sistema.stdout(), "] {}% ", porcentaje).unwrap();
                
                        sistema.boot_services().stall(15_000);
                    }
                    
                    writeln!(sistema.stdout(), "").unwrap();

                    writeln!(sistema.stdout(), "Optimizacion completada exitosamente.").unwrap();
                }
                "clcache" => {
                    writeln!(sistema.stdout(), "Cache del sistema eliminada correctamente.").unwrap();
                }
                "fetch" => {
                    let cpu = obtener_nombre_cpu();
                    let nombre_actual = core::str::from_utf8(&nombre_buffer[..*nombre_len]).unwrap_or("pc");
                    let usuario = if es_root { "root@mtrx-os" } else { nombre_actual };

                    if let Ok(ahora) = sistema.runtime_services().get_time() {
                        let ticks_ahora = (ahora.hour() as u32 * 3600) + (ahora.minute() as u32 * 60) + ahora.second() as u32;
                        let diff = if ticks_ahora >= ticks_inicio { ticks_ahora - ticks_inicio } else { 0 };
                        
                        let h = diff / 3600;
                        let m = (diff % 3600) / 60;
                        let s = diff % 60;

                        writeln!(sistema.stdout(), "------------------").unwrap();
                        writeln!(sistema.stdout(), "{}@mtrx-os", usuario).unwrap();
                        writeln!(sistema.stdout(), "------------------").unwrap();
                        writeln!(sistema.stdout(), "OS: MtrxOS 1.0 (Release)").unwrap();
                        writeln!(sistema.stdout(), "Kernel: UEFI MTRX Kernel").unwrap();
                        writeln!(sistema.stdout(), "Kernel V: 0.0.1-rust").unwrap();
                        writeln!(sistema.stdout(), "RevT: 2026.02.27-12.40").unwrap();
                        writeln!(sistema.stdout(), "Arquitectura: x86_64").unwrap();
                        writeln!(sistema.stdout(), "Tipo: Live OS").unwrap();
                        writeln!(sistema.stdout(), "Shell: MtrxShell").unwrap();
                        writeln!(sistema.stdout(), "GL: MtrxGL").unwrap();
                        writeln!(sistema.stdout(), "CPU: {}", cpu).unwrap();
                        writeln!(sistema.stdout(), "Uptime: {:02}:{:02}:{:02}", h, m, s).unwrap();
                    }
                }
                "sudo" => {
                    if !es_root {
                        es_root = true;
                        writeln!(sistema.stdout(), "Privilegios elevados a root.").unwrap();
                    } else {
                        writeln!(sistema.stdout(), "Ya eres usuario root.").unwrap();
                    }
                }
                "clear" => {
                    sistema.stdout().clear().unwrap();
                }
                "exit" => {
                    if es_root {
                        es_root = false;
                        writeln!(sistema.stdout(), "Ya no eres usuario root.").unwrap();
                    } else {
                        writeln!(sistema.stdout(), "No puedes salir del kernel de MtrxOS.").unwrap();
                    }
                }
                "uname" => {
                    let mut args = argumentos.splitn(2, ' ');
                    let bandera = args.next().unwrap_or("");
                    let nuevo = args.next().unwrap_or("");
                
                    if bandera == "-u" && !nuevo.is_empty() {
                        *nombre_buffer = [0u8; 32];
                        let bytes = nuevo.as_bytes();
                        let len = bytes.len().min(32);
                        nombre_buffer[..len].copy_from_slice(&bytes[..len]);
                        *nombre_len = len;
                        writeln!(sistema.stdout(), "Nombre actualizado.").unwrap();
                    } else {
                        writeln!(sistema.stdout(), "Uso: uname -u [nombre]").unwrap();
                    }
                }
                _ => {
                    writeln!(sistema.stdout(), "Comando no encontrado: {}", comando).unwrap();
                }
            }
        }
        sistema.boot_services().stall(microsegundos_frame);
    }
}

fn obtener_nombre_cpu() -> &'static str {
    let res = unsafe { core::arch::x86_64::__cpuid(0) };
    match res.ebx {
        0x756e6547 => "Intel Processor",
        0x68747541 => "AMD Processor",
        _ => "Generic x86_64 CPU",
    }
}

fn emitir_beep(frecuencia: u32) {
    if frecuencia == 0 { return; }
    let divisor = 1193180 / frecuencia;
    unsafe {
        asm!("out 0x43, al", in("al") 0xb6u8);
        asm!("out 0x42, al", in("al") (divisor & 0xff) as u8);
        asm!("out 0x42, al", in("al") ((divisor >> 8) & 0xff) as u8);
        let mut tmp: u8;
        asm!("in al, 0x61", out("al") tmp);
        asm!("out 0x61, al", in("al") tmp | 0x03);
    }
}

fn silenciar() {
    unsafe {
        let mut tmp: u8;
        asm!("in al, 0x61", out("al") tmp);
        asm!("out 0x61, al", in("al") tmp & 0xfc);
    }
}

fn ejecutar_panic(sistema: &mut SystemTable<Boot>, motivo: &str) {
    let _ = sistema.stdout().enable_cursor(false);
    let tiempo = sistema.runtime_services().get_time().ok();
    let stdout = sistema.stdout();
    
    let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Red);
    let _ = stdout.clear();

    let logo = [
        "       _       ",
        "      / \\      ",
        "     /   \\     ",
        "    /     \\    ",
        "   /  /\\   \\   ",
        "  /  /  \\   \\  ",
        " /__/    \\___\\  ",
    ];

    for (i, linea) in logo.iter().enumerate() {
        let _ = stdout.set_cursor_position(2, 1 + i);
        let _ = write!(stdout, "{}", linea);
    }

    let _ = stdout.set_cursor_position(20, 2);
    let _ = write!(stdout, "MtrxOS KERNEL PANIC :( ");
    
    let _ = stdout.set_cursor_position(20, 4);
    let _ = write!(stdout, "Se ha detectado una excepcion no controlada.");
    
    let _ = stdout.set_cursor_position(20, 5);
    let _ = write!(stdout, "El sistema se ha detenido para proteger los datos.");

    let _ = stdout.set_cursor_position(2, 10);
    let _ = write!(stdout, "DETALLES TECNICOS:");
    let _ = stdout.set_cursor_position(2, 11);
    let _ = write!(stdout, "--------------------------------------------------");
    
    let _ = stdout.set_cursor_position(2, 12);
    let _ = write!(stdout, "ERROR_CODE: {}", if motivo.is_empty() { "CRITICAL_SERVICE_FAILED" } else { motivo });

    if let Some(t) = tiempo {
        let _ = stdout.set_cursor_position(2, 13);
        let _ = write!(stdout, "TIME_STAMP: {:02}:{:02}:{:02} | DATE: {:02}/{:02}/{}", 
            t.hour(), t.minute(), t.second(), t.day(), t.month(), t.year());
    }

    let _ = stdout.set_cursor_position(2, 15);
    let _ = write!(stdout, "MEM_DUMP: 0x0000000000000F02h - 0xFFFFFFFFFFFFFFFFh");

    let _ = stdout.set_cursor_position(2, 17);
    let _ = write!(stdout, "--------------------------------------------------");
    
    let _ = stdout.set_cursor_position(2, 19);
    let _ = write!(stdout, "Presione cualquier tecla para reiniciar...");

    loop {
        if let Ok(Some(_)) = sistema.stdin().read_key() {
            sistema.runtime_services().reset(
                uefi::table::runtime::ResetType::COLD, 
                uefi::Status::ABORTED, 
                None
            );
        }
    }
}
