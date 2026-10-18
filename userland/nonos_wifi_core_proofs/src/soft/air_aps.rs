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

//! Who hears what in the simulated air. A frame reaches each AP tuned to,
//! when addressed to it or to everyone. What comes back is heard at the
//! signal of the AP that sent it. And with nothing waiting, each AP tuned
//! to beacons every 100 ms.

use crate::soft::ap::SimAp;
use crate::soft::rig::SimAir;
use nonos_wifi_core::regdom::Band;

fn on(ap: &SimAp, tuned: Option<(Band, u8)>) -> bool {
    ap.on && tuned == Some((Band::of_legacy(ap.channel), ap.channel))
}

impl SimAir {
    pub(crate) fn on_air(&mut self, mpdu: &[u8]) -> Vec<Vec<u8>> {
        let to = mpdu.get(4..10).unwrap_or(&[]);
        let tuned = self.tuned;
        let mut replies = Vec::new();
        for ap in core::iter::once(&mut self.ap).chain(self.ap2.as_mut()) {
            if on(ap, tuned) && (to == ap.bssid || to == [0xff; 6]) {
                replies.extend(ap.on_frame(mpdu));
            }
        }
        replies
    }

    pub(crate) fn next_frame(&mut self) -> Option<(Vec<u8>, i8)> {
        let now = self.clock.get();
        let second = self.ap2.as_ref().map(|a| a.bssid);
        if let Some(f) = self.inbox.pop_front() {
            let from_second = second.is_some_and(|b| f.get(10..16) == Some(&b[..]));
            return Some((f, self.rssi[usize::from(from_second)]));
        }
        let sta = self.chat_to.unwrap_or(self.addr.0);
        if on(&self.ap, self.tuned) && now >= self.beacon_at {
            self.beacon_at = now + 100;
            self.inbox.extend(self.ap.chat(sta));
            return Some((self.ap.beacon_out(), self.rssi[0]));
        }
        let tuned = self.tuned;
        let ap2 = self.ap2.as_mut().filter(|a| on(a, tuned) && now >= self.beacon_at2)?;
        let b = ap2.beacon_out();
        self.inbox.extend(ap2.chat(sta));
        self.beacon_at2 = now + 100;
        Some((b, self.rssi[1]))
    }
}
