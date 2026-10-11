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

use super::compat::{ide, Fixed, Ports, IDE_CLASS, IDE_SUBCLASS, PRIMARY, SECONDARY};
use super::fixed_line::{owns_line, PRIMARY_IRQ, SECONDARY_IRQ};

const P0: Ports = Ports { base: 0x1F0, count: 8 };
const P1: Ports = Ports { base: 0x3F6, count: 1 };
const S0: Ports = Ports { base: 0x170, count: 8 };
const S1: Ports = Ports { base: 0x376, count: 1 };

/// The lines a bind may name for this function, as the PIO broker asks.
fn lines(f: &Fixed) -> Vec<u32> {
    (0..=300).filter(|&irq| owns_line(f.bars[0].is_some(), f.bars[2].is_some(), irq)).collect()
}

#[test]
fn piix_in_compatibility_mode_gets_both_channels() {
    // PIIX3 and PIIX4 report 0x80: bus master, both channels fixed.
    let f = ide(IDE_CLASS, IDE_SUBCLASS, 0x80);
    assert_eq!(f.bars, [Some(P0), Some(P1), Some(S0), Some(S1)]);
    assert_eq!(lines(&f), [14, 15]);
    assert_eq!((PRIMARY, SECONDARY), ([P0, P1], [S0, S1]));
    assert_eq!((PRIMARY_IRQ, SECONDARY_IRQ), (14, 15));
}

#[test]
fn a_native_channel_keeps_its_bars_and_line() {
    let primary_native = ide(IDE_CLASS, IDE_SUBCLASS, 0x8F & !0x04);
    assert_eq!(primary_native.bars, [None, None, Some(S0), Some(S1)]);
    assert_eq!(lines(&primary_native), [15]);
    let secondary_native = ide(IDE_CLASS, IDE_SUBCLASS, 0x8A | 0x04);
    assert_eq!(secondary_native.bars, [Some(P0), Some(P1), None, None]);
    assert_eq!(lines(&secondary_native), [14]);
    let both = ide(IDE_CLASS, IDE_SUBCLASS, 0x8F);
    assert_eq!(both.bars, [None; 4]);
    assert!(lines(&both).is_empty());
}

#[test]
fn every_interface_byte_follows_its_two_mode_bits() {
    for progif in 0..=255u8 {
        let f = ide(IDE_CLASS, IDE_SUBCLASS, progif);
        let (pri, sec) = (progif & 1 == 0, progif & 4 == 0);
        assert_eq!(f.bars[0].is_some() && f.bars[1].is_some(), pri, "{progif:#x}");
        assert_eq!(f.bars[2].is_some() && f.bars[3].is_some(), sec, "{progif:#x}");
        let want: Vec<u32> = [(pri, 14), (sec, 15)].iter().filter(|w| w.0).map(|w| w.1).collect();
        assert_eq!(lines(&f), want, "{progif:#x}");
    }
}

#[test]
fn nothing_but_an_ide_function_gets_a_window() {
    for (class, sub) in [(0x01, 0x06), (0x01, 0x00), (0x01, 0x05), (0x03, 0x00), (0x0C, 0x01)] {
        for progif in [0x00, 0x01, 0x80, 0x8A] {
            let f = ide(class, sub, progif);
            assert_eq!(f.bars, [None; 4], "{class:x}/{sub:x}");
            assert!(lines(&f).is_empty());
        }
    }
}
