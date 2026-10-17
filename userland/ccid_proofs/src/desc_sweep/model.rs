// NONOS Operating System (AGPL-3.0-or-later)
//! What a reader's configuration says, stated apart from reader.rs (CCID
//! 1.1, 5.1, as recalled, unverified): the first interface of class 0Bh,
//! alternate 0, with a bulk IN and OUT among its endpoints; its first
//! type-21h descriptor of 54 bytes or more before the next interface;
//! dwFeatures (byte 40) APDU-level (bit 17 or 18); dwMaxCCIDMessageLength
//! (byte 44) cut to one 4096-byte transfer; bMaxSlotIndex (byte 4).

pub type Found = Option<Result<(u8, u8, u32, usize), &'static str>>;

pub fn rule(raw: &[u8]) -> Found {
    let (mut i, mut scope, mut bulk, mut d) = (0, None, [false; 2], None::<&[u8]>);
    while i + 2 <= raw.len() && raw[i] >= 2 && i + raw[i] as usize <= raw.len() {
        let x = &raw[i..i + raw[i] as usize];
        match (x[1], scope) {
            (0x04, None) if x.len() >= 9 && x[5] == 0x0B && x[3] == 0 => scope = Some(x[2]),
            (0x04, Some(_)) => break,
            (0x05, Some(_)) if x.len() >= 7 && x[3] & 3 == 2 => bulk[(x[2] >> 7) as usize] = true,
            (0x21, Some(_)) if x.len() >= 54 && d.is_none() => d = Some(x),
            _ => {}
        }
        i += raw[i] as usize;
    }
    let word = |d: &[u8], at: usize| u32::from_le_bytes(d[at..at + 4].try_into().unwrap());
    Some(match (scope?, bulk, d) {
        (_, [false, _] | [_, false], _) => Err("no bulk pair"),
        (_, _, None) => Err("no CCID descriptor"),
        (_, _, Some(d)) if word(d, 40) & 0x6_0000 == 0 => Err("TPDU or character level reader"),
        (n, _, Some(d)) => Ok((n, d[4], word(d, 40), (word(d, 44) as usize).min(4096))),
    })
}
