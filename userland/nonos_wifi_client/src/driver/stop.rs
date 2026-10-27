/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Why the RTL8821CE stopped short, in words for a screen: the firmware step
//! that failed, or the claim step the broker refused and its errno. A laptop
//! has no serial port, so this line is all a person can send back.

use super::fw_step::firmware_step_text;
use super::stop_words::{claim_step, decimal, errno_word};

/// Where bring-up stopped, read from the status reply.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopWhy {
    /// The firmware failure code (`firmware_step_text`).
    Firmware(u8),
    /// The claim step that was refused (1 the claim, 2 the command register's
    /// Memory Space, 3 the register map) and the errno, as a positive number.
    Claim(u32, u32),
}

impl StopWhy {
    /// The line, written into `out`: "firmware stopped: <step>", or "not
    /// claimed: <step>, <errno word> (-N)". `None` for a code this build does
    /// not know.
    pub fn text<'a>(&self, out: &'a mut [u8; 64]) -> Option<&'a [u8]> {
        let mut n = 0;
        let mut put = |b: &[u8]| {
            let k = b.len().min(out.len() - n);
            out[n..n + k].copy_from_slice(&b[..k]);
            n += k;
        };
        match *self {
            StopWhy::Firmware(code) => {
                put(b"firmware stopped: ");
                put(firmware_step_text(code)?.as_bytes());
            }
            StopWhy::Claim(step, errno) => {
                put(b"not claimed: ");
                put(claim_step(step)?);
                put(b", ");
                put(errno_word(errno));
                let mut dec = [0u8; 3];
                put(b" (-");
                put(decimal(errno, &mut dec));
                put(b")");
            }
        }
        Some(&out[..n])
    }
}
