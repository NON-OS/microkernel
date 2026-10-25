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

use super::engine::CapabilityEngine;
use super::isolation::IsolationLevel;
use super::types::Capability;
use spin::Once;

// Read on syscall paths, so a second CPU sees this while the boot CPU may
// still be publishing it. A static mut carried no ordering for that reader.
static CAPABILITY_ENGINE: Once<CapabilityEngine> = Once::new();

pub fn init_capability_system() -> Result<(), &'static str> {
    let engine = CapabilityEngine::new()?;
    CAPABILITY_ENGINE.call_once(|| engine);
    Ok(())
}

pub fn init_capability_engine() -> Result<(), &'static str> {
    init_capability_system()
}

pub fn get_capability_engine() -> Option<&'static CapabilityEngine> {
    CAPABILITY_ENGINE.get()
}

pub fn create_isolation_chamber(
    level: IsolationLevel,
    capabilities: &[Capability],
) -> Result<u64, &'static str> {
    get_capability_engine()
        .ok_or("Capability engine not initialized")?
        .create_isolation_chamber(level, capabilities)
}

pub fn enter_chamber(chamber_id: u64, process_id: u64) -> Result<(), &'static str> {
    get_capability_engine()
        .ok_or("Capability engine not initialized")?
        .enter_chamber(chamber_id, process_id)
}

pub fn check_capability(process_id: u64, capability: Capability) -> Result<bool, &'static str> {
    get_capability_engine()
        .ok_or("Capability engine not initialized")?
        .check_capability(process_id, capability)
}

/// 32 random bytes from the seeded RNG, or an error when it cannot answer.
/// Nothing weaker is ever handed out in their place.
pub fn get_secure_random_bytes() -> Result<[u8; 32], &'static str> {
    let mut bytes = [0u8; 32];
    crate::crypto::rng::fill_random_bytes_secure(&mut bytes).map_err(|_| "no randomness")?;
    Ok(bytes)
}

pub fn init_capabilities() -> Result<(), &'static str> {
    crate::log_info!("NONOS capabilities system initialized");
    Ok(())
}
