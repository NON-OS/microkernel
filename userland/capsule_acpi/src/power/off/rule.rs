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

//! When the wiped power-off runs, pure. A press runs it unless it bounces
//! (within DEBOUNCE_MS of the press before it) or comes before the first
//! publish pass is BOOT_GUARD_MS old, so a status bit stuck from before
//! boot never powers the machine straight back off.

pub const DEBOUNCE_MS: u64 = 500;
pub const BOOT_GUARD_MS: u64 = 2_000;
/// FADT flags (ACPI 6.5, 5.2.9, recalled): at offset 112, PWR_BUTTON is
/// bit 4, set when the power button is a control-method device instead.
const FLAGS: usize = 112;
const PWR_BUTTON: u32 = 1 << 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Press {
    PowerOff,
    Bounce,
    BootGuard,
}

/// A press at `now`; `last` the press before it, `pass` when the first
/// publish pass finished (None while it has not).
pub fn press(last: Option<u64>, pass: Option<u64>, now: u64) -> Press {
    if last.is_some_and(|l| now.saturating_sub(l) < DEBOUNCE_MS) {
        return Press::Bounce;
    }
    match pass {
        Some(t) if now.saturating_sub(t) >= BOOT_GUARD_MS => Press::PowerOff,
        _ => Press::BootGuard,
    }
}

/// A restarted capsule has no boot guard (F-107): the kernel served the
/// button while it was away, so a press it finds is a press, not a status
/// bit stuck from before boot. The bounce still holds.
pub fn on_restart(verdict: Press, restarted: bool) -> Press {
    match verdict {
        Press::BootGuard if restarted => Press::PowerOff,
        v => v,
    }
}

/// Critical while discharging: _BST's critical bit, or the remaining
/// capacity at or under _BIX's low level (ACPI 6.5, 10.2.2, recalled).
pub fn battery_critical(
    discharging: bool,
    flag: bool,
    left: Option<u32>,
    low: Option<u32>,
) -> bool {
    discharging && (flag || matches!((left, low), (Some(r), Some(l)) if r <= l))
}

/// The power button is the FADT's fixed one: not hardware-reduced (the
/// caller has checked) and not a control-method device.
pub fn fixed_button(fadt: &[u8]) -> bool {
    let Some(f) = fadt.get(FLAGS..FLAGS + 4) else { return false };
    u32::from_le_bytes([f[0], f[1], f[2], f[3]]) & PWR_BUTTON == 0
}
