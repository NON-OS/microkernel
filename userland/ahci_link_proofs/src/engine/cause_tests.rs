// NONOS Operating System (AGPL-3.0-or-later)
//! How a failed command's end decides its recovery. Only an error the drive
//! posted says it is done with the command; after anything else it may be
//! running it still, and only a COMRESET stops it.

use super::cause::Cause;
use crate::error::AhciError;

const TFES: u32 = 1 << 30;
const DHRS: u32 = 1 << 0;

#[test]
fn only_an_error_the_drive_posted_leaves_it_done() {
    let failed = AhciError::CommandFailed;
    // TFES with slot 0 still set (AHCI 1.3.1, 6.2.2), and ERR on a clear slot.
    assert_eq!(Cause::of(failed, TFES, 1, 0), Cause::Failed);
    assert_eq!(Cause::of(failed, TFES | DHRS, 1, 0), Cause::Failed);
    assert_eq!(Cause::of(failed, DHRS, 0, 0), Cause::Failed);
    // HBFS, HBDS, IFS and OFS, the HBA's own errors, and the link changes
    // PRCS and PCS: not the drive's end.
    for fatal in [1u32 << 29, 1 << 28, 1 << 27, 1 << 24, 1 << 22, 1 << 6] {
        assert_eq!(Cause::of(failed, fatal, 1, 0), Cause::Abandoned, "PxIS {fatal:#x}");
        let with_tfes = Cause::of(failed, fatal | TFES, 1, 0);
        assert_eq!(with_tfes, Cause::Abandoned, "PxIS {fatal:#x} with TFES");
    }
    // A slot the driver never issued.
    for (ci, sact) in [(1u32 << 1, 0u32), (u32::MAX, 0), (1, 1), (0, 1 << 31)] {
        let got = Cause::of(failed, TFES, ci, sact);
        assert_eq!(got, Cause::Abandoned, "PxCI {ci:#x} PxSACT {sact:#x}");
    }
}

#[test]
fn a_timeout_is_abandoned_whatever_the_port_reads() {
    for is in [0, TFES, DHRS, u32::MAX] {
        for ci in [0, 1] {
            let got = Cause::of(AhciError::Timeout, is, ci, 0);
            assert_eq!(got, Cause::Abandoned, "PxIS {is:#x} PxCI {ci}");
        }
    }
}
