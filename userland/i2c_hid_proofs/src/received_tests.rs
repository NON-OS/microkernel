// NONOS Operating System (AGPL-3.0-or-later)
//! What the run loop hands `parse`: the bytes a receive left, never more
//! than the buffer holds.

use crate::protocol::received;

#[test]
fn a_count_past_the_buffer_is_cut_at_its_end() {
    let rx = [7u8; 24];
    assert_eq!(received(&rx, 24).len(), 24);
    assert_eq!(received(&rx, 25).len(), 24, "one past: no panic, the buffer");
    assert_eq!(received(&rx, i64::MAX).len(), 24);
}

#[test]
fn a_count_within_it_is_kept_and_a_failure_is_nothing() {
    let rx = [7u8; 24];
    assert_eq!(received(&rx, 20).len(), 20);
    assert!(received(&rx, 0).is_empty());
    assert!(received(&rx, -11).is_empty(), "an errno is no bytes");
}
