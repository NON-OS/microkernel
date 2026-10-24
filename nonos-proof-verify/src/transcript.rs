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

//! A policy transcript (`nonos-policy-transcript v2`, written by the enroll
//! tool beside every root), read strictly and its root rebuilt from the
//! slots and the pad seed alone: leaves in slot order, the rest padded with
//! Pad leaves from the seed, committed at the gates' depth.

use nonos_attest_path::{digest_to_bytes, leaf, pad_leaf, Poseidon, Tree};

use crate::check::DEPTH;
use crate::measure::{Slot, BOOT_EPOCH, POLICY_EPOCH};

pub const HEADER: &str = "nonos-policy-transcript v2";
const LEAVES: usize = 1 << DEPTH;

/// A transcript whose recorded root its slots reproduce.
#[derive(Debug)]
pub struct Transcript {
    pub slots: Vec<Slot>,
    pub root: [u8; 32],
}

/// Read `text` and rebuild its root. Refused: another header, a depth or epoch
/// the gates do not use, a slot out of order or malformed, a key given twice
/// or missing, an unknown line, or a recorded root the slots do not give.
pub fn read(text: &str) -> Result<Transcript, String> {
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err(format!("not a policy transcript (want the header {HEADER:?})"));
    }
    let mut slots = Vec::new();
    let (mut seed, mut recorded) = (None, None);
    let (mut depth, mut policy, mut boot) = (false, false, false);
    for (n, line) in lines.enumerate() {
        let at = n + 2;
        let f: Vec<&str> = line.split_whitespace().collect();
        match f.as_slice() {
            [] => {}
            ["tool", ..] | ["source", ..] => {}
            ["depth", d] => once(&mut depth, at, "depth", number(d) == Some(DEPTH as u64))?,
            ["policy_epoch", e] => {
                once(&mut policy, at, "policy_epoch", number(e) == Some(POLICY_EPOCH))?
            }
            ["boot_epoch", e] => once(&mut boot, at, "boot_epoch", number(e) == Some(BOOT_EPOCH))?,
            ["pad_seed", p] => set(&mut seed, at, "pad_seed", hex32(p))?,
            ["root", r] => set(&mut recorded, at, "root", hex32(r))?,
            ["slot", i, rest @ ..] => {
                if recorded.is_some() {
                    return Err(format!("line {at}: a slot after the root"));
                }
                if number(i) != Some(slots.len() as u64) {
                    return Err(format!("line {at}: slot {i} out of order"));
                }
                slots.push(slot(rest).ok_or_else(|| format!("line {at}: malformed slot"))?);
            }
            _ => return Err(format!("line {at}: not a transcript line")),
        }
    }
    if !(depth && policy && boot) {
        return Err("the depth or an epoch is not recorded".into());
    }
    let seed = seed.ok_or("no pad seed recorded")?;
    let recorded = recorded.ok_or("no root recorded")?;
    let root = rebuild(&slots, &seed)?;
    if root != recorded {
        return Err(format!(
            "the transcript records root {} but its slots give {}",
            crate::hex(&recorded),
            crate::hex(&root)
        ));
    }
    Ok(Transcript { slots, root })
}

/// The root of `slots` padded with `seed`, as the enroll tool commits it.
pub fn rebuild(slots: &[Slot], seed: &[u8; 32]) -> Result<[u8; 32], String> {
    if slots.is_empty() || slots.len() > LEAVES {
        return Err(format!("{} slots, want 1 to {LEAVES}", slots.len()));
    }
    let h = Poseidon::new();
    let mut leaves: Vec<_> = (0..LEAVES as u32).map(|i| pad_leaf(&h, seed, i)).collect();
    for (at, s) in leaves.iter_mut().zip(slots) {
        *at = leaf(&h, s.kind(), &s.context()).ok_or("a slot has no leaf")?;
    }
    let tree = Tree::commit(&h, &leaves, DEPTH).ok_or("the tree does not commit")?;
    Ok(digest_to_bytes(&tree.root().ok_or("the tree has no root")?))
}

fn slot(f: &[&str]) -> Option<Slot> {
    match f {
        ["kernel", h] => Some(Slot::Kernel(hex32(h).ok()?)),
        ["bootloader", h] => Some(Slot::Bootloader(hex32(h).ok()?)),
        ["capsule", h, "caps", c] => {
            let c = c.strip_prefix("0x")?;
            if c.is_empty() || c.len() > 16 || !c.bytes().all(|b| b.is_ascii_hexdigit()) {
                return None;
            }
            Some(Slot::Capsule(hex32(h).ok()?, u64::from_str_radix(c, 16).ok()?))
        }
        _ => None,
    }
}

fn number(s: &str) -> Option<u64> {
    (!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())).then(|| s.parse().ok())?
}

fn once(seen: &mut bool, at: usize, key: &str, ok: bool) -> Result<(), String> {
    if std::mem::replace(seen, true) {
        return Err(format!("line {at}: {key} given twice"));
    }
    ok.then_some(()).ok_or_else(|| format!("line {at}: a {key} the gates do not use"))
}

fn set<T>(slot: &mut Option<T>, at: usize, key: &str, v: Result<T, String>) -> Result<(), String> {
    if slot.is_some() {
        return Err(format!("line {at}: {key} given twice"));
    }
    *slot = Some(v.map_err(|e| format!("line {at}: {e}"))?);
    Ok(())
}

/// 32 bytes as 64 hex digits, lower or upper case.
pub fn hex32(s: &str) -> Result<[u8; 32], String> {
    let b = s.as_bytes();
    if b.len() != 64 {
        return Err(format!("{s:?} is not 64 hex digits"));
    }
    let digit = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
    let mut out = [0u8; 32];
    for (i, o) in out.iter_mut().enumerate() {
        let (hi, lo) = (digit(b[2 * i]), digit(b[2 * i + 1]));
        *o = hi.zip(lo).map(|(h, l)| h * 16 + l).ok_or_else(|| format!("{s:?} is not hex"))?;
    }
    Ok(out)
}
