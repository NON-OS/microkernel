// NONOS Operating System (AGPL-3.0-or-later)
//! NameString (section 20.2.2) from its ASL spelling: `\_SB.PCI0`, `^^LNKA`,
//! `FOO`. Segments shorter than four characters are padded with '_', as ASL
//! does.

pub fn name(s: &str) -> Vec<u8> {
    let mut out = Vec::new();
    let mut rest = s;
    while let Some(r) = rest.strip_prefix('\\').or_else(|| rest.strip_prefix('^')) {
        out.push(rest.as_bytes()[0]);
        rest = r;
    }
    let segs: Vec<&str> = if rest.is_empty() { vec![] } else { rest.split('.').collect() };
    match segs.len() {
        0 => out.push(0x00),
        1 => {}
        2 => out.push(0x2E),
        n => out.extend_from_slice(&[0x2F, n as u8]),
    }
    for seg in segs {
        let mut b = [b'_'; 4];
        b[..seg.len()].copy_from_slice(seg.as_bytes());
        out.extend_from_slice(&b);
    }
    out
}
