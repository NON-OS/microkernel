#![no_std]
#![no_main]

extern crate alloc;

mod apps;
mod clients;
mod consent;
mod keep;
mod name;
mod network;
mod protocol;
mod qwen;
mod render;
mod server;
mod setup;
mod state;
mod text;

use nonos_libc::{heap_init, mk_debug, mk_exit};
use nonos_policy_proto::apps::exit_status;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    if let Some(apps_off) = keep::already_done() {
        /*
         * An earlier boot finished setup and kept its answers. Consent is
         * restored here because this boot shows no setup to restore it in.
         */
        let _ = consent::restore();
        keep::wait_for_policy();
        /* On an install boot the installer carries those kept answers. */
        if setup::machine::install_boot() {
            say(b"[SETUP] kept from an earlier boot; opening the installer\n");
            mk_exit(exit_status(render::screens::mode::EXIT_INSTALLER, apps_off));
        }
        say(b"[SETUP] kept from an earlier boot; starting the desktop\n");
        mk_exit(exit_status(0, apps_off));
    }
    let ctx = match setup::run() {
        Ok(ctx) => ctx,
        Err(why) => {
            /*
             * Setup ending is what starts the rest of the desktop, so a setup
             * that cannot run says why before the desktop comes up without it.
             */
            say(b"[SETUP] not started: ");
            say(why.as_bytes());
            say(b"; the desktop starts without first-boot setup\n");
            mk_exit(2)
        }
    };
    let mut ctx = ctx;
    server::restore_poll::poll(&mut ctx);
    server::runner::run(ctx)
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
