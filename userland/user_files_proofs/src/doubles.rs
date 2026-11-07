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

//! The doubles every test builds on.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

use nonos_user_files::errno::{Errno, EIO, ENOENT};
use nonos_user_files::wire::client::{Client, Transport};
use nonos_user_files::{serve, Broker, Host, Objects};

/// The volume: objects by number. `cut` counts puts and removes down to a
/// power cut, after which every call fails EIO and nothing more lands; a
/// put the cut falls on lands only its first `torn` bytes, if any.
#[derive(Clone, Default)]
pub struct Volume {
    pub objects: BTreeMap<u64, Vec<u8>>,
    pub cut: Option<usize>,
    pub torn: Option<usize>,
    pub puts: usize,
    pub put_bytes: usize,
    /// Puts and removes that landed: where a cut can fall.
    pub spent: usize,
    /// A full volume: every put is ENOSPC.
    pub full: bool,
}

impl Volume {
    fn spend(&mut self) -> bool {
        self.spent += 1;
        match self.cut {
            Some(0) => false,
            Some(n) => {
                self.cut = Some(n - 1);
                true
            }
            None => true,
        }
    }

    /// The volume as the next boot finds it: the same objects, no cut.
    pub fn reboot(&self) -> Volume {
        Volume { objects: self.objects.clone(), ..Volume::default() }
    }
}

impl Objects for Volume {
    fn put(&mut self, id: u64, bytes: &[u8]) -> Result<(), Errno> {
        if self.full {
            return Err(nonos_user_files::errno::ENOSPC);
        }
        if !self.spend() {
            if let Some(n) = self.torn.take() {
                self.objects.insert(id, bytes[..n.min(bytes.len())].to_vec());
            }
            return Err(EIO);
        }
        self.puts += 1;
        self.put_bytes += bytes.len();
        self.objects.insert(id, bytes.to_vec());
        Ok(())
    }

    fn read(&mut self, id: u64, offset: u64, buf: &mut [u8]) -> Result<usize, Errno> {
        if self.cut == Some(0) {
            return Err(EIO);
        }
        let obj = self.objects.get(&id).ok_or(ENOENT)?;
        let from = (offset as usize).min(obj.len());
        let n = (obj.len() - from).min(buf.len());
        buf[..n].copy_from_slice(&obj[from..from + n]);
        Ok(n)
    }

    fn size(&mut self, id: u64) -> Result<u64, Errno> {
        if self.cut == Some(0) {
            return Err(EIO);
        }
        self.objects.get(&id).map(|o| o.len() as u64).ok_or(ENOENT)
    }

    fn remove(&mut self, id: u64) -> Result<(), Errno> {
        if !self.spend() {
            return Err(EIO);
        }
        self.objects.remove(&id).map(drop).ok_or(ENOENT)
    }
}

/// Clients the test lets in and keeps running, and a clock it moves.
pub struct Desk {
    pub now: Cell<u64>,
    /// How far the wall clock has been set back from the monotonic one.
    pub wall_back: Cell<u64>,
    pub allowed: BTreeSet<u32>,
    pub running: BTreeSet<u32>,
}

impl Desk {
    pub fn new(clients: &[u32]) -> Desk {
        let set: BTreeSet<u32> = clients.iter().copied().collect();
        Desk {
            now: Cell::new(1_700_000_000_000_000_000),
            wall_back: Cell::new(0),
            allowed: set.clone(),
            running: set,
        }
    }

    pub fn step(&self, ns: u64) {
        self.now.set(self.now.get() + ns);
    }
}

impl Host for Desk {
    fn now_ns(&self) -> u64 {
        self.now.get() - self.wall_back.get()
    }
    fn mono_ns(&self) -> u64 {
        self.now.get()
    }
    fn may_use(&self, pid: u32) -> bool {
        self.allowed.contains(&pid)
    }
    fn alive(&self, pid: u32) -> bool {
        self.running.contains(&pid)
    }
}

/// A client's requests served by `broker` as `pid`.
pub struct Wire<'a> {
    pub broker: &'a mut Broker<Volume>,
    pub desk: &'a Desk,
    pub pid: u32,
}

impl Transport for Wire<'_> {
    fn call(&mut self, request: &[u8], reply: &mut [u8]) -> Result<usize, Errno> {
        let out = serve(self.broker, self.desk, self.pid, request);
        reply[..out.len()].copy_from_slice(&out);
        Ok(out.len())
    }
}

/// A client of `broker` as `pid`.
pub fn client<'a>(broker: &'a mut Broker<Volume>, desk: &'a Desk, pid: u32) -> Client<Wire<'a>> {
    Client::new(Wire { broker, desk, pid })
}
