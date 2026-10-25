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

//! The names a loaded capsule's manifest declares, held to the image's
//! reservation and to its publisher's namespace before anything is spawned
//!. The manifest is the one the spawn then verifies, byte for byte,
//! so a refusal here rests on what the signature will cover.

use super::super::error::LoadError;
use super::baked_apps::APPS;
use super::baked_names::{CORE, FAMILIES};
use super::name_rule::admit_loaded;
use crate::kernel_core::process_spawn::capsule_spawn::runner::{classify_tier, Tier};

/// Refused when either endpoint names what this capsule may not hold.
pub(super) fn check(service: &str, reply: &str, namespace: &str) -> Result<(), LoadError> {
    let publisher = matches!(classify_tier(namespace), Tier::Publisher);
    let baked: [&[(&str, &str)]; 2] = [CORE, APPS];
    for (name, is_reply) in [(service, false), (reply, true)] {
        admit_loaded(name, is_reply, namespace, publisher, &baked, FAMILIES)
            .map_err(|_| LoadError::ReservedName)?;
    }
    Ok(())
}
