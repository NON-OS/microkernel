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

/*
 * Every wait a stream through a proxy makes, and every buffer it keeps, has
 * a bound, so a proxy that stops answering or a far end that never stops
 * sending costs a known amount and then an error with a reason.
 */

/*
 * How long a frame with nothing to carry waits for its answer. net.socks5
 * holds such a poll while its exit is quiet (a second today, and 40 ms
 * once it gathers replies), net.anon answers at once.
 */
pub const POLL_WAIT_MS: u64 = 2_000;

/* How long a frame carrying bytes waits: they cross into the network first. */
pub const SEND_WAIT_MS: u64 = 15_000;

/*
 * Times one frame is sent before the proxy counts as silent. A frame whose
 * answer did not come is sent again, unchanged, and the proxy gives back
 * the answer it kept for that number instead of carrying the bytes twice.
 */
pub const ASKS: usize = 3;

/* The pause before a frame is sent again. */
pub const ASK_GAP_MS: u64 = 50;

/* The pause between polls that brought nothing. */
pub const POLL_GAP_MS: u64 = 25;

/*
 * How long a stream may take to open: the greeting, the CONNECT, net.anon
 * waiting for its exit to connect, and the retries while a network that is
 * running has not connected yet.
 */
pub const OPEN_MS: i64 = 45_000;

/* The pause before asking a network that has not connected yet again. */
pub const NOT_YET_GAP_MS: u64 = 1_000;

/* How long the reset that ends a stream waits for the proxy to answer it. */
pub const RESET_WAIT_MS: u64 = 2_000;

/*
 * The most the far end's bytes may pile up unread. A reader takes them as
 * they come; one that does not has stopped, and holding more for it only
 * takes memory the capsule needs.
 */
pub const PENDING_MAX: usize = 1024 * 1024;

/*
 * A caller that works in slices waits only a slice for each answer and asks
 * again later under the same number. It leaves this long between asks, so
 * the calls it stopped waiting for do not pile up at the proxy: the kernel
 * holds a place for each until the proxy answers it, and refuses a caller
 * that already holds eight.
 */
pub const REASK_GAP_MS: i64 = 200;
