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

use crate::orchestrator::enumerate::HidEndpoint;
use crate::state::State;
use crate::xhci::interrupt_in;

use super::constants::HID_REPORT_MAX;
use super::feed_report::feed_report;
use super::read_len::read_len;

/// Reports read from one endpoint in one poll at most. A device that always
/// has another ready held the loop on its endpoint: no other device, no
/// request and no key repeat was served while it streamed.
const REPORTS_PER_POLL: u32 = 16;

/// Read the endpoint if it is due at `now_ms` (`Pace`); whether a report
/// came. The buffer is cleared once each report is handed on: it lives as
/// long as the driver, and the keys a keyboard last held stayed in it.
pub(super) fn drain_endpoint(
    state: &mut State,
    ep: &mut HidEndpoint,
    buf: &mut [u8; HID_REPORT_MAX],
    now_ms: u64,
) -> bool {
    if !ep.pace.due(now_ms) {
        return false;
    }
    let mut drained = false;
    let len = read_len(ep.max_packet);
    for _ in 0..REPORTS_PER_POLL {
        let Ok(Some(n)) = interrupt_in(ep.port, ep.slot, ep.dci, len, buf) else { break };
        feed_report(state, ep, buf, n);
        buf.fill(0);
        drained = true;
    }
    ep.pace.asked(now_ms, drained);
    drained
}
