#![no_std]
#![no_main]

extern crate alloc;

mod clients;
mod protocol;
mod render;
mod server;
mod setup;
mod state;

use nonos_libc::{heap_init, mk_debug, mk_exit};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let ctx = match setup::run() {
        Ok(ctx) => ctx,
        Err(why) => {
            // Setup ending is what starts the rest of the desktop, so a setup
            // that cannot run says why before the desktop comes up without it.
            say(b"[SETUP] not started: ");
            say(why.as_bytes());
            say(b"; the desktop starts without first-boot setup\n");
            mk_exit(2)
        }
    };
    server::runner::run(ctx)
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
