//! Enough RLP to read a signed type-2 transaction back in a test.

/// The list's items, each as its payload: enough RLP for a signed tx.
pub(super) fn items(raw: &[u8]) -> Vec<Vec<u8>> {
    let len_at = |b: &[u8], n: usize| b[..n].iter().fold(0usize, |a, x| a << 8 | *x as usize);
    let mut at = if raw[1] <= 0xf7 { 2 } else { 2 + (raw[1] - 0xf7) as usize };
    let mut out = Vec::new();
    while at < raw.len() {
        let b = raw[at];
        let (head, len) = match b {
            0..=0x7f => (0, 1),
            0x80..=0xb7 => (1, (b - 0x80) as usize),
            0xb8..=0xbf => (1 + (b - 0xb7) as usize, len_at(&raw[at + 1..], (b - 0xb7) as usize)),
            _ => (1, (b - 0xc0) as usize),
        };
        out.push(raw[at + head..at + head + len].to_vec());
        at += head + len;
    }
    out
}
