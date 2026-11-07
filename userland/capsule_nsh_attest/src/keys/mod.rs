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

//! The host's hybrid keys, held by nsh.attest and by no other capsule.
//!
//! On the first ask of a boot the keys sealed in the store are opened with
//! the TPM's machine key. A machine that has none yet, at its first boot,
//! makes them from the kernel's generator, seals them and keeps them.
//! Keys sealed under another boot state (a new kernel or firmware) do not
//! open: they are set aside and new ones made, and the machine is enrolled
//! again. A machine with no TPM has no keys, and says so; nothing is ever
//! made in the clear. Until the store keeps what is written (first-boot
//! setup has not finished) the keys made are held in memory and kept as
//! soon as it does, so the keys the operator enrols are the ones kept.

mod held;
pub mod label;
pub mod layout;
mod make;
mod seal;

use alloc::boxed::Box;

pub use held::Held;
use layout::PLAIN_LEN;
use nsh_front_wire::{ST_NO_KEYS, ST_NO_TPM};
use seal::SEALED_LEN;

use crate::said::Event;
use crate::store::{self, Disk, Found};

/// Where the sealed keys are kept.
pub const KEYS_PATH: &[u8] = b"/nonos/nsh/host.keys";
/// Where keys that no longer open are set aside.
pub const PREV_PATH: &[u8] = b"/nonos/nsh/host.keys.prev";
/// How long after a fault the keys are asked for again.
const RETRY_MS: u64 = 5_000;

/// Why there are no keys now.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// The machine has no TPM.
    NoTpm,
    /// The TPM did not give the sealing key: this errno.
    Tpm(i64),
    /// The kernel's generator did not answer.
    Entropy,
    /// The sealed bytes are not this machine's in this boot state.
    Foreign,
    /// The store did not answer.
    Store,
}

pub struct HostKeys {
    held: Option<Held>,
    /// The sealed bytes of keys made this boot and not yet kept.
    unkept: Option<Box<[u8; SEALED_LEN]>>,
    no_tpm: bool,
    retry_at_ms: u64,
    keep_at_ms: u64,
    last_said: Option<Event>,
}

impl HostKeys {
    pub const fn new() -> HostKeys {
        HostKeys {
            held: None,
            unkept: None,
            no_tpm: false,
            retry_at_ms: 0,
            keep_at_ms: 0,
            last_said: None,
        }
    }

    /// The keys, made or opened on the first ask, or the status nsh.front
    /// is told: `ST_NO_TPM` on a machine with no TPM, else `ST_NO_KEYS`.
    pub fn get(&mut self, now_ms: u64) -> Result<&Held, u16> {
        if self.no_tpm {
            return Err(ST_NO_TPM);
        }
        if self.held.is_none() && now_ms >= self.retry_at_ms {
            if let Err(f) = self.load() {
                self.fail(f, now_ms);
            }
        }
        self.keep_unkept(now_ms);
        match &self.held {
            Some(h) => Ok(h),
            None if self.no_tpm => Err(ST_NO_TPM),
            None => Err(ST_NO_KEYS),
        }
    }

    /// Whether the machine has been found to have no TPM: nothing more is
    /// ever tried this boot.
    pub fn no_tpm(&self) -> bool {
        self.no_tpm
    }

    /// Whether the keys held are kept in the store.
    pub fn kept(&self) -> bool {
        self.held.is_some() && self.unkept.is_none()
    }

    fn load(&mut self) -> Result<(), Fault> {
        seal::probe()?;
        if store::disk() == Disk::Unsettled {
            return Err(Fault::Store);
        }
        let mut plain = Box::new([0u8; PLAIN_LEN]);
        match store::read(KEYS_PATH, SEALED_LEN) {
            Found::Bytes(sealed) => match seal::open(&sealed, &mut plain) {
                Ok(()) => {
                    self.hold(&mut plain, Event::Opened);
                    return Ok(());
                }
                Err(Fault::Foreign) => self.set_aside(&sealed)?,
                Err(f) => return Err(f),
            },
            Found::Over => self.set_aside(&[])?,
            Found::Absent => {}
            Found::Fault => return Err(Fault::Store),
        }
        self.make(&mut plain)
    }

    /// Keep `old` beside the new keys, so a boot back into the state that
    /// sealed it can still be recovered by hand.
    fn set_aside(&mut self, old: &[u8]) -> Result<(), Fault> {
        if !old.is_empty() && !store::put(PREV_PATH, old) {
            return Err(Fault::Store);
        }
        self.last_said = None;
        crate::say(Event::Replaced, None);
        Ok(())
    }

    fn make(&mut self, plain: &mut Box<[u8; PLAIN_LEN]>) -> Result<(), Fault> {
        make::fresh(plain)?;
        let mut sealed = Box::new([0u8; SEALED_LEN]);
        if let Err(f) = seal::seal(plain, &mut sealed) {
            nonos_hash::wipe(&mut plain[..]);
            return Err(f);
        }
        let kept = store::put(KEYS_PATH, &sealed[..]);
        self.unkept = (!kept).then_some(sealed);
        self.hold(plain, if kept { Event::Made } else { Event::MadeUnkept });
        Ok(())
    }

    fn hold(&mut self, plain: &mut [u8; PLAIN_LEN], said: Event) {
        let held = Held::from_plain(plain);
        crate::say(said, Some(held.id()));
        self.held = Some(held);
        self.last_said = Some(said);
    }

    /// Keep keys made this boot once the store keeps what is written,
    /// asking at most every RETRY_MS.
    fn keep_unkept(&mut self, now_ms: u64) {
        let Some(sealed) = &self.unkept else { return };
        if now_ms < self.keep_at_ms {
            return;
        }
        self.keep_at_ms = now_ms.saturating_add(RETRY_MS);
        if store::disk() == Disk::Read && store::put(KEYS_PATH, &sealed[..]) {
            self.unkept = None;
            crate::say(Event::Kept, self.held.as_ref().map(Held::id));
        }
    }

    /// Say a fault once, until another is met.
    fn fail(&mut self, f: Fault, now_ms: u64) {
        let said = match f {
            Fault::NoTpm => {
                self.no_tpm = true;
                Event::NoTpm
            }
            Fault::Tpm(rc) => Event::Tpm(rc),
            Fault::Entropy => Event::Entropy,
            Fault::Foreign | Fault::Store => Event::Store,
        };
        self.retry_at_ms = now_ms.saturating_add(RETRY_MS);
        if self.last_said != Some(said) {
            crate::say(said, None);
            self.last_said = Some(said);
        }
    }
}
