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

use nonos_libc::mk_yield;

use nonos_policy_proto::IPC_PAYLOAD_MAX;

use super::{recv, serve};

pub fn run(endpoint: u64) -> ! {
    let mut buf = [0u8; IPC_PAYLOAD_MAX];
    let mut sender: u32 = 0;
    loop {
        crate::restore::tick();
        crate::keep::tick();
        let n = recv::poll(endpoint, &mut buf, &mut sender as *mut u32);
        if n <= 0 {
            mk_yield();
            continue;
        }
        serve::serve(sender, &buf[..n as usize]);
    }
}
