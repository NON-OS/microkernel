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

use super::calibrate_consts::{LAPIC_TICKS_PER_MS_MAX, LAPIC_TICKS_PER_MS_MIN};

/* ---- LAPIC timer calibration band ---- */

#[test]
fn calibration_band_admits_crystal_clocked_parts() {
    // Ticks per ms at divide-by-16: Gemini Lake 19.2 MHz crystal, Skylake
    // client 24 MHz, Alder Lake 38.4 MHz, 100 MHz bus, AMD Zen 100 MHz.
    for hz in [19_200_000u64, 24_000_000, 38_400_000, 100_000_000, 400_000_000] {
        let per_ms = hz / 16 / 1000;
        assert!(
            (LAPIC_TICKS_PER_MS_MIN..=LAPIC_TICKS_PER_MS_MAX).contains(&per_ms),
            "{hz} Hz -> {per_ms}/ms clamped"
        );
    }
}
