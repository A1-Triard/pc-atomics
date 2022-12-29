#![deny(warnings)]

#![no_std]

#[cfg(pc-atomics)]
mod atomics {
    #[no_mangle]
    unsafe extern "C" fn __atomic_load(src: *const usize, _model: c_int) -> usize {
        asm!("cli");
        let dest = *src;
        asm!("sti");
        dest
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_store(dest: *mut usize, src: usize, _model: c_int) {
        asm!("cli");
        *dest = src;
        asm!("sti");
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_load_8(src: *const u8, _model: c_int) -> u8 {
        asm!("cli");
        let dest = *src;
        asm!("sti");
        dest
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_store_8(dest: *mut u8, src: u8, _model: c_int) {
        asm!("cli");
        *dest = src;
        asm!("sti");
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_load_16(src: *const u16, _model: c_int) -> u16 {
        asm!("cli");
        let dest = *src;
        asm!("sti");
        dest
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_store_16(dest: *mut u16, src: u16, _model: c_int) {
        asm!("cli");
        *dest = src;
        asm!("sti");
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_load_32(src: *const u32, _model: c_int) -> u32 {
        asm!("cli");
        let dest = *src;
        asm!("sti");
        dest
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_store_32(dest: *mut u32, src: u32, _model: c_int) {
        asm!("cli");
        *dest = src;
        asm!("sti");
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_load_64(src: *const u64, _model: c_int) -> u64 {
        asm!("cli");
        let dest = *src;
        asm!("sti");
        dest
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_store_64(dest: *mut u64, src: u64, _model: c_int) {
        asm!("cli");
        *dest = src;
        asm!("sti");
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_load_128(src: *const u128, _model: c_int) -> u128 {
        asm!("cli");
        let dest = *src;
        asm!("sti");
        dest
    }

    #[no_mangle]
    unsafe extern "C" fn __atomic_store_128(dest: *mut u128, src: u128, _model: c_int) {
        asm!("cli");
        *dest = src;
        asm!("sti");
    }
}
