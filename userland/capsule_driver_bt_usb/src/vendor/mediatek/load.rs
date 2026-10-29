/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The patch download, as Linux v6.12 `btmtk_setup_firmware_79xx` runs it,
//! over WMT's patch op. Each section with octets is first asked about:
//! flag 0, then a zero (the legacy download mode) and the section's spec.
//! The answer's flag is its status: 0 not applied, so send it; 1 in
//! progress, the patch semaphore held by another, so ask again in 100 ms,
//! at most 20 times; 2 applied already, so leave it. Its octets then go in
//! blocks of 250, each flagged with its phase: 1 the first, 3 the last,
//! 2 those between (a section of one block is flagged 1). Linux sends
//! after the twentieth "in progress" and takes any other status for "not
//! applied"; this driver stops at both.

use super::fail::Fail;
use super::image::{sections, Section, SPEC};
use super::port::Port;
use super::wmt::{wmt, OP_PATCH};

const BLOCK: usize = 250;
const LOOKS: usize = 20;
const LOOK_MS: u64 = 100;
pub const FIRST: u8 = 1;
pub const MIDDLE: u8 = 2;
pub const LAST: u8 = 3;

/// A block's phase flag: the first, or by the octets left after it.
pub fn phase(first: bool, left_after: usize) -> u8 {
    match (first, left_after) {
        (true, _) => FIRST,
        (false, 0) => LAST,
        _ => MIDDLE,
    }
}

/// The sections sent, and those the chip held already.
pub struct Sent {
    pub sent: usize,
    pub applied: usize,
}

pub fn download(p: &mut impl Port, img: &[u8]) -> Result<Sent, Fail> {
    let mut done = Sent { sent: 0, applied: 0 };
    for s in sections(img).ok_or(Fail::Image)?.iter().filter(|s| s.len > 0) {
        if applied(p, s)? {
            done.applied += 1;
            continue;
        }
        let (mut rest, mut first) = (&img[s.at..s.at + s.len], true);
        while !rest.is_empty() {
            let (block, after) = rest.split_at(rest.len().min(BLOCK));
            wmt(p, OP_PATCH, phase(first, after.len()), block)?;
            (rest, first) = (after, false);
        }
        done.sent += 1;
    }
    p.wait(100); // the firmware's activation
    Ok(done)
}

fn applied(p: &mut impl Port, s: &Section) -> Result<bool, Fail> {
    let mut ask = [0u8; 1 + SPEC];
    ask[1..].copy_from_slice(&s.spec);
    for _ in 0..LOOKS {
        match wmt(p, OP_PATCH, 0, &ask)? {
            0 => return Ok(false),
            1 => p.wait(LOOK_MS),
            2 => return Ok(true),
            _ => return Err(Fail::Status),
        }
    }
    Err(Fail::Busy)
}
