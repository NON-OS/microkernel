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

//! An endpoint whose Reset Endpoint fails for any reason but its state is
//! not recovered: the failure is reported and nothing more is queued.

use super::command_at::{command_at, endpoint, quiet_context, ring_base};
use super::events::command;
use super::fixture::{Fixture, SLOT, TRB};
use crate::controller::recover_endpoint;
use crate::error::XhciError;

#[test]
fn any_other_reset_failure_is_reported_and_the_ring_left_alone() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let (ring, out) = (fx.transfer_ring(), quiet_context(&fx));
    let base = ring_base(&cmd);
    fx.hc.post(command(base, 5, SLOT));
    let got =
        recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, endpoint(&ring, &out, 1));
    assert_eq!(got, Err(XhciError::CommandCompletionFailed(5)));
    assert_eq!(command_at(base + TRB).get_type(), 0, "no further command was queued");
}
