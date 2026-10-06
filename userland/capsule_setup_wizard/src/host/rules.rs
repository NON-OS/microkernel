/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * What the computer-name step takes, key by key: lowercase letters, digits
 * and -, a letter first, at most HOST_MAX. Every start of a name these rules
 * take is one they take too, so Backspace never leaves one they refuse. The
 * last byte is checked on Enter, since the kernel takes no name ending in -.
 */

use nonos_policy_proto::setup_record::HOST_MAX;

#[derive(Clone, Copy)]
pub enum Refused {
    Capital,
    Space,
    Symbol,
    NotLetterFirst,
    Full,
    DashLast,
}

pub fn check(sofar: &[u8], c: u8) -> Result<(), Refused> {
    if sofar.len() >= HOST_MAX {
        return Err(Refused::Full);
    }
    match c {
        b'a'..=b'z' => Ok(()),
        b'A'..=b'Z' => Err(Refused::Capital),
        b' ' => Err(Refused::Space),
        b'0'..=b'9' | b'-' if sofar.is_empty() => Err(Refused::NotLetterFirst),
        b'0'..=b'9' | b'-' => Ok(()),
        _ => Err(Refused::Symbol),
    }
}

impl Refused {
    pub fn text(self) -> &'static [u8] {
        match self {
            Refused::Capital => b"the name is lowercase.",
            Refused::Space => b"no spaces; use - instead.",
            Refused::Symbol => b"only a-z, 0-9 and - are taken.",
            Refused::NotLetterFirst => b"the name starts with a letter.",
            Refused::Full => b"63 characters at most.",
            Refused::DashLast => b"the name ends with a letter or a digit.",
        }
    }
}
