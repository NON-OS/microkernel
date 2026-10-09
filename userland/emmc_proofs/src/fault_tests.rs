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

//! Failures in service: each is reported as an error within the kernel's
//! five-second reply wait, and the card is back in transfer state for the
//! next request.

use crate::emmc::disk::SECTOR_SIZE;
use crate::emmc::error::EmmcError;
use crate::model::{CardCfg, HostCfg, Rig, St};

/// The kernel gives a request this long (reply_wait.rs SLOW_BUDGET_MS).
const REPLY_BUDGET_MS: u64 = 5_000;

#[test]
fn a_data_crc_error_fails_one_read_and_the_next_one_works() {
    let rig = Rig::new(HostCfg::default(), CardCfg::default());
    let mut disk = rig.disk(true).unwrap();
    rig.sim.borrow_mut().fail_next_data = 1;
    let mut buf = vec![0u8; 8 * SECTOR_SIZE];
    let t0 = rig.ms();
    match disk.read(40, 8, &mut buf) {
        Err(EmmcError::DataError { cmd: 18, err }) => assert!(err & 1 << 5 != 0, "err {err:#x}"),
        other => panic!("{other:?}"),
    }
    assert!(rig.ms() - t0 < REPLY_BUDGET_MS);
    assert_eq!(rig.sim.borrow().card.state, St::Tran);
    assert!(rig.logged("read failed: emmc: data transfer failed on the bus (err 0x20)"));
    disk.read(40, 8, &mut buf).expect("next read");
    rig.assert_clean();
}

#[test]
fn a_data_crc_error_on_a_write_leaves_the_card_ready() {
    let rig = Rig::new(HostCfg::default(), CardCfg::default());
    let mut disk = rig.disk(true).unwrap();
    rig.sim.borrow_mut().fail_next_data = 1;
    let data = vec![0xa5u8; 2 * SECTOR_SIZE];
    assert!(matches!(disk.write(3, 2, &data), Err(EmmcError::DataError { cmd: 25, .. })));
    assert_eq!(rig.sim.borrow().card.state, St::Tran);
    disk.write(3, 2, &data).expect("retry");
    rig.assert_clean();
}

#[test]
fn a_card_stuck_programming_fails_the_write_inside_the_reply_budget() {
    let rig =
        Rig::new(HostCfg::default(), CardCfg { stuck_after_write: true, ..CardCfg::default() });
    let mut disk = rig.disk(true).unwrap();
    let t0 = rig.ms();
    let data = vec![1u8; SECTOR_SIZE];
    assert_eq!(disk.write(9, 1, &data), Err(EmmcError::CardStuck));
    let took = rig.ms() - t0;
    assert!(took < REPLY_BUDGET_MS, "took {took} ms");
}

#[test]
fn a_card_that_stops_answering_fails_fast() {
    let rig = Rig::new(HostCfg::default(), CardCfg::default());
    let mut disk = rig.disk(true).unwrap();
    rig.sim.borrow_mut().card.cfg.absent = true;
    let t0 = rig.ms();
    let mut buf = vec![0u8; 4 * SECTOR_SIZE];
    assert_eq!(disk.read(0, 4, &mut buf), Err(EmmcError::CmdTimeout(23)));
    assert!(rig.ms() - t0 < REPLY_BUDGET_MS);
    assert_eq!(disk.flush(), Ok(()));
    let mut cached = Rig::new(
        HostCfg::default(),
        CardCfg { cache_kib: 64, cache_on: true, ..CardCfg::default() },
    )
    .disk(true)
    .unwrap();
    cached.host.io.0.borrow_mut().card.cfg.absent = true;
    assert_eq!(cached.flush(), Err(EmmcError::CmdTimeout(6)));
}

#[test]
fn a_request_past_the_card_end_the_card_rejects_is_an_error() {
    // The driver checks the span itself; this holds the card's own check
    // against a capacity the card shrank after bring-up.
    let rig = Rig::new(HostCfg::default(), CardCfg::default());
    let mut disk = rig.disk(true).unwrap();
    rig.sim.borrow_mut().card.cfg.sec_count = 10;
    let mut buf = vec![0u8; SECTOR_SIZE];
    assert!(disk.read(20, 1, &mut buf).is_err());
    assert_eq!(rig.sim.borrow().card.state, St::Tran);
}
