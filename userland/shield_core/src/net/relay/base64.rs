//! Standard base64 with padding, the form the relayer reads its files in.
//! Encoding only: nothing the wallet receives is base64.

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn symbol(index: u32) -> char {
    let i = usize::try_from(index & 0x3f).unwrap_or(0);
    ALPHABET.get(i).map_or('A', |b| char::from(*b))
}

/// `bytes` as base64, three bytes to four symbols, `=` padding the last group.
pub(crate) fn encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3).saturating_mul(4));
    for group in bytes.chunks(3) {
        let b = |i: usize| u32::from(group.get(i).copied().unwrap_or(0));
        let n = (b(0) << 16) | (b(1) << 8) | b(2);
        out.push(symbol(n >> 18));
        out.push(symbol(n >> 12));
        out.push(if group.len() > 1 { symbol(n >> 6) } else { '=' });
        out.push(if group.len() > 2 { symbol(n) } else { '=' });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::encode;

    /// The test vectors of RFC 4648, section 10.
    #[test]
    fn the_rfc_vectors_encode_exactly() {
        for (plain, coded) in [
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg=="),
            ("fooba", "Zm9vYmE="),
            ("foobar", "Zm9vYmFy"),
        ] {
            assert_eq!(encode(plain.as_bytes()), coded);
        }
        assert_eq!(encode(&[0xff, 0xfe, 0xfd]), "//79");
    }
}
