#![no_std]
#![no_main]

extern crate alloc;

mod chrome;
mod detail;
mod display;
mod dwell;
mod input;
mod interact;
mod logo;
mod look;
mod paint;
mod proto;
mod ready;
mod scene;
mod surface;
mod title;
mod vignette;

use nonos_libc::{heap_init, mk_attest_status, mk_exit, mk_surface_release, AttestStatus};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    mk_exit(run())
}

fn run() -> i32 {
    let comp = match ready::wait_compositor() {
        Some(p) => p,
        None => return 2,
    };
    let (w, h, stride) = match display::query(comp, 2) {
        Ok(d) => d,
        Err(_) => return 4,
    };
    let (base, handle) = match surface::setup(w, h, stride) {
        Some(s) => s,
        None => return 5,
    };
    let mut att = AttestStatus::default();
    let badge = if mk_attest_status(&mut att) == 0 { Some(att.zk_verified == 1) } else { None };
    let look = look::Look { att, badge, logo: logo::fetch::fetch(w, h) };
    paint::splash(base, w, h, stride, &look);
    if scene::submit(comp, 3, handle, w, h).is_err() {
        let _ = mk_surface_release(handle);
        return 6;
    }
    let _ = scene::damage(comp, 4, w, h);
    grabbed_interact(comp, base, w, h, stride, &look);
    let _ = scene::remove(comp, 5);
    let _ = mk_surface_release(handle);
    0
}

fn grabbed_interact(comp: u32, base: u64, w: u32, h: u32, stride: u32, look: &look::Look) {
    let router = match proto::lookup(b"input_router") {
        Some(rp) => rp,
        None => return interact::interact(comp, base, w, h, stride, look),
    };
    let _ = input::subscribe(router, 10);
    let _ = input::grab(router, 11);
    interact::interact(comp, base, w, h, stride, look);
    let _ = input::release(router, 12);
}
