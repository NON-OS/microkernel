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

//! One pass: is a remembered network due, and if one is in range, join it.
//!
//! Nothing happens unless the policy store says this boot keeps state (it
//! does only once it restored what setup kept, and never in amnesic mode)
//! and the Wi-Fi switch is not off. The saved record is opened with the TPM
//! key once, the first pass that gets this far, and kept until autojoin is
//! done (`forget_saved`), when the list drops and wipes the passphrases:
//! opening it is a TPM round trip, which every pass used to repeat inside the
//! serve loop. A network is in range when the scan heard its name; a hidden
//! one, whose beacons carry no name, is tried anyway (the driver probes for it
//! by name, the only case a saved name goes on the air). Each join carries the
//! saved flags, so a network saved as WPA3 is never joined with WPA2.

use spin::Mutex;

use nonos_wifi_client::{join_text, load, SavedError, SavedList, ScanNetwork, ScanOutcome};

use super::machine::{Step, CALL_MS};
use super::ready::ready;
use super::tick::say;

/// The saved networks, opened once and kept while autojoin runs.
static SAVED: Mutex<Option<SavedList>> = Mutex::new(None);

/// Drop the saved list, which wipes its passphrases.
pub fn forget_saved() {
    *SAVED.lock() = None;
}

pub fn attempt(tried: u8) -> Step {
    let driver = match ready() {
        Ok(driver) => driver,
        Err(step) => return step,
    };
    let mut saved = SAVED.lock();
    if saved.is_none() {
        match load() {
            Ok(list) => *saved = Some(list),
            Err(SavedError::NoStore) => return Step::NotYet,
            Err(e) => {
                say(b"[NET-CORE] saved Wi-Fi networks not opened: ");
                say(e.text().as_bytes());
                say(b"\n");
                return Step::Done;
            }
        }
    }
    let Some(list) = saved.as_ref() else { return Step::NotYet };
    if list.is_empty() {
        return Step::Done;
    }
    let mut due = (0..list.len()).filter(|i| tried & (1 << i) == 0).peekable();
    if due.peek().is_none() {
        return Step::Done;
    }
    // The driver answers a scan from what its background scan heard; a driver
    // too busy to answer in CALL_MS is not ready for a join either.
    let mut heard = [ScanNetwork::EMPTY; 16];
    let (count, outcome, _) = driver.scan_within(&mut heard, CALL_MS);
    if outcome != ScanOutcome::Scanned {
        return Step::NotYet;
    }
    let in_range = |i: usize| {
        list.join_flags(i).hidden || heard[..count].iter().any(|n| n.ssid() == list.ssid(i))
    };
    let Some(i) = due.find(|&i| in_range(i)) else {
        return Step::NoneInRange;
    };
    let index = i as u8;
    match driver.connect_within(list.ssid(i), list.passphrase(i), list.join_flags(i), CALL_MS) {
        None => {
            say(b"[NET-CORE] joining a saved Wi-Fi network\n");
            Step::Started(index)
        }
        Some(r) if r.code == 0 => {
            say(b"[NET-CORE] joined a saved Wi-Fi network\n");
            Step::Answered { index, joined: true }
        }
        Some(r) => {
            say(b"[NET-CORE] saved Wi-Fi network not joined: ");
            say(join_text(r.code).as_bytes());
            say(b"\n");
            Step::Answered { index, joined: false }
        }
    }
}
