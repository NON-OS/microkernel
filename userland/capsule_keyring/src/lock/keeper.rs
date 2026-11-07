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

//! The verifier and the wrong-entry count, with no I/O: what a set, a check
//! and a load do to them. The handlers frame these on the wire.

use nonos_lock::argon2::Argon2Error;
use nonos_lock::wire::Count;
use nonos_lock::{rules, verify, Gate, Record, SALT_LEN};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Done: the right passphrase, a new record, or a record taken in.
    Taken,
    /// The passphrase is not the one set; the count went up.
    Wrong,
    /// Too soon after wrong entries; nothing was derived.
    Wait,
    /// No record is held.
    NoRecord,
    /// A record is held already.
    Held,
    /// A passphrase that may not be chosen or checked.
    Invalid,
    /// The Argon2id work area could not be had.
    NoMemory,
}

pub struct Keeper {
    record: Option<Record>,
    gate: Gate,
}

impl Default for Keeper {
    fn default() -> Self {
        Keeper::new()
    }
}

impl Keeper {
    pub fn new() -> Keeper {
        Keeper { record: None, gate: Gate::new(0, 0) }
    }

    pub fn held(&self) -> bool {
        self.record.is_some()
    }

    pub fn record(&self) -> Option<&Record> {
        self.record.as_ref()
    }

    pub fn count(&self, now_ms: i64) -> Count {
        let wait = self.gate.wait_left(now_ms);
        Count { failures: self.gate.failures(), wait_ms: u32::try_from(wait).unwrap_or(u32::MAX) }
    }

    /// Whether `passphrase` is the one set. Wrong entries are counted and
    /// held back as `backoff` says; while the wait runs nothing is derived.
    pub fn check(&mut self, passphrase: &[u8], now_ms: i64) -> Outcome {
        let Some(record) = self.record.as_mut() else {
            return Outcome::NoRecord;
        };
        if !rules::checkable(passphrase) {
            return Outcome::Invalid;
        }
        if self.gate.wait_left(now_ms) > 0 {
            return Outcome::Wait;
        }
        match verify::matches(record, passphrase, &mut || {}) {
            Ok(true) => {
                self.gate.passed();
                record.failures = 0;
                Outcome::Taken
            }
            Ok(false) => {
                self.gate.failed(now_ms);
                record.failures = self.gate.failures();
                Outcome::Wrong
            }
            Err(e) => refused(e),
        }
    }

    /// Choose `new` under the fresh `salt`. With a record held, `current`
    /// must be its passphrase, checked as `check` checks it; with none,
    /// `current` must be empty.
    pub fn set(
        &mut self,
        current: &[u8],
        new: &[u8],
        salt: &[u8; SALT_LEN],
        now_ms: i64,
    ) -> Outcome {
        if rules::acceptable(new).is_err() {
            return Outcome::Invalid;
        }
        match (self.record.is_some(), current.is_empty()) {
            (true, true) => return Outcome::Held,
            (true, false) => match self.check(current, now_ms) {
                Outcome::Taken => {}
                other => return other,
            },
            (false, false) => return Outcome::NoRecord,
            (false, true) => {}
        }
        match verify::make(new, salt, &mut || {}) {
            Ok(record) => {
                self.record = Some(record);
                self.gate = Gate::new(0, now_ms);
                Outcome::Taken
            }
            Err(e) => refused(e),
        }
    }

    /// Take in a record read back from disk, its count with it, when none
    /// is held: a record set this boot is never replaced by an older one.
    pub fn load(&mut self, record: Record, now_ms: i64) -> Outcome {
        if self.record.is_some() {
            return Outcome::Held;
        }
        self.gate = Gate::new(record.failures, now_ms);
        self.record = Some(record);
        Outcome::Taken
    }
}

fn refused(e: Argon2Error) -> Outcome {
    match e {
        Argon2Error::NoMemory => Outcome::NoMemory,
        Argon2Error::BadParams => Outcome::Invalid,
    }
}
