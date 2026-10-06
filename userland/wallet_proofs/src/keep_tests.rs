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

//! A new wallet's records: the vault cleared first and written last, each
//! write checked, and the status naming what the next boot will find.

use crate::keep_plan::{keep_records, kept_status, Kept, Records};
use crate::keyring_says::{refusal, unnamed};
use crate::wallet::vault::Unsealed;

/// A disk that records each write and fails the one named.
struct Disk {
    log: Vec<&'static str>,
    fail: Option<&'static str>,
    seal: Unsealed,
}

impl Disk {
    fn failing(at: Option<&'static str>) -> Disk {
        Disk { log: Vec::new(), fail: at, seal: Unsealed::Store }
    }

    fn write(&mut self, what: &'static str) -> Result<(), Unsealed> {
        self.log.push(what);
        if self.fail == Some(what) {
            Err(self.seal)
        } else {
            Ok(())
        }
    }
}

impl Records for Disk {
    fn clear_vault(&mut self) -> Result<(), Unsealed> {
        self.write("clear vault")
    }
    fn kind(&mut self, _from_key: bool) -> Result<(), Unsealed> {
        self.write("kind")
    }
    fn words(&mut self) -> Result<(), Unsealed> {
        self.write("words")
    }
    fn clear_words(&mut self) -> Result<(), Unsealed> {
        self.write("clear words")
    }
    fn accounts(&mut self) -> Result<(), Unsealed> {
        self.write("accounts")
    }
    fn vault(&mut self) -> Result<(), Unsealed> {
        self.write("vault")
    }
}

#[test]
fn a_wallet_with_words_writes_the_vault_last() {
    let mut disk = Disk::failing(None);
    assert_eq!(keep_records(&mut disk, false), Kept::Kept);
    assert_eq!(disk.log, ["clear vault", "kind", "words", "accounts", "vault"]);
    assert_eq!(kept_status(Kept::Kept, false), None);
}

#[test]
fn a_wallet_from_a_key_clears_the_last_words() {
    let mut disk = Disk::failing(None);
    assert_eq!(keep_records(&mut disk, true), Kept::Kept);
    assert_eq!(disk.log, ["clear vault", "kind", "clear words", "accounts", "vault"]);
}

#[test]
fn a_vault_that_will_not_clear_stops_everything() {
    let mut disk = Disk::failing(Some("clear vault"));
    assert_eq!(keep_records(&mut disk, false), Kept::LastStays(Unsealed::Store));
    assert_eq!(disk.log, ["clear vault"]);
    let said = kept_status(Kept::LastStays(Unsealed::Store), false).unwrap();
    assert!(said.windows(31).any(|w| w == b"any wallet stored before stays:"));
}

#[test]
fn a_failure_after_the_clear_writes_no_vault() {
    for at in ["kind", "words", "accounts"] {
        let mut disk = Disk::failing(Some(at));
        assert_eq!(keep_records(&mut disk, false), Kept::Lost(Unsealed::Store), "{at}");
        assert!(!disk.log.contains(&"vault"), "{at}");
        assert_eq!(disk.log.last(), Some(&at));
    }
    let mut disk = Disk::failing(Some("clear words"));
    assert_eq!(keep_records(&mut disk, true), Kept::Lost(Unsealed::Store));
    assert!(!disk.log.contains(&"vault"));
}

#[test]
fn a_seal_refusal_is_named_as_itself() {
    for why in [Unsealed::NoMachineKey, Unsealed::Keyring] {
        let mut disk = Disk::failing(Some("vault"));
        disk.seal = why;
        assert_eq!(keep_records(&mut disk, false), Kept::Lost(why));
    }
    assert_eq!(Unsealed::from_seal(-2), Unsealed::NoMachineKey);
    assert_eq!(Unsealed::from_seal(-11), Unsealed::Keyring);
    assert_eq!(Unsealed::from_seal(-22), Unsealed::Keyring);
}

#[test]
fn the_status_names_the_way_back_for_each_kind() {
    let lost = [
        Kept::Live,
        Kept::LastStays(Unsealed::Store),
        Kept::LastStays(Unsealed::DiskFull),
        Kept::Lost(Unsealed::DiskFull),
        Kept::Lost(Unsealed::NoMachineKey),
        Kept::Lost(Unsealed::Keyring),
        Kept::Lost(Unsealed::Store),
    ];
    for kept in lost {
        let words = kept_status(kept, false).unwrap();
        let key = kept_status(kept, true).unwrap();
        assert!(words.ends_with(b"write down the phrase"));
        assert!(key.ends_with(b"keep the private key"));
    }
}

#[test]
fn each_keyring_refusal_is_its_own_sentence() {
    let invalid: &[u8] = b"bad phrase";
    let said =
        [refusal(-13, invalid), refusal(-28, invalid), refusal(-11, invalid), refusal(-1, invalid)];
    for (i, a) in said.iter().enumerate() {
        assert_ne!(*a, invalid);
        for b in &said[i + 1..] {
            assert_ne!(a, b);
        }
    }
    assert_eq!(refusal(-22, invalid), invalid);
    assert_ne!(unnamed(true), unnamed(false));
}

#[test]
fn a_full_disk_is_said_as_one() {
    let mut disk = Disk::failing(Some("words"));
    disk.seal = Unsealed::DiskFull;
    assert_eq!(keep_records(&mut disk, false), Kept::Lost(Unsealed::DiskFull));
    assert!(!disk.log.contains(&"vault"));
    assert!(kept_status(Kept::Lost(Unsealed::DiskFull), false)
        .unwrap()
        .starts_with(b"the disk is full"));
    assert_eq!(Unsealed::from_store(-28), Unsealed::DiskFull);
    assert_eq!(Unsealed::from_store(-5), Unsealed::Store);
}

#[test]
fn a_live_session_says_plainly_that_power_off_ends_the_wallet() {
    for from_key in [false, true] {
        let said = kept_status(Kept::Live, from_key).unwrap();
        assert!(said.starts_with(b"this is a live session: nothing is kept past power off"));
    }
}

#[test]
fn the_footer_reads_only_while_a_refresh_runs() {
    use crate::status_words::{kept, reading};
    assert_eq!(reading(false, true, false), None, "no wallet, nothing to read");
    assert_eq!(reading(true, true, false), Some("reading"));
    assert_eq!(reading(true, true, true), Some("reading"));
    assert_eq!(reading(true, false, true), Some("synced"));
    assert_eq!(
        reading(true, false, false),
        Some("not read"),
        "a refresh that ended unread is said"
    );
    assert_eq!(kept(false, false), None);
    assert_eq!(kept(true, true), Some("sealed"));
    assert_eq!(kept(true, false), Some("RAM only"));
}
