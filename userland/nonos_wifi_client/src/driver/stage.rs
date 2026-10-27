/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! How far the driver's radio bring-up got, from the status op.

use super::call::{OP_STATUS, WIFI_HDR};
use super::find::Driver;
use super::stage_kind::DriverStage;
use super::stop::StopWhy;

/// The status reply comes back at once; no scan runs.
const STATUS_TIMEOUT_MS: u64 = 500;

/// Where the RTL8821CE's status reply puts the claim step that refused and
/// its errno (words 9 and 10 past the stage byte), and the firmware failure
/// step (after the stage byte and the 52 bytes of counters and registers).
const CLAIM_AT: usize = WIFI_HDR + 1 + 36;
const FW_STEP_AT: usize = WIFI_HDR + 1 + 52;

impl Driver {
    /// Why bring-up stopped at `stage`, for the screen ([`StopWhy`]): the
    /// firmware step after `FirmwareFailed`, the refused claim step after
    /// `NotClaimed`. `None` at any other stage, when the driver did not answer
    /// or answers a shorter status (iwlwifi, or a driver built before it).
    pub fn stop_why(&self, stage: DriverStage) -> Option<StopWhy> {
        let mut resp = [0u8; FW_STEP_AT + 1];
        let n = self.request(OP_STATUS, &[], &mut resp, STATUS_TIMEOUT_MS)?;
        let word =
            |at: usize| u32::from_le_bytes([resp[at], resp[at + 1], resp[at + 2], resp[at + 3]]);
        match stage {
            DriverStage::FirmwareFailed if n > FW_STEP_AT => {
                Some(StopWhy::Firmware(resp[FW_STEP_AT]))
            }
            DriverStage::NotClaimed if n >= CLAIM_AT + 8 && word(CLAIM_AT) != 0 => {
                Some(StopWhy::Claim(word(CLAIM_AT), word(CLAIM_AT + 4)))
            }
            _ => None,
        }
    }

    /// Ask how far bring-up got. `None` when the driver did not answer.
    pub fn stage(&self) -> Option<DriverStage> {
        self.stage_within(STATUS_TIMEOUT_MS)
    }

    /// `stage`, waiting at most `timeout_ms` for the answer: for a caller
    /// whose loop serves others (net_core) and must not wait out a busy driver.
    pub fn stage_within(&self, timeout_ms: u64) -> Option<DriverStage> {
        let mut resp = [0u8; WIFI_HDR + 1];
        match self.request(OP_STATUS, &[], &mut resp, timeout_ms) {
            Some(n) if n > WIFI_HDR => Some(DriverStage::from_code(resp[WIFI_HDR])),
            _ => None,
        }
    }
}
