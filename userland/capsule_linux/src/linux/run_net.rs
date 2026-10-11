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

//! The run role starts without Network, as init cannot read a mask
//!. Once the program's policy is bound, a run whose box has
//! NET_ANON asks the kernel to start it again as app.linux.run.net, which
//! holds Network, and ends here; that role binds the policy again and runs
//! only a program whose box has NET_ANON. A run the kernel will not start
//! again, a terminal's among them, goes on without the network.

use nonos_libc::{mk_cap_check, mk_exit, mk_getpid, mk_linux_run_net};

use super::app::{net_step, NetStep};
use super::say::say;
use super::start_say::routine;

/// Capability::Network, bit 2 of the kernel's capability mask.
const NETWORK: u64 = 1 << 2;

pub(super) fn settle(mask: u64) -> Result<(), &'static [u8]> {
    let me = mk_getpid();
    match net_step(mask, mk_cap_check(me, NETWORK)) {
        NetStep::Run => Ok(()),
        NetStep::Refuse => {
            Err(&b"[LINUX] the network role runs only a program granted NET_ANON\n"[..])
        }
        NetStep::AskAgain if mk_linux_run_net(me) == 0 => {
            routine(b"[LINUX] starting again with the network\n");
            super::file::clear_private();
            mk_exit(0)
        }
        NetStep::AskAgain => {
            say(b"[LINUX] its policy grants NET_ANON; this run has no network\n");
            Ok(())
        }
    }
}
