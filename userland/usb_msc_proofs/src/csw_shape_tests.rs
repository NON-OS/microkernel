// NONOS Operating System (AGPL-3.0-or-later)
//! is_csw takes only the 13 bytes of a CSW (BOT 1.0, 5.2, as recalled,
//! unverified). The BOT command hands it exactly 13, so its own length
//! check is shown here: signed bytes one short or one long are no CSW,
//! and fewer than the signature are refused rather than read past.

use crate::bot::is_csw;

#[test]
fn only_thirteen_signed_bytes_are_a_csw() {
    let mut w = b"USBS".to_vec();
    w.resize(13, 0);
    assert!(is_csw(&w));
    assert!(!is_csw(&w[..12]), "one short");
    w.push(0);
    assert!(!is_csw(&w), "one long");
    for n in 0..4 {
        assert!(!is_csw(&b"USBS"[..n]), "{n} bytes");
    }
}
