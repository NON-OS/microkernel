#![no_std]
#![no_main]

extern crate alloc;

mod attest_lines;
mod canvas;
mod chrome;
mod detail;
mod display;
mod dma;
mod dwell;
mod follow;
mod input;
mod interact;
mod keys;
mod layout;
mod layout_fit;
mod look;
mod paint;
mod proto;
mod ready;
mod run;
mod say;
mod scene;
mod show;
mod spin;
mod status;
mod surface;
mod title;
mod vignette;
mod warning;

use nonos_libc::{heap_init, mk_exit};

/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    mk_exit(run::run())
}
