//! Proved for every reply up to eight bytes: a hostile or broken RPC
//! reply is read or refused, never a panic.

use super::one;

#[kani::proof]
#[kani::unwind(10)]
fn any_short_reply_is_read_or_refused_without_panicking() {
    let bytes: [u8; 8] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= bytes.len());
    if let Ok(text) = core::str::from_utf8(&bytes[..len]) {
        if let Ok(answer) = one(text) {
            let _ = answer.quantity();
            let _ = answer.word();
            let _ = answer.data();
        }
    }
}
