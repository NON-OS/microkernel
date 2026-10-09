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
 * What carries frames to a proxy and back, and the clock a stream's waits
 * are measured on. On a machine it is an IPC call to the proxy's service
 * port (`ipc`); on the host it is a proxy written to misbehave on purpose,
 * so the tunnel above it is the code that ships in both.
 */

pub trait Carrier {
    /*
     * Send `frame` and wait at most `wait_ms` for its answer, written into
     * `into`. Returns what the call returned: the answer's length, or a
     * negative number when none came. The length is the callee's word and
     * is checked before anything is read.
     */
    fn call(&mut self, frame: &[u8], wait_ms: u64, into: &mut [u8]) -> i64;

    /* Milliseconds since boot. */
    fn now_ms(&self) -> i64;

    /* Sleep without spinning. */
    fn pause(&mut self, ms: u64);
}
