// NONOS Operating System (AGPL-3.0-or-later)
//! Opcodes and PkgLength (section 20.2.4).

/// An opcode's bytes: one, or ExtOpPrefix and one.
pub fn op(code: u16) -> Vec<u8> {
    if code >> 8 == 0x5B {
        vec![0x5B, code as u8]
    } else {
        vec![code as u8]
    }
}

pub fn cat(parts: &[&[u8]]) -> Vec<u8> {
    parts.concat()
}

/// The shortest PkgLength for a package whose contents after the PkgLength
/// are `body` bytes long. The length counts its own bytes, so the encoding is
/// sized first.
pub fn pkg_len(body: usize) -> Vec<u8> {
    for follow in 0..4usize {
        let total = body + follow + 1;
        let fits = if follow == 0 { total < 0x40 } else { total < 1 << (4 + 8 * follow) };
        if fits {
            if follow == 0 {
                return vec![total as u8];
            }
            let mut v = vec![((follow as u8) << 6) | (total & 0x0F) as u8];
            for i in 0..follow {
                v.push((total >> (4 + 8 * i)) as u8);
            }
            return v;
        }
    }
    panic!("package too large for a PkgLength")
}

/// An opcode, a PkgLength, then the package's contents.
pub fn pkg(code: u16, body: &[u8]) -> Vec<u8> {
    cat(&[&op(code), &pkg_len(body.len()), body])
}
