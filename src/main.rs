#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        core::arch::asm!(
        "mv a1, {0}",
        "ecall",
        in(reg) 1,
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
    let args = unsafe { core::slice::from_raw_parts(0x9000_0000 as *const u32, arg_count as usize) };

    let n = args[0];

    let mut sum : u32 = 0;

    for i in 0..n {
        sum += i;
        // Prevent optimization with a volatile write
        unsafe { core::ptr::write_volatile(&mut sum, sum); }
    }

    // unsafe {
    //     core::arch::asm!(
    //     "mv t0, zero",      // sum = 0
    //     "mv t1, zero",      // i = 0
    //     "2:",               // main loop start
    //     "add t0, t0, t1",   // sum += i
    //     "addi t1, t1, 1",
    //     "add t0, t0, t1",   // sum += (i+1)
    //     "addi t1, t1, 1",
    //     "add t0, t0, t1",   // sum += (i+2)
    //     "addi t1, t1, 1",
    //     "add t0, t0, t1",   // sum += (i+3)
    //     "addi t1, t1, 1",
    //     "add t0, t0, t1",   // sum += (i+4)
    //     "addi t1, t1, 1",
    //     "add t0, t0, t1",   // sum += (i+5)
    //     "addi t1, t1, 1",
    //     "add t0, t0, t1",   // sum += (i+6)
    //     "addi t1, t1, 1",
    //     "add t0, t0, t1",   // sum += (i+7)
    //     "addi t1, t1, 1",
    //     "sub t2, {n}, t1",  // check if we can do 8 more
    //     "bgtz t2, 2b",      // if yes, continue main loop
    //     "3:",               // cleanup loop for remaining iterations
    //     "beq t1, {n}, 4f",  // if done, exit
    //     "add t0, t0, t1",   // sum += i
    //     "addi t1, t1, 1",
    //     "j 3b",             // continue cleanup
    //     "4:",               // done
    //     "mv {sum}, t0",     // store result
    //     n = in(reg) n,
    //     sum = out(reg) sum,
    //     );
    // }

    unsafe {
        let scratch_ptr = 0x00002000 as *mut u32;
        *scratch_ptr = sum;
    }

    // Code ends here

    // Return
    unsafe {
        core::arch::asm!(
        "mv a1, {0}",
        "ecall",
        in(reg) 0,
        options(noreturn, nomem, nostack)
        );
    }

}

