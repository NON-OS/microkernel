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

//! A grant is in the table before its claim's release drains it, or is never entered (K10).

use std::sync::{mpsc::channel, Arc, Mutex};
use std::{thread, time::Duration};

use super::leaving::{done, take_held};
use super::state::{next_epoch, BUSY, CLAIMS};
use super::still_held::record_while_held;
use super::types::Claim;

static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());
const PID: u32 = 9;

fn claimed(device_id: u64) -> u64 {
    let epoch = next_epoch();
    CLAIMS.lock().push(Claim { pid: PID, device_id, epoch });
    epoch
}

/// The release's first step, as claim/release.rs takes it.
fn release(device_id: u64) {
    let mut claims = CLAIMS.lock();
    take_held(&mut claims, &mut BUSY.lock(), PID, device_id).unwrap();
}

#[test]
fn a_grant_is_entered_only_while_its_claim_holds() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let epoch = claimed(1);
    assert_eq!(record_while_held(PID, 1, epoch, || 10), Some(10));
    assert_eq!(record_while_held(8, 1, epoch, || 11), None, "another pid");
    assert_eq!(record_while_held(PID, 1, epoch + 1, || 12), None, "another epoch");
    release(1);
    assert_eq!(record_while_held(PID, 1, epoch, || 13), None, "released");
    done(&mut BUSY.lock(), 1);
}

#[test]
fn a_release_during_a_grants_entry_waits_and_drains_it() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let epoch = claimed(2);
    let table = Arc::new(Mutex::new(Vec::new()));
    let (inside, entered) = channel();
    let t = Arc::clone(&table);
    let mapper = thread::spawn(move || {
        record_while_held(PID, 2, epoch, || {
            inside.send(()).unwrap();
            thread::sleep(Duration::from_millis(50));
            t.lock().unwrap().push(1u64);
        })
    });
    entered.recv().unwrap();
    release(2);
    let drained = std::mem::take(&mut *table.lock().unwrap());
    assert_eq!(mapper.join().unwrap(), Some(()));
    assert_eq!(drained, [1], "the release waited for the entry, then drained it");
    assert!(table.lock().unwrap().is_empty(), "nothing entered after the drain");
    done(&mut BUSY.lock(), 2);
}
