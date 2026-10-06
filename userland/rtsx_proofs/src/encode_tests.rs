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

//! The words the driver writes, against the expressions in rtsx_pcr.c.

use crate::regs::sd::SD_CMD0;
use crate::wire::buffer::{CmdBuf, MAX_CMDS};
use crate::wire::encode::*;
use crate::wire::{classify, CmdKind, Outcome};

#[test]
fn a_command_entry_is_built_as_rtsx_pci_add_cmd_builds_it() {
    // WRITE_REG_CMD, SD_CMD0 (0xFDA9), mask 0xFF, SD_CMD_START | 8.
    assert_eq!(cmd_entry(CmdKind::Write, SD_CMD0, 0xFF, 0x48), 0x7DA9_FF48);
    assert_eq!(cmd_entry(CmdKind::Check, 0xFDB3, 0x60, 0x60), 0xBDB3_6060);
    assert_eq!(cmd_entry(CmdKind::Read, 0xFDA3, 0, 0), 0x3DA3_0000);
}

#[test]
fn haimr_and_the_engine_control_words() {
    // HAIMR_WRITE_START | (FPDCTL & 0x3FFF) << 16 | mask << 8 | data.
    assert_eq!(haimr_write(0xFC00, 0x01, 0x00), 0xFC00_0100);
    assert_eq!(haimr_read(0xFE90), 0x80000000 | (0x3E90 << 16));
    assert!(haimr_done(0x7FFF_FFFF) && !haimr_done(0x8000_0000));
    // Three entries: START, hardware auto response, 12 bytes.
    assert_eq!(hcbctlr(3), 0xC000_000C);
    // DEVICE_TO_HOST << 29 | TRIG_DMA | ADMA_MODE.
    assert_eq!(hdbctlr_read(), 0xA800_0000);
}

#[test]
fn a_scatter_gather_entry_ends_valid_with_data() {
    // RTSX_SG_VALID | RTSX_SG_TRANS_DATA | RTSX_SG_END = 0x23.
    assert_eq!(sg_entry(0x1234_5000, 512, true), 0x1234_5000_0020_0023);
    assert_eq!(sg_entry(0x1000, 0x10000, false), 0x0000_1000_1000_0021);
}

#[test]
fn the_buffer_keeps_order_and_refuses_the_257th_entry() {
    let mut buf = CmdBuf::new();
    buf.write(0xFDA0, 0xC0, 0x80).add(CmdKind::Read, 0xFDA3, 0, 0);
    assert_eq!(buf.words(), &[0x7DA0_C080, 0x3DA3_0000]);
    for _ in 2..MAX_CMDS {
        buf.write(0xFC00, 1, 0);
    }
    assert!(!buf.overflowed());
    buf.write(0xFC00, 1, 0);
    assert!(buf.overflowed());
    assert_eq!(buf.words().len(), MAX_CMDS);
}

#[test]
fn bipr_is_read_as_the_isr_reads_it() {
    assert_eq!(classify(0), Outcome::Pending);
    assert_eq!(classify(1 << 16), Outcome::Pending);
    assert_eq!(classify(1 << 29), Outcome::Ok);
    assert_eq!(classify((1 << 29) | (1 << 28)), Outcome::Fail);
    assert_eq!(classify((1 << 29) | (1 << 24)), Outcome::Fail);
    assert_eq!(classify(u32::MAX), Outcome::Gone);
}
