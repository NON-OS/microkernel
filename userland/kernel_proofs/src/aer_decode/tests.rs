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

use super::decode::{
    ext_header, names, AER_CAP_ID, CORRECTABLE, COR_STATUS, UNCORRECTABLE, UNCOR_STATUS,
};

fn all(status: u32, table: &'static [(u32, &'static str)]) -> Vec<&'static str> {
    names(status, table).collect()
}

// PCIe Base 5.0, 7.8.4: capability id 0001h, status registers at +04h and +10h.
#[test]
fn the_register_offsets_are_the_specs() {
    assert_eq!(AER_CAP_ID, 1);
    assert_eq!(UNCOR_STATUS, 0x04);
    assert_eq!(COR_STATUS, 0x10);
}

// 7.8.4.2: Completion Timeout is bit 14, Unsupported Request bit 20,
// Surprise Down bit 5.
#[test]
fn uncorrectable_bits_name_what_the_spec_says() {
    assert_eq!(all(1 << 14, UNCORRECTABLE), ["completion-timeout"]);
    assert_eq!(all(1 << 20, UNCORRECTABLE), ["unsup-req"]);
    assert_eq!(
        all(0x0010_4020, UNCORRECTABLE),
        ["surprise-down", "completion-timeout", "unsup-req"]
    );
    // Bit 0 is reserved (formerly Training Error) and stays unnamed.
    assert!(all(1, UNCORRECTABLE).is_empty());
}

// 7.8.4.5: Receiver Error bit 0, Bad TLP 6, Bad DLLP 7, Replay Timer Timeout 12.
#[test]
fn correctable_bits_name_what_the_spec_says() {
    assert_eq!(
        all(0x0000_10C1, CORRECTABLE),
        ["receiver-error", "bad-tlp", "bad-dllp", "replay-timer-timeout"]
    );
    assert_eq!(all(1 << 13, CORRECTABLE), ["advisory-non-fatal"]);
}

#[test]
fn every_named_bit_is_named_once() {
    for table in [UNCORRECTABLE, CORRECTABLE] {
        for (i, (bit, _)) in table.iter().enumerate() {
            assert!(*bit < 32);
            assert!(table[i + 1..].iter().all(|(b, _)| b != bit));
        }
    }
}

// 7.6.3: id in 15:0, version in 19:16, next offset in 31:20 with 1:0 reserved.
#[test]
fn an_extended_header_gives_id_and_next() {
    assert_eq!(ext_header(0x1401_0001), (AER_CAP_ID, 0x140));
    assert_eq!(ext_header(0x0001_000D), (0x000D, 0));
    // Reserved low bits of the next pointer are dropped.
    assert_eq!(ext_header(0x1431_0001), (AER_CAP_ID, 0x140));
}
