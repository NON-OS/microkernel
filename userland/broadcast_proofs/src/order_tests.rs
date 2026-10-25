// NONOS Operating System (AGPL-3.0-or-later)
//! The order around the book, read from the kernel's source: the frame
//! written under the book's lock with what the book returned, and the frame
//! off while it changes.

const HAND: &str = include_str!("../../../src/arch/aarch64/timer/broadcast/hand.rs");
const HANDLER: &str = include_str!("../../../src/arch/aarch64/timer/broadcast/handler.rs");
const FRAME: &str = include_str!("../../../src/arch/aarch64/timer/broadcast/frame.rs");

pub(crate) fn code(src: &str) -> String {
    let lines = src.lines().map(|l| l.split("//").next().unwrap_or("").trim());
    lines.filter(|l| !l.is_empty()).collect::<Vec<_>>().join("\n")
}

/// Each mark found after the one before it.
pub(crate) fn order(src: &str, marks: &[&str]) {
    let mut from = 0;
    for mark in marks {
        let at = src[from..].find(mark).unwrap_or_else(|| panic!("no `{mark}` after {from}"));
        from += at + mark.len();
    }
}

#[test]
fn the_frame_holds_what_the_book_returned_under_its_lock() {
    let hand = code(HAND);
    order(
        &hand,
        &["let mut book = BOOK.lock();", "match book.sleep(cpu, due)", "frame.hold(next);"],
    );
    assert!(hand.contains("Err(_) => false,"));
    order(
        &hand,
        &["pub fn take_back", "let mut book = BOOK.lock();", "frame.hold(book.wake(cpu));"],
    );
    let handler = code(HANDLER);
    order(
        &handler,
        &["let mut book = BOOK.lock();", "book.expire(frame.count(), wake);", "frame.hold(next);"],
    );
    assert_eq!(code(HAND).matches("frame.hold(").count(), 2);
    // The lock is held to the end of each, never let go before the write.
    assert!(!hand.contains("drop(") && !handler.contains("drop("));
}

#[test]
fn the_frame_is_off_while_its_compare_changes() {
    let frame = code(FRAME);
    let hold = &frame[frame.find("pub(super) fn hold").unwrap()..];
    order(
        hold,
        &[
            "self.write(CNTP_CTL, 0);",
            "self.write(CNTP_CVAL_LO, at as u32);",
            "self.write(CNTP_CVAL_HI, (at >> 32) as u32);",
            "self.write(CNTP_CTL, CTL_ENABLE);",
        ],
    );
}
