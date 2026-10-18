// NONOS Operating System (AGPL-3.0-or-later)
//! A drive that takes seconds to come out of each COMRESET, as spinning
//! disks do: BSY until it posts its signature, a command FIS sent meanwhile
//! dropped, and the slot then ended by that signature as if done. The
//! driver must not issue into it (a write or a flush that never happened
//! would answer Ok), nor reset it again inside ATA's 31 s (its reset would
//! start over), and must serve once it is ready.

use std::time::Duration;

use super::slow_rig::{after_timeout, request};
use crate::clock::{mk_idle_ms, now};
use crate::constants::ata::ATA_WRITE_DMA_EXT;
use crate::constants::timing::READY_MS;
use crate::error::AhciError;

/// Writes and flushes, `gap_ms` apart, from just after the reset until
/// four are served: none answers Ok unless the drive ran it, none fails
/// once the drive is ready, and the drive is never reset again.
fn serve_through(reset_ms: u64, sclo: bool, gap_ms: u64) {
    let at = format!("reset {reset_ms} ms, SCLO {sclo}, {gap_ms} ms apart");
    let (mut rig, ready) = after_timeout(sclo, reset_ms);
    let (mut i, mut served) = (0u64, 0);
    while served < 4 {
        let (start, taken) = (now(), rig.hba().drive.taken.len());
        assert!(start < ready + Duration::from_secs(60), "{at}: never served again");
        let (r, (op, lba)) = request(&mut rig, i);
        let hba = rig.hba();
        if r.is_ok() {
            let last = hba.drive.taken.last().map(|c| (c.op, c.lba));
            let ran = hba.drive.taken.len() == taken + 1 && last == Some((op, lba));
            assert!(ran, "{at}: request {i} answered Ok, never run");
            let lost = op == ATA_WRITE_DMA_EXT && hba.drive.medium.sector(lba) != [i as u8; 512];
            assert!(!lost, "{at}: write {i} answered Ok, not on the disk");
            served += 1;
        } else {
            assert_eq!(r, Err(AhciError::Timeout), "{at}: request {i}");
            assert!(start < ready, "{at}: request {i} failed with the drive ready");
            assert!(now() - start <= Duration::from_millis(READY_MS + 100), "{at}: {i} held");
        }
        let count = &hba.count;
        assert_eq!((count.comresets, count.clo, count.lost), (1, 0, 0), "{at}: request {i}");
        drop(hba);
        i += 1;
        let _ = mk_idle_ms(gap_ms);
    }
}

#[test]
fn a_drive_slow_out_of_its_reset_is_waited_for_never_issued_to() {
    for reset_ms in [2_500, 5_000, 10_000] {
        for sclo in [true, false] {
            for gap_ms in [0, 1_000, 2_000] {
                serve_through(reset_ms, sclo, gap_ms);
            }
        }
    }
}
