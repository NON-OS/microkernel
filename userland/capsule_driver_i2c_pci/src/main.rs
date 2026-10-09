#![no_std]
#![no_main]

extern crate alloc;

use nonos_libc::{heap_init, mk_exit};

mod constants;
mod discover;
mod driver;
mod init;
mod protocol;
mod regs;
mod server;
mod setup;
mod transaction;

const DRIVER: &[u8] = b"driver.i2c_pci";

/// Without an LPSS I2C controller the driver says so and leaves
/// (`EXIT_ABSENT`) before claiming anything. With controllers, the pass over
/// them is retried on the shared bounded schedule while none comes up, each
/// pass giving back what it does not keep, and running out is `EXIT_GAVE_UP`.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let _ = heap_init();
    let mut cands = [discover::Found::default(); discover::MAX_CONTROLLERS];
    let n = discover::find_controllers(&mut cands);
    let found = (n > 0).then_some(&cands[..n]);
    if found.is_none() {
        setup::say_no_controller();
    }
    let started = nonos_libc::bringup::decide(
        found,
        || nonos_libc::say_absent(DRIVER),
        |cands| server::bring_up(DRIVER, || setup::run(cands)),
    );
    match started {
        Ok(driver) => server::run(driver),
        Err(code) => mk_exit(code),
    }
}
