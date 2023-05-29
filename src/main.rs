#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // vga_buffer::print_test();
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello, Umbra!").unwrap();
    let _ = write!(vga_buffer::WRITER.lock(), "");

    loop {}
}


