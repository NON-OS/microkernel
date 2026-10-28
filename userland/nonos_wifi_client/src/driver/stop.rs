/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Why a driver stopped short, in words for a screen: the RTL8821CE's
//! firmware step or refused claim step and its errno, or the Intel driver's
//! bring-up step and detail word. A laptop has no serial port, so this line
//! is all a person can send back.

use super::fw_step::firmware_step_text;
use super::intel_step::{hex32, intel_step_text};
use super::stop_words::{claim_step, decimal, errno_word};

/// Where bring-up stopped, read from the status reply.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopWhy {
    /// The firmware failure code (`firmware_step_text`).
    Firmware(u8),
    /// The claim step that was refused (1 the claim, 2 the command register's
    /// Memory Space, 3 the register map) and the errno, as a positive number.
    Claim(u32, u32),
    /// The Intel driver's bring-up step (`intel_step_text`) and detail word.
    Intel(u8, u32),
}

impl StopWhy {
    /// The Intel driver's stop from its status reply past the header (stage,
    /// step, detail word): `None` while it is ready or restarting (stages 0
    /// and 9, the restart's step being its try) or when it names no step.
    pub fn intel(status: &[u8]) -> Option<StopWhy> {
        let (&stage, &step, d) = (status.first()?, status.get(1)?, status.get(2..6)?);
        let detail = u32::from_le_bytes([d[0], d[1], d[2], d[3]]);
        (!matches!(stage, 0 | 9) && step != 0).then_some(StopWhy::Intel(step, detail))
    }

    /// The line, written into `out`: "firmware stopped: <step>", "not
    /// claimed: <step>, <errno word> (-N)" or "stopped at <N>: <step>
    /// (0x<detail>)". `None` for a code this build does not know.
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
            StopWhy::Intel(step, detail) => {
                let (mut dec, mut hex) = ([0u8; 3], [0u8; 8]);
                put(b"stopped at ");
                put(decimal(u32::from(step), &mut dec));
                put(b": ");
                put(intel_step_text(step)?.as_bytes());
                put(b" (0x");
                put(hex32(detail, &mut hex));
                put(b")");
            }
        }
        Some(&out[..n])
    }
}
