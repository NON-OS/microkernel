// NONOS Operating System (AGPL-3.0-or-later)

//! A refused interrupt bind ends the attempt only when the claim is gone.

use crate::irq_plan::{after_intx, after_msix, Refusal};

#[test]
fn only_a_lost_claim_ends_the_interrupt_ladder() {
    // ESTALE: the claim epoch is gone, nothing more can be bound.
    assert_eq!(after_intx(-116), Refusal::ClaimLost);
    assert_eq!(after_msix(-116), Refusal::ClaimLost);
    // EPERM on INTx can be a kernel-reserved line: MSI-X is still worth trying.
    assert_eq!(after_intx(-1), Refusal::TryNext);
    // EPERM on MSI-X means the device is not ours.
    assert_eq!(after_msix(-1), Refusal::ClaimLost);
    // No INTx, no MSI-X capability, a busy line: move on, finally run polled.
    for rc in [-22i64, -16, -12, -19, -95] {
        assert_eq!(after_intx(rc), Refusal::TryNext, "intx {rc}");
        assert_eq!(after_msix(rc), Refusal::TryNext, "msix {rc}");
    }
}
