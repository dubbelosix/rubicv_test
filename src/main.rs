#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let x = 42;
    let y = 27;
    let result = x + y;
    
    unsafe {
        core::arch::asm!("mv a0, {}", in(reg) result);
        core::arch::asm!("ebreak");
    }
    
    loop {}
}
