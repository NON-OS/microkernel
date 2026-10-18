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

//! An abort the controller ignores. The caller used to see only the
//! timeout, then a timeout on every command after it.

use super::fixture::Fixture;
use crate::controller::run_command;
use crate::error::XhciError;
use crate::trb::commands::noop_command;

/*
 * A controller that keeps CRR set through the CA write cannot be modelled
 * deterministically here: the window is shared memory, so the driver's own
 * write reads back with CRR clear until the model thread puts it back. A
 * ring whose registers were never programmed has no abort to issue, and
 * reaches the same refusal.
 */
#[test]
fn a_command_whose_ring_cannot_be_stopped_is_its_own_error() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let trb = noop_command(cmd.cycle() != 0);
    let got = run_command(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, trb);
    assert_eq!(got.map(|_| ()), Err(XhciError::CommandAbortFailed));
}
