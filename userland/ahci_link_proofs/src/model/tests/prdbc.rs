// NONOS Operating System (AGPL-3.0-or-later)
//! A read or a write the drive ends short, with a clean status. The HBA
//! counts the bytes it moved in the command header (PRDBC), in either
//! direction (AHCI 1.3.1, 4.2.2): a short read leaves the previous
//! request's data in the buffer's tail, and a short write never reached
//! the disk whole.

use crate::constants::ata::DATA_BUF_BYTES;
use crate::engine::transfer::transfer;
use crate::error::AhciError;
use crate::model::{Outcome, Rig, Step};

#[test]
fn a_short_transfer_is_an_error_never_stale_data_or_a_lost_write() {
    for write in [false, true] {
        for sectors in [1u32, 2, 8, 63, 64] {
            let bytes = sectors * 512;
            for less in [2, 512, bytes - 2, bytes] {
                let short = Step { ms: 1, outcome: Outcome::Short(less) };
                let mut rig = Rig::new(vec![short], true);
                let regs = rig.regs();
                let r = transfer(&mut rig.port, regs, 7, sectors, write);
                let at = format!("write {write}, {sectors} sectors, {less} bytes short");
                assert_eq!(r, Err(AhciError::ShortTransfer), "{at}");
            }
        }
    }
}

#[test]
fn a_byte_count_no_transfer_could_give_is_an_error() {
    for write in [false, true] {
        for sectors in [1u32, 8, 64] {
            let bytes = sectors * 512;
            for count in [0, 1, bytes - 1, bytes + 1, DATA_BUF_BYTES as u32 + 1, u32::MAX] {
                let mut rig = Rig::new(Vec::new(), true);
                let regs = rig.regs();
                rig.hba().quirks.prdbc = Some(count);
                let r = transfer(&mut rig.port, regs, 7, sectors, write);
                let at = format!("write {write}, {sectors} sectors, PRDBC {count}");
                assert_eq!(r, Err(AhciError::ShortTransfer), "{at}");
            }
        }
    }
}

#[test]
fn a_full_transfer_passes() {
    for write in [false, true] {
        for sectors in 1u32..=64 {
            let mut rig = Rig::new(Vec::new(), true);
            let regs = rig.regs();
            assert_eq!(transfer(&mut rig.port, regs, 7, sectors, write), Ok(()), "{write}");
        }
    }
}
