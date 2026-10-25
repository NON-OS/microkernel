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

//! What the access point sends: data, a replay, a protected management frame.

use nonos_wifi_core::dot11::ccmp::{decrypt, encrypt, CCMP_HDR_LEN};
use nonos_wifi_core::dot11::data::protect;
use nonos_wifi_core::dot11::header::MAC_HEADER_LEN;

use super::*;
use crate::gen3_model::Out;

impl Ap {
    /// Deliver `frame` as the firmware would: protected frames decrypted
    /// (CCMP header kept, MIC gone) when the firmware has the key.
    pub fn deliver(&self, frame: &[u8]) -> Out {
        let protected = frame[1] & 0x40 != 0;
        if !(protected && self.sw.fw_decrypts && self.keyed) {
            return rx(frame, 0);
        }
        let plain = decrypt(frame, &self.tk()).expect("the access point's own frame decrypts");
        let mut f = frame[..MAC_HEADER_LEN + CCMP_HDR_LEN].to_vec();
        f.extend_from_slice(&plain);
        rx(&f, (2 << 8) | (1 << 6))
    }

    /// A protected data frame to the station carrying `snap_body`.
    pub fn protected(&mut self, snap_body: &[u8]) -> Out {
        let seq = self.next_seq();
        let clear = from_ap(snap_body, seq);
        self.pn += 1;
        let f = protect(&clear, self.pn, &self.tk()).unwrap();
        self.last = Some(f.clone());
        self.deliver(&f)
    }

    /// The last protected frame again, as a replay.
    pub fn deliver_replay(&self) -> Out {
        self.deliver(self.last.as_ref().expect("a frame was sent"))
    }

    /// A management frame protected under the pairwise key.
    pub fn protected_mgmt(&mut self, clear: &[u8]) -> Out {
        self.pn += 1;
        let f = encrypt(clear, self.pn, &self.tk()).unwrap();
        self.deliver(&f)
    }
}
