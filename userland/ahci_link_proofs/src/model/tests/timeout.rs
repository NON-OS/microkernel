// NONOS Operating System (AGPL-3.0-or-later)
//! A command the drive is still running when the driver gives up on it. The
//! drive keeps going unless it is reset; if it is not, its late data and its
//! ending status land in whatever slot 0 holds next. On a QEMU-like HBA,
//! PxTFD never showed BSY; on an Intel-like one, CLO clears the HBA's BSY.

use crate::constants::ata::SECTOR_SIZE;
use crate::engine::transfer::transfer;
use crate::error::AhciError;
use crate::model::{pattern, Outcome, Rig, Step};

const LATE: Step = Step { ms: 40_000, outcome: Outcome::Clean };

/// Each HBA kind: (PxTFD shows BSY once the command is sent, CAP.SCLO).
const HBAS: [(bool, bool); 4] = [(false, true), (false, false), (true, true), (true, false)];

fn sectors(lba: u64, n: u64) -> Vec<u8> {
    (lba..lba + n).flat_map(pattern).collect()
}

#[test]
fn no_late_read_data_lands_in_the_next_request() {
    let mut bad = Vec::new();
    for (bsy_on_send, sclo) in HBAS {
        let mut rig = Rig::new(vec![LATE], sclo);
        let regs = rig.regs();
        rig.hba().quirks.bsy_on_send = bsy_on_send;
        let r = transfer(&mut rig.port, regs, 1_000, 8, false);
        assert_eq!(r, Err(AhciError::Timeout));
        let r = transfer(&mut rig.port, regs, 2_000, 8, false);
        let got = rig.data(8 * SECTOR_SIZE);
        if r != Ok(()) || got != sectors(2_000, 8) {
            let late = got == sectors(1_000, 8);
            bad.push((bsy_on_send, sclo, r, if late { "the late read's data" } else { "other" }));
        }
    }
    assert!(bad.is_empty(), "(BSY on send, SCLO, result, served): {bad:?}");
}

#[test]
fn no_late_write_lands_the_next_payload_on_the_old_sector() {
    let mut bad = Vec::new();
    for (bsy_on_send, sclo) in HBAS {
        let mut rig = Rig::new(vec![LATE], sclo);
        let regs = rig.regs();
        rig.hba().quirks.bsy_on_send = bsy_on_send;
        rig.load(&[0x11; 512]);
        assert_eq!(transfer(&mut rig.port, regs, 1_000, 1, true), Err(AhciError::Timeout));
        rig.load(&[0x22; 512]);
        let r = transfer(&mut rig.port, regs, 2_000, 1, true);
        let medium = &rig.hba().drive.medium;
        let wrong_lba = medium.sector(1_000) == [0x22; 512];
        let lost = medium.sector(2_000) != [0x22; 512];
        if r != Ok(()) || wrong_lba || lost {
            bad.push((bsy_on_send, sclo, r, wrong_lba, lost));
        }
    }
    assert!(bad.is_empty(), "(BSY on send, SCLO, result, on the old LBA, lost): {bad:?}");
}
