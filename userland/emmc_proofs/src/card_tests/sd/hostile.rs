// NONOS Operating System (AGPL-3.0-or-later)
//! An SD card that lies (SD Physical Layer Simplified Specification, as
//! recalled, unverified): a CSD whose version disagrees with the capacity
//! class its OCR claims, the address 0, and answers from a state the
//! sequence did not put the card in. Each is refused, not followed.

use crate::emmc::error::EmmcError;
use crate::model::sd::{sdsc, SdCfg};
use crate::model::{sd_slot, Rig};

fn refused(card: SdCfg, lie: Option<(u8, u32)>) -> (EmmcError, Rig) {
    let rig = Rig::sd(sd_slot(), card);
    rig.sim.borrow_mut().lie = lie;
    let e = rig.disk(false).err().expect("refused");
    (e, rig)
}

#[test]
fn a_csd_version_against_the_cards_addressing_is_refused() {
    let (e, rig) = refused(SdCfg { ccs: false, ..SdCfg::default() }, None);
    assert_eq!(e, EmmcError::NoCapacity, "bytes past 4 GiB would wrap");
    assert!(rig.logged("SD csd structure 1 on a byte-addressed card"));
    let (e, rig) = refused(SdCfg { v2: true, ccs: true, ..sdsc() }, None);
    assert_eq!(e, EmmcError::NoCapacity);
    assert!(rig.logged("SD csd structure 0 on a block-addressed card"));
}

#[test]
fn the_address_zero_is_refused() {
    let (e, _) = refused(SdCfg { rca: 0, ..SdCfg::default() }, None);
    assert!(matches!(e, EmmcError::Status { cmd: 3, .. }));
}

#[test]
fn an_answer_from_the_wrong_state_is_refused() {
    let (e, _) = refused(SdCfg::default(), Some((3, 3)));
    assert!(matches!(e, EmmcError::Status { cmd: 3, .. }), "CMD3 from stby");
    let (e, rig) = refused(SdCfg::default(), Some((7, 4)));
    assert!(matches!(e, EmmcError::BadState(_)) && rig.logged("CMD7 found the card in tran"));
    let (e, rig) = refused(SdCfg::default(), Some((6, 3)));
    assert!(matches!(e, EmmcError::BadState(_)) && rig.logged("CMD6 found the card in stby"));
    let (e, rig) = refused(sdsc(), Some((16, 5)));
    assert!(matches!(e, EmmcError::BadState(_)) && rig.logged("CMD16 found the card in data"));
}
