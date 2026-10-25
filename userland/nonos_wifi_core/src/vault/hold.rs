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

//! A secret the station keeps past one exchange: a password, a PMK, a
//! password token, the PMKSA cache's tag key. Its bytes live in one box,
//! so they stay at one address while whatever holds the vault moves. The
//! vault is neither `Copy` nor `Clone`, so no second copy can be made of
//! it, and has no `Debug`, so it cannot be logged. It is wiped when it is
//! dropped, which every end of what holds it does.

use super::not_copy::assert_not_copy;
use super::wipe::wipe;
use alloc::boxed::Box;

pub struct Vault<const N: usize> {
    bytes: Box<[u8; N]>,
    len: usize,
}

assert_not_copy!(Vault<32>);
assert_not_copy!(Vault<64>);

impl<const N: usize> Vault<N> {
    /// A vault holding `src`; `None` if it is longer than the vault.
    pub fn new(src: &[u8]) -> Option<Self> {
        let mut v = Self { bytes: Box::new([0u8; N]), len: src.len() };
        v.bytes.get_mut(..src.len())?.copy_from_slice(src);
        Some(v)
    }

    /// A full vault filled in place by `fill`, so the secret is never held
    /// anywhere else; `None` if `fill` could not.
    pub fn fill(fill: impl FnOnce(&mut [u8; N]) -> bool) -> Option<Self> {
        let mut v = Self { bytes: Box::new([0u8; N]), len: N };
        fill(&mut v.bytes).then_some(v)
    }

    pub fn get(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl<const N: usize> Drop for Vault<N> {
    fn drop(&mut self) {
        wipe(&mut self.bytes[..]);
        self.len = 0;
    }
}
