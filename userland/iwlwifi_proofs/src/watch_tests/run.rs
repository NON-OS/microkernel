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

//! A joined link serviced as the time moves on.

use nonos_wifi_core::netif::LinkPort;
use nonos_wifi_core::rsn::JoinPolicy;

use crate::gen3::join::exchange::Progress;
use crate::gen3::join::link::Port;
use crate::gen3::join::run::join;
use crate::join_rig::*;

/// Join, then service the open port as the time moves by `steps`, in ms:
/// the queues the join's authentication went on, what each service sent on
/// which queue, and whether the link is up and went silent.
pub(super) fn after(
    r: &Rig,
    missed: bool,
    steps: &[u64],
) -> (Vec<u16>, Vec<(u16, u8)>, bool, bool) {
    let mut st = Station::new();
    let mut seen = (vec![], vec![], false, false);
    with_fw(r, &mut st, |fw| {
        let beacon = r.ap.borrow().beacon();
        let mut j =
            join(fw, &request(JoinPolicy::ANY), &beacon, 6, 0x3, 0x3, &mut Progress::default())
                .unwrap();
        seen.0 = r.sent().iter().filter(|s| s.frame[0] == 0xB0).map(|s| s.queue).collect();
        let mut port = Port { fw, link: &mut j.link };
        port.service();
        port.fw.inbox.missed = missed;
        for ms in steps {
            port.fw.clock.delays_us += ms * 1000;
            let before = r.sent().len();
            port.service();
            seen.1.extend(r.sent()[before..].iter().map(|s| (s.queue, s.frame[0])));
        }
        (seen.2, seen.3) = (port.link_up(), j.link.silent);
    });
    seen
}
