#![no_std]
#![no_main]

extern crate alloc;

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
mod logo;
mod look;
mod paint;
mod proto;
mod ready;
mod say;
mod scene;
mod show;
mod spin;
mod status;
mod surface;
mod title;
mod vignette;
mod warning;

use nonos_libc::{heap_init, mk_attest_status, mk_exit, mk_uptime_ms, AttestStatus};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    mk_exit(run())
}

fn run() -> i32 {
    let Some(comp) = ready::wait_compositor() else { return 2 };
    let Ok(size) = display::query(comp, 2) else { return 4 };
    let mut att = AttestStatus::default();
    let badge = if mk_attest_status(&mut att) == 0 { Some(att.zk_verified == 1) } else { None };
    let (mut up, mut look) = match show::raise(comp, size, att, badge) {
        Ok(u) => u,
        Err(code) => return code,
    };
    let mut f = follow::Follow::new(size, mk_uptime_ms());
    let grab = proto::lookup(b"input_router");
    if let Some(rp) = grab {
        let _ = input::subscribe(rp, 10);
        let _ = input::grab(rp, 11);
    }
    while let Some(next) = interact::interact(comp, up.base, &look, &mut f) {
        match show::raise(comp, next, att, badge) {
            Ok((u, l)) => {
                show::give_back(&up);
                (up, look) = (u, l);
            }
            // Not drawn at the new size: the old drawing stays, as it was.
            Err(_) => f.size = up.size,
        }
    }
    if let Some(rp) = grab {
        let _ = input::release(rp, 12);
    }
    let _ = scene::remove(comp, 5);
    show::give_back(&up);
    0
}
