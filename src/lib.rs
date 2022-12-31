#![deny(warnings)]

#![no_std]

#[cfg(pc_atomics)]
mod atomics {
    use core::arch::asm;
    use core::ffi::c_int;

    #[no_mangle]
    unsafe extern "C" fn __atomic_load(src: *const usize, _model: c_int) -> usize {
        asm!("cli");
        let dest = *src;
        asm!("sti", "nop");
        dest
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_store(dest: *mut usize, src: usize, _model: c_int) {
        asm!("cli");
        *dest = src;
        asm!("sti", "nop");
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_load_1(src: *const u8, _model: c_int) -> u8 {
        asm!("cli");
        let dest = *src;
        asm!("sti", "nop");
        dest
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_store_1(dest: *mut u8, src: u8, _model: c_int) {
        asm!("cli");
        *dest = src;
        asm!("sti", "nop");
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_load_2(src: *const u16, _model: c_int) -> u16 {
        asm!("cli");
        let dest = *src;
        asm!("sti", "nop");
        dest
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_store_2(dest: *mut u16, src: u16, _model: c_int) {
        asm!("cli");
        *dest = src;
        asm!("sti", "nop");
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_load_4(src: *const u32, _model: c_int) -> u32 {
        asm!("cli");
        let dest = *src;
        asm!("sti", "nop");
        dest
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_store_4(dest: *mut u32, src: u32, _model: c_int) {
        asm!("cli");
        *dest = src;
        asm!("sti", "nop");
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_load_8(src: *const u64, _model: c_int) -> u64 {
        asm!("cli");
        let dest = *src;
        asm!("sti", "nop");
        dest
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_store_8(dest: *mut u64, src: u64, _model: c_int) {
        asm!("cli");
        *dest = src;
        asm!("sti", "nop");
    }
}
