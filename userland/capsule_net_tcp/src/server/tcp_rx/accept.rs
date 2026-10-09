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

use crate::state::TABLE;
use crate::tcp::{Endpoint4, State, Tcb};

pub fn syn(local: Endpoint4, remote: Endpoint4, seq: u32, mss: Option<u16>) -> Option<Tcb> {
    let mut table = TABLE.lock();
    if let Some(e) = table.connection_match_mut(local, remote) {
        if e.tcb.state == State::SynReceived {
            let mut resend = e.tcb;
            resend.send.nxt = resend.send.iss;
            return Some(resend);
        }
        return None;
    }
    let (owner, parent) = {
        let l = table.listener_for_mut(local.port)?;
        (l.owner_pid, l.handle)
    };
    table.make_half_open_room(parent);
    let iss = table.iss_for_pair(local, remote);
    let mut tcb = Tcb::listen(local);
    tcb.remote = remote;
    tcb.state = State::SynReceived;
    tcb.recv.irs = seq;
    tcb.recv.nxt = seq.wrapping_add(1);
    tcb.recv.wnd = 8192;
    tcb.send.iss = iss;
    // RFC 9293: SND.UNA = ISS until the peer acknowledges the SYN.
    tcb.send.una = iss;
    tcb.send.nxt = iss;
    tcb.send.wnd = 8192;
    tcb.send.mss = crate::tcp::send_mss(mss);
    let tx_tcb = tcb;
    tcb.send.nxt = iss.wrapping_add(1);
    let handle = table.insert(owner, parent, tcb).ok()?;
    table.arm_half_open(handle, crate::clock::now_ms());
    Some(tx_tcb)
}
