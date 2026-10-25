use core::mem::MaybeUninit;
use core::ptr;

// The pattern zeroize.rs fills its secrets with, for the same reason: all zeros would pass
// against a wipe that never happened.
const PATTERN: u8 = 0xA5;

// The control: a seed shaped type with no wipe, put through the same steps, must still hold
// the pattern. If it reads zeros, the tests in zeroize.rs prove nothing.
struct Unwiped([u8; 64]);

#[test]
fn the_technique_would_notice_a_missing_wipe() {
    let mut slot = MaybeUninit::new(Unwiped([PATTERN; 64]));
    let plain = slot.as_mut_ptr();

    // SAFETY: the slot holds the value just written, not yet dropped.
    let before = unsafe { &*plain };
    assert_eq!(before.0, [PATTERN; 64], "the control starts out holding the pattern");

    // SAFETY: the storage is this test's, and it holds an initialised value.
    unsafe { ptr::drop_in_place(plain) };

    // SAFETY: still this test's storage, holding 64 plain bytes.
    let seen = unsafe { ptr::read(plain.cast::<[u8; 64]>()) };
    assert_eq!(
        seen, [PATTERN; 64],
        "reading storage after a drop returns zeros by itself, so the wipe tests prove nothing"
    );
}
