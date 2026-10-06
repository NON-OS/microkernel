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

//! Pointing tpm2-tools at the software TPM, finding it ports, and taking it
//! down again.

use std::net::TcpListener;

use super::dir::Dir;
use super::swtpm::Swtpm;

impl Swtpm {
    /// The TCTI string tpm2-tools takes for this instance.
    pub fn tcti(&self) -> String {
        format!("swtpm:host=127.0.0.1,port={}", self.port)
    }

    /// The machine restarting: the same TPM state, its PCRs cleared.
    pub fn reboot(mut self, test: &str) -> Swtpm {
        let dir = std::mem::replace(&mut self.dir, Dir::new());
        drop(self);
        Swtpm::start_in(dir, test).expect("swtpm again")
    }
}

/// A free port whose neighbour is free too, for the control port.
pub(super) fn free_pair() -> u16 {
    loop {
        let a = TcpListener::bind(("127.0.0.1", 0)).expect("bind a free port");
        let port = a.local_addr().expect("bound address").port();
        if port < u16::MAX && TcpListener::bind(("127.0.0.1", port + 1)).is_ok() {
            return port;
        }
    }
}

impl Drop for Swtpm {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
