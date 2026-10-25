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
use alloc::boxed::Box;
use core::ops::Deref;

#[path = "traffic_secrets.rs"]
mod secrets;
pub use secrets::{wipe, Secrets};

/// One stage of a connection's keys. The secrets live in one heap block the
/// value points to, written there by `build` and nowhere else, so moving a
/// `TrafficKeys` copies a pointer and the suite, never a secret; dropping it
/// wipes the block where it lies. Not `Copy`.
#[derive(Clone, PartialEq, Eq)]
pub struct TrafficKeys {
    pub suite: u16,
    secrets: Box<Secrets>,
}

impl TrafficKeys {
    /// Keys `fill` writes into their block in place; `None`, with the block
    /// wiped, when it fails.
    pub fn build(suite: u16, fill: impl FnOnce(&mut Secrets) -> bool) -> Option<Self> {
        let mut secrets = Box::<Secrets>::default();
        fill(&mut secrets).then_some(Self { suite, secrets })
    }
}

impl Deref for TrafficKeys {
    type Target = Secrets;
    fn deref(&self) -> &Secrets {
        &self.secrets
    }
}
