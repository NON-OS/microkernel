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

//! The port window a PIO grant may hand out (plan/54 K6, A10 gaps 3 and 5).
//! Pure, and proven on the host (kernel_proofs::pio_window).
//!
//! A device record's I/O BAR is granted whole, so a record that names the
//! machine's own ports would hand a capsule PCI configuration (0xCF8 to
//! 0xCFF, every device's config space and BARs), the interrupt controllers,
//! the timer or CMOS. A window touching any of them is refused. Its count is
//! converted, never masked: a 64 KiB window does not become zero ports.

/// The ports no grant may include, as inclusive ranges.
pub(crate) const RESERVED: [(u16, u16); 5] = [
    (0x0020, 0x0021), // the primary PIC
    (0x0040, 0x0043), // the PIT
    (0x0070, 0x0071), // CMOS and the NMI mask
    (0x00A0, 0x00A1), // the secondary PIC
    (0x0CF8, 0x0CFF), // PCI configuration
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Refused {
    /// Past port 0xFFFF, or a count no u16 holds.
    Overflow,
    /// It includes a reserved port.
    Reserved,
}

/// (base, count) for a BAR of `size` ports at `base`; `size` is not zero.
pub(crate) fn window(base: u64, size: u64) -> Result<(u16, u16), Refused> {
    let first = u16::try_from(base).map_err(|_| Refused::Overflow)?;
    let count = u16::try_from(size).map_err(|_| Refused::Overflow)?;
    let last = u64::from(first) + u64::from(count) - 1;
    if count == 0 || last > u64::from(u16::MAX) {
        return Err(Refused::Overflow);
    }
    let touches =
        |&(lo, hi): &(u16, u16)| u64::from(lo) <= last && u64::from(first) <= u64::from(hi);
    if RESERVED.iter().any(touches) {
        return Err(Refused::Reserved);
    }
    Ok((first, count))
}
