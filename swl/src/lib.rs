use std::ffi::c_void;

unsafe extern "C" {
    unsafe fn free(ptr: *mut c_void);
}

#[unsafe(no_mangle)]
extern "C" fn swl_power_both_side(a: f64, b: f64) -> f64 {
    a.powf(b) + b.powf(a)
}

#[unsafe(no_mangle)]
extern "C" fn swl_enable_raw_mode() -> isize {
    crossterm::terminal::enable_raw_mode()
        .map(|_| 0)
        .unwrap_or(-1)
}

#[unsafe(no_mangle)]
extern "C" fn swl_disable_raw_mode() -> isize {
    crossterm::terminal::disable_raw_mode()
        .map(|_| 0)
        .unwrap_or(-1)
}

#[unsafe(no_mangle)]
extern "C" fn swl_delete(ptr: isize) {
    let ptr = ptr as *mut c_void;
    unsafe {
        free(ptr);
    }
}
