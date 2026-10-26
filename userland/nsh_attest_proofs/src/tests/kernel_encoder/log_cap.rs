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

//! The kernel puts a log in its document up to the cap the client reads
//! (nsh_attest's `MAX_LOG`) and refuses a longer one rather than cut it.

use nsh_attest::consts::MAX_LOG;

use crate::kernel_document::log_fits;

#[test]
fn the_kernel_attests_every_log_the_client_reads_and_no_longer_one() {
    assert!(log_fits(&vec![0; MAX_LOG]) && !log_fits(&vec![0; MAX_LOG + 1]));
}
