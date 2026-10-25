// NONOS Operating System (AGPL-3.0-or-later)
//! The handler table: one slot per line, claimed once. A banked SGI or PPI is
//! registered by the boot CPU and only checked by a secondary, which must find
//! the boot CPU's own handler there and nothing else.

use crate::gic::irq_handlers::{
    holds_kernel_handler, register, register_for_capsule, unregister, unregister_for_capsule,
    GicIrqError,
};

// Different bodies, so the two are never folded into one address.
fn tick(intid: u32) {
    std::hint::black_box(intid.wrapping_add(1));
}
fn other(intid: u32) {
    std::hint::black_box(intid.wrapping_mul(3));
}

#[test]
fn a_banked_line_is_claimed_once_and_then_found() {
    // The generic timer's physical PPI on virt.
    let ppi = 30;
    assert!(!holds_kernel_handler(ppi, tick), "nothing registered yet");
    assert!(register(ppi, tick).is_ok());
    assert!(holds_kernel_handler(ppi, tick));
    // A second registration, as each secondary used to make, is refused.
    assert!(matches!(register(ppi, tick), Err(GicIrqError::AlreadyClaimed)));
    assert!(holds_kernel_handler(ppi, tick), "the refusal changes nothing");
}

#[test]
fn another_handler_or_owner_is_not_the_one_asked_for() {
    assert!(register(27, tick).is_ok());
    assert!(!holds_kernel_handler(27, other));
    assert!(register_for_capsule(40, tick).is_ok());
    assert!(!holds_kernel_handler(40, tick), "a capsule's line is not the kernel's");
    assert!(!holds_kernel_handler(1020, tick), "a special value has no slot");
}

#[test]
fn a_released_line_is_no_longer_held() {
    assert!(register(28, tick).is_ok());
    assert!(matches!(unregister_for_capsule(28), Err(GicIrqError::NotOwner)));
    assert!(holds_kernel_handler(28, tick));
    assert!(unregister(28).is_ok());
    assert!(!holds_kernel_handler(28, tick));
    assert!(register(28, other).is_ok(), "free again");
    assert!(holds_kernel_handler(28, other));
}
