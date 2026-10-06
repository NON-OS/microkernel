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

//! The request and seed texts, read the way the pool's tools write them,
//! and the entropy stream the witness draws from.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use stark_proofs::crypto::stark::air::RATE;
use stark_proofs::crypto::stark::field::{Fp, P};

/// A flat JSON object, read by field name. The request is written by this
/// wallet or by the pool's tools, both flat, so a field is found by its key.
pub struct Json<'a>(pub &'a str);

impl<'a> Json<'a> {
    pub fn has(&self, name: &str) -> bool {
        self.0.contains(&format!("\"{name}\""))
    }

    pub fn try_field(&self, name: &str) -> Result<&'a str, String> {
        let key = format!("\"{name}\"");
        let at = self.0.find(&key).ok_or_else(|| format!("request lacks \"{name}\""))?;
        let rest = &self.0[at + key.len()..];
        let colon = rest.find(':').ok_or_else(|| format!("\"{name}\" has no value"))?;
        Ok(rest[colon + 1..].trim_start())
    }

    pub fn try_u64(&self, name: &str) -> Result<u64, String> {
        let v = self.try_field(name)?;
        let end = v.find(|c: char| !c.is_ascii_digit()).unwrap_or(v.len());
        v[..end].parse().map_err(|_| format!("\"{name}\" is not a number"))
    }

    pub fn try_string(&self, name: &str) -> Result<&'a str, String> {
        let v = self.try_field(name)?;
        let v = v.strip_prefix('"').ok_or_else(|| format!("\"{name}\" is not a string"))?;
        let end = v.find('"').ok_or_else(|| format!("\"{name}\" is unterminated"))?;
        Ok(&v[..end])
    }

    pub fn try_strings(&self, name: &str) -> Result<Vec<&'a str>, String> {
        let v = self.try_field(name)?;
        let v = v.strip_prefix('[').ok_or_else(|| format!("\"{name}\" is not a list"))?;
        let end = v.find(']').ok_or_else(|| format!("\"{name}\" is unterminated"))?;
        Ok(v[..end]
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_matches('"'))
            .collect())
    }

    pub fn try_u64s(&self, name: &str) -> Result<Vec<u64>, String> {
        self.try_strings(name)?
            .iter()
            .map(|s| s.parse().map_err(|_| format!("\"{name}\" holds a non-number")))
            .collect()
    }
}

/// A 256-bit word as the pool writes it, into four field limbs, low first.
pub fn try_unpack_digest(hex: &str) -> Result<[Fp; RATE], String> {
    let h = hex.trim().trim_start_matches("0x");
    if h.len() > 64 || !h.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!("{hex} is not a 256 bit hex word"));
    }
    let padded = format!("{h:0>64}");
    let mut limbs = [Fp::ZERO; RATE];
    for (i, limb) in limbs.iter_mut().enumerate() {
        let lo = 64 - 16 * (i + 1);
        let v = u64::from_str_radix(&padded[lo..lo + 16], 16)
            .map_err(|_| format!("{hex} is not hex"))?;
        if v >= P {
            return Err(format!("{hex}: limb {i} is {v}, at or above the field modulus"));
        }
        *limb = Fp::from_u64(v);
    }
    Ok(limbs)
}

/// Four limbs back into the pool's 256-bit word.
pub fn pack_u256(limbs: &[Fp; RATE]) -> String {
    let mut lo: u128 = 0;
    let mut hi: u128 = 0;
    for (i, l) in limbs.iter().enumerate() {
        let x = l.to_u64() as u128;
        match i {
            0 => lo |= x,
            1 => lo |= x << 64,
            2 => hi |= x,
            _ => hi |= x << 64,
        }
    }
    format!("0x{hi:032x}{lo:032x}")
}

pub fn try_address(hex: &str) -> Result<[u8; 20], String> {
    let h = hex.trim().trim_start_matches("0x");
    if h.len() != 40 || !h.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!("address {hex} is not twenty bytes of hex"));
    }
    let mut a = [0u8; 20];
    for (i, b) in a.iter_mut().enumerate() {
        *b = u8::from_str_radix(&h[2 * i..2 * i + 2], 16)
            .map_err(|_| format!("{hex} is not hex"))?;
    }
    Ok(a)
}

/// Field words drawn from a byte stream, eight bytes a draw, a draw at or
/// above p thrown away so every word is uniform.
pub struct Entropy<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Entropy<'a> {
    pub fn new(bytes: &'a [u8]) -> Entropy<'a> {
        Entropy { bytes, at: 0 }
    }

    pub fn words(&mut self, n: usize) -> Result<Vec<Fp>, String> {
        let mut out = Vec::with_capacity(n);
        while out.len() < n {
            let Some(chunk) = self.bytes.get(self.at..self.at + 8) else {
                return Err("not enough entropy: pass at least 512 random bytes".to_string());
            };
            self.at += 8;
            let mut w = [0u8; 8];
            w.copy_from_slice(chunk);
            let v = u64::from_le_bytes(w);
            if v < P {
                out.push(Fp::from_u64(v));
            }
        }
        Ok(out)
    }
}

pub fn quad(w: &[Fp]) -> [u64; RATE] {
    core::array::from_fn(|i| w[i].to_u64())
}
