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

//! What every FIFO test shares: frames, and the checks each run ends with.

use super::fail::FifoFail;
use super::model::{Model, State};
use super::session::run_command;

/// A frame whose size field says `size_field` and whose body is
/// `payload_len` counting bytes, so a misplaced byte shows in a comparison.
pub fn frame(size_field: u32, payload_len: usize) -> Vec<u8> {
    let mut out = vec![0x80, 0x01];
    out.extend_from_slice(&size_field.to_be_bytes());
    out.extend_from_slice(&0x0000_0144u32.to_be_bytes());
    out.extend((0..payload_len).map(|i| i as u8));
    out
}

/// A well-formed command of `len` bytes.
pub fn command(len: usize) -> Vec<u8> {
    frame(len as u32, len - 10)
}

/// Run to success or a refusal, holding the driver to what a real part
/// accepts whichever way it ends.
pub fn run(model: &mut Model, cmd: &[u8], out: &mut [u8]) -> Result<usize, FifoFail> {
    let result = run_command(model, cmd, out);
    assert!(!model.active, "locality kept after {result:?}");
    assert!(model.violations.is_empty(), "{:?} after {result:?}", model.violations);
    result
}

/// Run a command the model was set up to refuse, and check the refusal
/// left the part neither executing nor held.
pub fn refused(model: &mut Model, cmd: &[u8], out_len: usize) -> FifoFail {
    let mut out = vec![0u8; out_len];
    let fail = run(model, cmd, &mut out).expect_err("the model was set up to fail");
    assert_ne!(model.state, State::Execution, "left executing after {fail:?}");
    fail
}
