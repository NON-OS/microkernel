// NONOS Operating System (AGPL-3.0-or-later)
//! An eMMC device that lies (JEDEC eMMC 5.1, as recalled, unverified): a
//! sector-mode card whose CSD gives a size of its own instead of deferring
//! to SEC_COUNT, and answers from a state the sequence did not put it in.

use crate::emmc::error::EmmcError;
use crate::model::{CardCfg, HostCfg, Rig};

fn refused(card: CardCfg, lie: Option<(u8, u32)>) -> (EmmcError, Rig) {
    let rig = Rig::new(HostCfg::default(), card);
    rig.sim.borrow_mut().lie = lie;
    let e = rig.disk(true).err().expect("refused");
    (e, rig)
}

#[test]
fn a_sector_mode_csd_that_names_its_own_size_is_refused() {
    let (e, rig) = refused(CardCfg { c_size: 0x0ff, ..CardCfg::default() }, None);
    assert_eq!(e, EmmcError::NoCapacity, "SEC_COUNT and C_SIZE disagree");
    assert!(rig.logged("card reports no capacity"));
}

#[test]
fn an_answer_from_the_wrong_state_is_refused() {
    let (e, rig) = refused(CardCfg::default(), Some((3, 3)));
    assert!(matches!(e, EmmcError::BadState(_)) && rig.logged("CMD3 found the card in stby"));
    let (e, rig) = refused(CardCfg::default(), Some((7, 4)));
    assert!(matches!(e, EmmcError::BadState(_)) && rig.logged("CMD7 found the card in tran"));
}
