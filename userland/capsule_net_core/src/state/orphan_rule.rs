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

//! Which closed TCP sockets the stack still holds are let go now. A close
//! hands its socket to smoltcp to finish with the peer (orphans.rs), and
//! one whose peer never closes its side waits in FIN-WAIT-2 for good:
//! smoltcp keeps no timer there, and each kept its 80 KiB of buffers. An
//! install's GETs, each on a fresh connection to a keep-alive server, ran
//! net.core out of memory after about 250 (KVM, Telegram's 254 packages).
//! Pure, so the host proofs hold it.

/// Linux's tcp_fin_timeout: how long a closed socket waits for its peer.
pub const LINGER_MS: i64 = 60_000;
/// The most held at once, the oldest let go first past it: 2.5 MiB.
pub const MOST: usize = 32;

/// Whether the orphan at `index` of `count`, oldest first, adopted `age_ms`
/// ago, goes now: its exchange is over (`done`), it has waited LINGER_MS,
/// or it is among the oldest past MOST.
pub fn goes(done: bool, age_ms: i64, index: usize, count: usize) -> bool {
    done || age_ms >= LINGER_MS || index < count.saturating_sub(MOST)
}
