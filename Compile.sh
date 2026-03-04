cargo clean && cargo build --release && cp target/x86_64-unknown-uefi/release/valenos.efi iso_root/EFI/BOOT/BOOTX64.EFI && grub-mkrescue -o mtrxos.iso iso_root
