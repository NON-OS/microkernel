// NONOS Operating System (AGPL-3.0-or-later)
//! Any UNIT ATTENTION but no medium (3Ah), and a sense that does not read,
//! is a possible change of medium (scan/check.rs). A reader whose transport
//! was reset may say a power-on (29h) where it would have said 28h, and the
//! unfixed driver kept a card of the same size swapped in with the
//! generation unmoved: the kernel's next write landed on it.

use crate::answers::{fails, fails_with, passes_check};
use crate::fixture::class_resets;
use crate::kernel::{generation, write};
use crate::protocol::{E_AGAIN, E_IO};
use crate::scan::Medium;
use crate::serving::read;
use crate::sticks::{bound, held};
use crate::xhci::{setup, Answer};

/// Descriptor-format sense (SPC-4 4.5.2): response code 72h, then the key,
/// ASC and ASCQ in bytes 1 to 3; the driver reads fixed format alone.
fn descriptor(key: u8, asc: u8) -> Vec<u8> {
    let mut s = vec![0u8; 18];
    (s[0], s[1], s[2], s[7]) = (0x72, key, asc, 10);
    s
}

#[test]
fn a_power_on_said_on_a_held_read_raises_the_generation() {
    // Card A held; swapped for B (same size); the reader says 29h, not 28h,
    // on the next READ (or its reset replaced the 28h).
    let mut server = held(1000);
    setup(|d| d.answers = fails(0x06, 0x29).into_iter().chain(passes_check(1000)).collect());
    assert_eq!(read(&mut server), E_IO);
    assert!(matches!(server.scanner.medium, Medium::Ready(_)), "{:?}", server.scanner.report);
    assert_eq!(generation(&mut server), 1, "served afresh, so the kernel can know");
}

#[test]
fn a_card_pulled_mid_transfer_then_a_power_on_raises_the_generation() {
    // Card A held; pulled mid-READ: the data phase comes back as a bad CSW,
    // so the transport is reset. Card B (same size) is in by the check, which
    // hears the reset's 29h only.
    let mut server = held(1000);
    let lost = [Answer::Data(vec![0; 512]), Answer::Data(vec![0; 13])];
    let check = fails(0x06, 0x29).into_iter().chain(passes_check(1000));
    setup(|d| d.answers = lost.into_iter().chain(check).collect());
    assert_eq!(read(&mut server), E_IO);
    assert!(matches!(server.scanner.medium, Medium::Ready(_)), "{:?}", server.scanner.report);
    assert!(class_resets() >= 1, "the transport was reset");
    let g = generation(&mut server);
    assert_eq!(write(&mut server), 0, "the kernel's next write lands on card B");
    assert!(g > 0, "kept, generation {g}, write landed");
}

#[test]
fn a_change_said_in_descriptor_format_is_still_seen() {
    let mut server = held(1000);
    let said = fails_with(descriptor(0x06, 0x28)).into_iter().chain(passes_check(1000));
    setup(|d| d.answers = said.collect());
    assert_eq!(read(&mut server), E_IO);
    assert_eq!(generation(&mut server), 1, "a sense that did not read may hide a change");
    let mut server = bound(1000);
    let said = fails_with(descriptor(0x06, 0x28)).into_iter().chain(passes_check(2000));
    setup(|d| d.answers = said.collect());
    assert_eq!(read(&mut server), E_AGAIN, "nothing held yet: asked again");
    assert!(matches!(server.scanner.medium, Medium::Ready(d) if d.blocks == 2000));
}

#[test]
fn a_power_on_said_on_the_first_read_is_asked_again_and_served_afresh() {
    let mut server = bound(1000);
    setup(|d| d.answers = fails(0x06, 0x29).into_iter().chain(passes_check(1000)).collect());
    assert_eq!(read(&mut server), E_AGAIN, "the kernel's identify asks again, not passes it over");
    assert_eq!(generation(&mut server), 1);
}
