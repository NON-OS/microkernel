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

//! A live boot's volume and memory: which of their stops offer a Retry.

use nonos_market_proto::reason;
use nonos_market_proto::reason::*;

use crate::store::progress::Progress;

#[test]
fn no_data_volume_is_a_live_boot_and_offers_no_retry() {
    let said = reason(WHY_NO_VOLUME);
    assert!(!said.retry);
    // Only a machine with no NONOS disk at all: a live stick's volume is in memory.
    assert!(said.line.contains("no NONOS disk at all") && said.line.contains("Install NONOS"));
    let p = Progress::Failed(WHY_NO_VOLUME);
    assert!(p.needs_installed_system() && !p.retryable());
    assert_eq!(p.button(true), b"Live boot");
}

#[test]
fn a_volume_that_failed_offers_a_retry_and_says_so_honestly() {
    let said = reason(WHY_VOLUME_FAILED);
    assert!(said.retry);
    // The kernel answers EIO for a machine with no NONOS disk too, today.
    assert!(said.line.contains("retry") && said.line.contains("no NONOS disk"), "{}", said.line);
    let p = Progress::Failed(WHY_VOLUME_FAILED);
    assert!(p.retryable() && !p.needs_installed_system());
    assert_eq!(p.button(true), b"Retry");
}

/*
 * A live session's volume is held in memory. Memory filling (ENOSPC) is
 * mended by freeing memory and retrying; too little to hold a volume at all
 * (ENOMEM) is not mended by asking again.
 */
#[test]
fn a_live_session_out_of_memory_is_said_and_only_enospc_retries() {
    let room = reason(WHY_NO_ROOM);
    assert!(room.retry && room.line.contains("free memory") && room.line.contains("install NONOS"));
    let memory = reason(WHY_NO_MEMORY);
    assert!(!memory.retry && memory.line.contains("free memory"), "{}", memory.line);
    assert_eq!(Progress::Failed(WHY_NO_MEMORY).button(true), b"Details");
    assert_eq!(Progress::Failed(WHY_NO_ROOM).button(true), b"Retry");
}
