#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        core::arch::asm!(
        "ecall",
        options(noreturn, nomem, nostack)
        );
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Fetching args
    let arg_count: u32;
    unsafe {
        core::arch::asm!("mv {}, a0", out(reg) arg_count);
    }
    let args_ptr: *const u32 = 0xA000_0000 as *const u32;
    let args = unsafe { core::slice::from_raw_parts(args_ptr, arg_count as usize) };

    // Code starts here
    let x = args[0];
    let y = args[1];
    let result = x + y;
    // Code ends here

    // Return
    unsafe {
        core::arch::asm!(
        "mv a0, {0}",
        "ecall",
        in(reg) result,
        options(noreturn, nomem, nostack)
        );
    }

}

