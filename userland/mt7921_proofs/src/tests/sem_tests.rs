// NONOS Operating System (AGPL-3.0-or-later)

//! The patch semaphore on the model (`mt76_connac_mcu_patch_sem_ctrl`): given
//! back after a failed finish, its release confirmed, and a semaphore the
//! driver never had never given back.

use super::download_tests::run;
use super::mcu_tests::card;
use crate::fw::Step;
use crate::model::images::{patch_7922, ram_7922};

fn sem_ops(c: &crate::model::Shared) -> Vec<u8> {
    c.borrow().mcu.plain(0x10, 0).iter().map(|s| s.body[0]).collect()
}

#[test]
fn the_semaphore_is_given_back_after_a_failed_finish_and_must_confirm() {
    let (mut k, c) = card();
    c.borrow_mut().faults.finish = 1;
    assert_eq!(run(&mut k, &patch_7922(), &ram_7922()), Err(Step::Finish));
    assert_eq!(sem_ops(&c), [1, 0], "taken, then given back");
    let (mut k, c) = card();
    c.borrow_mut().faults.sem = Some(2);
    assert_eq!(
        run(&mut k, &patch_7922(), &ram_7922()),
        Err(Step::Semaphore),
        "an unconfirmed release"
    );
    let (mut k, c) = card();
    c.borrow_mut().faults.sem = Some(0);
    assert_eq!(run(&mut k, &patch_7922(), &ram_7922()), Err(Step::Semaphore));
    assert_eq!(sem_ops(&c), [1], "a semaphore not had is not given back");
}
