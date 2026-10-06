//! The account key as the hex another wallet imports. Only the export call reaches it, after
//! the keystore has confirmed the owner, and the string wipes itself when it drops.

use super::EvmAccount;
use zeroize::Zeroizing;

impl EvmAccount {
    /// `0x` and 64 lower case hex digits.
    pub(crate) fn exported(&self) -> Zeroizing<String> {
        let mut out = Zeroizing::new(String::with_capacity(66));
        out.push_str("0x");
        for byte in self.key.iter() {
            for nibble in [byte >> 4, byte & 0x0f] {
                if let Some(digit) = char::from_digit(u32::from(nibble), 16) {
                    out.push(digit);
                }
            }
        }
        out
    }
}
