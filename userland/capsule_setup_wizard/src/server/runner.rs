use alloc::vec;

use nonos_libc::{mk_exit, mk_ipc_recv_from, INPUT_KIND_KEY_DOWN};

use crate::clients::{compositor, input_router};
use crate::protocol::{parse_delivery, DELIVERY_LEN};
use crate::render::screens;
use crate::state::Context;

use super::step::{self, DONE};

pub fn run(mut ctx: Context) -> ! {
    // Said on the console: a setup that never gets the keyboard looks like one
    // that is waiting for a person.
    let heard = input_router::subscribe(ctx.router_port, 1).is_ok();
    let held = input_router::grab_keyboard(ctx.router_port, 2).is_ok();
    let line: &[u8] = match (heard, held) {
        (true, true) => b"[SETUP] keyboard held\n",
        (true, false) => b"[SETUP] subscribed, but the keyboard grab was refused\n",
        _ => b"[SETUP] the input router refused the subscription\n",
    };
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    redraw(&ctx);
    let mut rx = vec![0u8; DELIVERY_LEN.max(64)];
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), 0, &mut sender);
        if n <= 0 {
            continue;
        }
        let Some(ev) = parse_delivery(&rx[..n as usize]) else {
            continue;
        };
        if ev.kind != INPUT_KIND_KEY_DOWN {
            continue;
        }
        let outcome = screens::on_key(&mut ctx, ev.code);
        ctx.step = step::apply(ctx.step, outcome);
        if ctx.step >= DONE {
            let _ = compositor::push_scene_remove(ctx.compositor_port, 3);
            mk_exit(0);
        }
        redraw(&ctx);
    }
}

fn redraw(ctx: &Context) {
    screens::draw(ctx);
    let _ = compositor::damage_commit(ctx.compositor_port, 9, ctx.width, ctx.height);
}
