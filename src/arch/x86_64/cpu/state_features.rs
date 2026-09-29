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

//! The feature cache on its own, for the one boot step that needs it before
//! the rest of `init`: turning on SSE, AVX and XSAVE. CPUID only; no timer is
//! touched, so this is safe as early as it is called.

use super::features::CpuFeatures;
use super::state_globals::CPU_FEATURES;

/// Fill the feature cache from CPUID. Idempotent; `init` fills it again later.
pub fn detect_features() {
    let found = CpuFeatures::detect();
    // SAFETY: eK@nonos.systems - called on the boot CPU before any other CPU
    // or thread runs, so nothing reads the cache while it is written.
    unsafe { CPU_FEATURES = found };
}
