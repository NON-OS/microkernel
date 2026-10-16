//! What the last `chosen` has to say about the tier it ran, said once.

use core::sync::atomic::Ordering;

use alloc::vec::Vec;

use super::super::default_tier::{Source, WEIGHTS};
use super::super::tiers::{default_line, UNANSWERED, UNKNOWN};
use super::{DEFAULT, NOTE, SOURCE};

/*
 * The lines the last `chosen` left, once: why the choice could not be read,
 * if it could not, and which default runs and why.
 */
pub fn take_note() -> Option<Vec<u8>> {
    let mut out = match NOTE.swap(0, Ordering::Relaxed) {
        1 => UNANSWERED.to_vec(),
        2 => UNKNOWN.to_vec(),
        _ => Vec::new(),
    };
    let at = usize::from(DEFAULT.swap(0, Ordering::Relaxed));
    let source = match SOURCE.load(Ordering::Relaxed) {
        1 => Source::Stick,
        2 => Source::Largest,
        _ => Source::Smallest,
    };
    if let Some(line) = at.checked_sub(1).and_then(|i| default_line(WEIGHTS[i].0, source)) {
        if !out.is_empty() {
            out.push(b'\n');
        }
        out.extend_from_slice(line.as_bytes());
    }
    (!out.is_empty()).then_some(out)
}
