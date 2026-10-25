// NONOS Operating System (AGPL-3.0-or-later)
//! The riscv64 level a trap came from, out of the kernel's own frame: from
//! U-mode exactly when sstatus.SPP is clear, whatever the other bits hold
//! (privileged spec, sstatus: SPP is bit 8, 0 for U-mode).

use crate::fields::TrapFrame;

#[test]
fn from_user_exactly_when_spp_is_clear() {
    // A spread of sstatus values: every low 12-bit pattern, then each with
    // the higher bits the kernel sets (SUM, MXR, FS, VS) filled in.
    let high = [0usize, 1 << 18, 1 << 19, 0b11 << 13, 0b11 << 9, usize::MAX << 12];
    for low in 0..1usize << 12 {
        for h in high {
            let sstatus = low | h;
            let frame = TrapFrame { sstatus, ..TrapFrame::default() };
            assert_eq!(frame.is_from_user(), sstatus >> 8 & 1 == 0, "sstatus {sstatus:#x}");
        }
    }
    assert_eq!(crate::csr_status::SSTATUS_SPP, 1 << 8);
}
