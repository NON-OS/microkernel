// NONOS Operating System (AGPL-3.0-or-later)
//! The saved kernel Context, run: a save returns once, a restore lands back
//! at it with the stack and the callee-saved registers it had.

use crate::kernel::Context;
use std::sync::atomic::{AtomicU32, Ordering};

static RETURNS: AtomicU32 = AtomicU32::new(0);

#[inline(never)]
fn save_then_resume(ctx: &mut Context) -> u32 {
    // SAFETY: `ctx` is a live, aligned Context.
    unsafe { Context::save_to(ctx as *mut Context) };
    let n = RETURNS.fetch_add(1, Ordering::SeqCst) + 1;
    if n == 1 {
        let saved = *ctx;
        // SAFETY: saved just above, on this frame, which is still live: the
        // restore returns into this function a second time.
        unsafe { Context::restore_from(&saved as *const Context) }
    }
    n
}

#[test]
fn a_restore_returns_from_the_save_a_second_time() {
    let mut ctx = Context { s: [0; 12], ra: 0, sp: 0 };
    assert_eq!(save_then_resume(&mut ctx), 2, "came back through the save");
    assert_ne!(ctx.ra, 0, "ra is the save's return address");
    assert_ne!(ctx.sp, 0, "sp is the caller's stack");
}

#[test]
fn every_callee_saved_register_is_stored_in_field_order() {
    let mut ctx = Context { s: [0; 12], ra: 0, sp: 0 };
    let want: [u64; 12] = core::array::from_fn(|i| 0x5a00 + i as u64);
    // SAFETY: s1..s11 are ours to use only if put back: they go to the stack,
    // take the planted values for the call, and come back after it. s0, the
    // frame pointer, is left as it is.
    unsafe {
        core::arch::asm!(
            "addi sp, sp, -96",
            "sd s1, 0(sp)", "sd s2, 8(sp)", "sd s3, 16(sp)", "sd s4, 24(sp)",
            "sd s5, 32(sp)", "sd s6, 40(sp)", "sd s7, 48(sp)", "sd s8, 56(sp)",
            "sd s9, 64(sp)", "sd s10, 72(sp)", "sd s11, 80(sp)",
            "ld s1, 8(a1)", "ld s2, 16(a1)", "ld s3, 24(a1)", "ld s4, 32(a1)",
            "ld s5, 40(a1)", "ld s6, 48(a1)", "ld s7, 56(a1)", "ld s8, 64(a1)",
            "ld s9, 72(a1)", "ld s10, 80(a1)", "ld s11, 88(a1)",
            "mv a0, a2",
            "call {save}",
            "ld s1, 0(sp)", "ld s2, 8(sp)", "ld s3, 16(sp)", "ld s4, 24(sp)",
            "ld s5, 32(sp)", "ld s6, 40(sp)", "ld s7, 48(sp)", "ld s8, 56(sp)",
            "ld s9, 64(sp)", "ld s10, 72(sp)", "ld s11, 80(sp)",
            "addi sp, sp, 96",
            save = sym Context::save_to,
            in("a1") want.as_ptr(),
            in("a2") &mut ctx as *mut Context,
            clobber_abi("C"),
        )
    };
    assert_eq!(&ctx.s[1..], &want[1..], "s1..s11 at their offsets");
}
