//! Writing a view key: version, X-Wing seed, `spend_pk`, `nk` for a full key, checksum, padding.

use super::account::Account;
use super::base32::encode;
use super::view_key::{checksum, padded, prefix, push_words, ViewKind, VERSION};
use zeroize::Zeroizing;

/// The view key of `account` as text, wiped when it drops.
pub(crate) fn view_key_text(account: &Account, kind: ViewKind) -> Zeroizing<String> {
    let mut bytes = Zeroizing::new(vec![VERSION]);
    bytes.extend_from_slice(account.receive().x_wing_seed());
    push_words(&mut bytes, &account.spend_pk());
    if kind == ViewKind::Full {
        push_words(&mut bytes, &account.nk());
    }
    let check = checksum(kind, &bytes);
    bytes.extend_from_slice(&check);
    bytes.resize(padded(kind), 0);
    let mut text = Zeroizing::new(String::from(prefix(kind)));
    text.push_str(&Zeroizing::new(encode(&bytes).unwrap_or_default()));
    text
}
