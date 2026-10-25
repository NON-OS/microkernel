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

//! A channel given to its driver: the kernel makes it a record of its own
//! (`mk_hv_channel`), which the driver claims and drives the channel
//! through. The core never opens a channel; from here the channel is the
//! driver's, and the kernel keeps the core from it.

use nonos_libc::mk_hv_channel;

use crate::guid::given;
use crate::hv::Hv;
use crate::msg::Offer;
use crate::say;

/// Give `o` a record when its interface has a driver.
pub fn give(hv: &Hv, o: &mut Offer) {
    if !given(o.interface) || o.record.is_some() {
        return;
    }
    let rc = mk_hv_channel(hv.device_id, o.relid);
    if rc > 0 {
        o.record = Some(rc as u64);
    } else {
        say::not_given(o, rc);
    }
}
