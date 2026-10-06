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

//! The device registry R, as the registrar keeps it.
//!
//! One slot per device, keyed on the TPM endorsement key. The registrar sees
//! the key once, at enrollment, and keeps only its hash. A re-enrollment under
//! the same key replaces the commitment, never adds a second: one device, one
//! commitment, so one tag per scope. A revoked key is left out of every later
//! root and refused if it comes back.
//!
//! What the registrar learns: which endorsement key holds which commitment.
//! What it never learns: any tag, because a proof carries the commitment only
//! as a private witness. So the registry links hardware to a commitment and
//! nothing a verifier sees links back to either.
//!
//! The tree is deterministic: slots in the order of the key hashes, empty slots
//! the zero word. Opening an empty slot would need `commit(s) = 0`, a Poseidon
//! preimage. The transcript reproduces the root, so anyone holding it can check
//! the registrar published the root its entries give.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use stark_proofs::crypto::stark::air::RATE;
use stark_proofs::crypto::stark::field::Fp;

use crate::params::hasher;
use crate::witness::Path;

const EK_DOMAIN: &[u8] = b"NONOS-DEVICE-EK-v1";
const HEADER: &str = "nonos-device-registry v1";

/// The deepest registry the circuit is built for: a million devices.
pub const MAX_DEVICE_DEPTH: usize = 20;

/// The hash the registrar keeps in place of an endorsement key.
pub fn ek_id(ek_public: &[u8]) -> [u8; 32] {
    let mut b = blake3::Hasher::new();
    b.update(EK_DOMAIN);
    b.update(ek_public);
    *b.finalize().as_bytes()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Enrolled {
    Added,
    /// The device's old commitment is out of every later root.
    Replaced,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    Depth,
    Full,
    Revoked,
    /// The commitment is already held under another endorsement key.
    Duplicate,
    Transcript(String),
}

#[derive(Clone)]
pub struct Registry {
    depth: usize,
    devices: BTreeMap<[u8; 32], [Fp; RATE]>,
    revoked: BTreeSet<[u8; 32]>,
}

impl Registry {
    pub fn new(depth: usize) -> Result<Registry, RegistryError> {
        if depth == 0 || depth > MAX_DEVICE_DEPTH {
            return Err(RegistryError::Depth);
        }
        Ok(Registry { depth, devices: BTreeMap::new(), revoked: BTreeSet::new() })
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn enroll(
        &mut self,
        ek: [u8; 32],
        commitment: [Fp; RATE],
    ) -> Result<Enrolled, RegistryError> {
        if self.revoked.contains(&ek) {
            return Err(RegistryError::Revoked);
        }
        if self.devices.iter().any(|(k, c)| *k != ek && *c == commitment) {
            return Err(RegistryError::Duplicate);
        }
        if !self.devices.contains_key(&ek) && self.devices.len() >= 1usize << self.depth {
            return Err(RegistryError::Full);
        }
        match self.devices.insert(ek, commitment) {
            Some(_) => Ok(Enrolled::Replaced),
            None => Ok(Enrolled::Added),
        }
    }

    /// Out of every later root, and refused if it enrolls again.
    pub fn revoke(&mut self, ek: [u8; 32]) {
        self.devices.remove(&ek);
        self.revoked.insert(ek);
    }

    fn leaves(&self) -> Vec<[Fp; RATE]> {
        let mut out: Vec<[Fp; RATE]> = self.devices.values().copied().collect();
        out.resize(1usize << self.depth, [Fp::ZERO; RATE]);
        out
    }

    fn levels(&self) -> Vec<Vec<[Fp; RATE]>> {
        let h = hasher();
        let mut levels = alloc::vec![self.leaves()];
        while let Some(top) = levels.last().filter(|l| l.len() > 1) {
            let next = top.as_chunks::<2>().0.iter().map(|[l, r]| h.compress(l, r)).collect();
            levels.push(next);
        }
        levels
    }

    pub fn root(&self) -> [Fp; RATE] {
        self.levels().last().and_then(|l| l.first().copied()).unwrap_or([Fp::ZERO; RATE])
    }

    /// The device's path, for the device alone: the registrar hands it back at
    /// enrollment and whenever the root moves.
    pub fn path(&self, ek: &[u8; 32]) -> Option<Path> {
        let at = self.devices.keys().position(|k| k == ek)?;
        let levels = self.levels();
        let mut siblings = Vec::with_capacity(self.depth);
        let mut right = Vec::with_capacity(self.depth);
        let mut i = at;
        for level in levels.iter().take(self.depth) {
            siblings.push(*level.get(i ^ 1)?);
            right.push(i & 1 == 1);
            i >>= 1;
        }
        Some(Path { siblings, right })
    }

    pub fn transcript(&self) -> String {
        let mut s = format!("{HEADER}\ndepth {}\n", self.depth);
        for (k, c) in &self.devices {
            s += &format!(
                "device {} {} {} {} {}\n",
                hex(k),
                c[0].to_u64(),
                c[1].to_u64(),
                c[2].to_u64(),
                c[3].to_u64()
            );
        }
        for k in &self.revoked {
            s += &format!("revoked {}\n", hex(k));
        }
        let r = self.root();
        s + &format!(
            "root {} {} {} {}\n",
            r[0].to_u64(),
            r[1].to_u64(),
            r[2].to_u64(),
            r[3].to_u64()
        )
    }

    /// Rebuild from a transcript and hold the recorded root to what its entries
    /// give. Every word is refused unless canonical.
    pub fn recompute(text: &str) -> Result<Registry, RegistryError> {
        let bad = |why: &str| RegistryError::Transcript(why.into());
        let mut lines = text.lines();
        if lines.next() != Some(HEADER) {
            return Err(bad("not a device registry transcript"));
        }
        let depth = match lines.next().map(|l| l.split_whitespace().collect::<Vec<_>>()).as_deref()
        {
            Some(["depth", d]) => d.parse::<usize>().map_err(|_| bad("depth"))?,
            _ => return Err(bad("no depth")),
        };
        let mut reg = Registry::new(depth)?;
        let mut recorded = None;
        let mut devices = Vec::new();
        for line in lines {
            match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["device", k, a, b, c, d] => devices
                    .push((unhex(k).ok_or(bad("key"))?, words([a, b, c, d]).ok_or(bad("word"))?)),
                ["revoked", k] => {
                    reg.revoked.insert(unhex(k).ok_or(bad("key"))?);
                }
                ["root", a, b, c, d] => recorded = Some(words([a, b, c, d]).ok_or(bad("word"))?),
                _ => return Err(bad("unknown line")),
            }
        }
        for (k, c) in devices {
            reg.enroll(k, c)?;
        }
        match recorded {
            Some(r) if r == reg.root() => Ok(reg),
            Some(_) => Err(bad("the recorded root is not the one the entries give")),
            None => Err(bad("no root recorded")),
        }
    }
}

fn hex(b: &[u8; 32]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn unhex(s: &str) -> Option<[u8; 32]> {
    if s.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (i, o) in out.iter_mut().enumerate() {
        *o = u8::from_str_radix(s.get(2 * i..2 * i + 2)?, 16).ok()?;
    }
    Some(out)
}

fn words(w: [&&str; 4]) -> Option<[Fp; RATE]> {
    let mut out = [Fp::ZERO; RATE];
    for (o, s) in out.iter_mut().zip(w) {
        let v: u64 = s.parse().ok()?;
        if v >= stark_proofs::crypto::stark::field::P {
            return None;
        }
        *o = Fp::from_u64(v);
    }
    Some(out)
}
