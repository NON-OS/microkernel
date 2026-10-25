// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The shutdown budget from RTD3E (bytes 91:88), capped, and each CSTS read
//! during the wait named.

use super::{COMPLETE, OCCURRING};
use crate::admin::{
    shutdown_budget_ms, shutdown_step, ControllerIdentity, ShutdownStep, SHUTDOWN_CAP_MS,
    SHUTDOWN_DEFAULT_MS,
};
use crate::error::NvmeError;

#[test]
fn the_budget_is_rtd3e_rounded_up_capped_or_the_default() {
    assert_eq!(shutdown_budget_ms(0), SHUTDOWN_DEFAULT_MS, "no latency named");
    assert_eq!(shutdown_budget_ms(1), 1);
    assert_eq!(shutdown_budget_ms(1_500), 2, "rounded up, never cut short");
    assert_eq!(shutdown_budget_ms(8_000_000), 8_000);
    assert_eq!(shutdown_budget_ms(30_000_000), SHUTDOWN_CAP_MS, "capped");
    assert_eq!(shutdown_budget_ms(u32::MAX), SHUTDOWN_CAP_MS);
}

#[test]
fn each_csts_read_is_named() {
    assert_eq!(shutdown_step(COMPLETE | 1, 0, 10), Ok(ShutdownStep::Complete));
    assert_eq!(shutdown_step(OCCURRING | 1, 9, 10), Ok(ShutdownStep::Wait));
    assert_eq!(shutdown_step(1, 9, 10), Ok(ShutdownStep::Wait), "not yet begun");
    assert_eq!(shutdown_step(OCCURRING, 10, 10), Err(NvmeError::ShutdownTimeout));
    assert_eq!(shutdown_step(0b10, 0, 10), Err(NvmeError::ControllerFatal), "CFS");
    assert_eq!(shutdown_step(COMPLETE | 0b10, 0, 10), Ok(ShutdownStep::Complete));
    assert_eq!(shutdown_step(u32::MAX, 0, 10), Err(NvmeError::UnsupportedController));
}

#[test]
fn rtd3e_is_read_from_bytes_91_to_88() {
    let mut page = [0u8; 4096];
    page[0x58..0x5c].copy_from_slice(&0x0098_9680u32.to_le_bytes());
    assert_eq!(ControllerIdentity::parse(&page).rtd3e_us, 10_000_000);
}
