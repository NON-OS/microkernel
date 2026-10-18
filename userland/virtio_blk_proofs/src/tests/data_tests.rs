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

//! The data buffer: emptied before a read is posted, handed to the driver
//! only while no request holds it, and never larger than one request.

use std::cell::Cell;
use std::rc::Rc;

use nonos_libc::{on_wait, Wake};

use super::model::{Rig, IRQ};
use super::script::answer_once;
use crate::constants::DATA_BUF_LEN;
use crate::io::{submit, BlkError};
use crate::queue::Direction;

#[test]
fn a_read_buffer_holds_nothing_of_the_last_request_when_the_device_gets_it() {
    let rig = Rig::modern();
    let mut q = rig.queue();
    assert!(q.fill(&[0xAA; 4096]), "the last request's bytes");
    let seen = Rc::new(Cell::new(0xFFu8));
    let (dev, s) = (rig.clone(), seen.clone());
    on_wait(move |_| {
        s.set((0..4096).map(|at| dev.data.wrote8(at)).fold(0, |a, b| a | b));
        dev.answer(4096 + 1);
        Wake { after_ms: 0, fired: true }
    });
    assert_eq!(submit(rig.transport(), &mut q, IRQ, Direction::Read, 0, 8), Ok(()));
    assert_eq!(seen.get(), 0, "the device was handed the last request's bytes");
}

#[test]
fn the_buffer_is_the_drivers_only_while_no_request_holds_it() {
    let rig = Rig::modern();
    let mut q = rig.queue();
    answer_once(&rig, |d| {
        (0..512).for_each(|at| d.data.present8(at, at as u8));
        d.answer(513);
    });
    assert_eq!(submit(rig.transport(), &mut q, IRQ, Direction::Read, 0, 1), Ok(()));
    let mut out = [0u8; 512];
    assert!(q.take(&mut out));
    assert!(out.iter().enumerate().all(|(at, &b)| b == at as u8), "the device's bytes");
    let _ = submit(rig.transport(), &mut q, IRQ, Direction::Read, 0, 1);
    assert!(!q.take(&mut out) && !q.fill(&out), "a held buffer was handed out");
}

#[test]
fn nothing_larger_than_the_buffer_is_taken_or_posted() {
    let rig = Rig::modern();
    let mut q = rig.queue();
    let big = vec![0u8; DATA_BUF_LEN as usize + 1];
    assert!(!q.fill(&big));
    assert!(!q.take(&mut vec![0u8; DATA_BUF_LEN as usize + 1]));
    let over = (DATA_BUF_LEN / 512) as u32 + 1;
    for n in [over, u32::MAX] {
        let r = submit(rig.transport(), &mut q, IRQ, Direction::Write, 0, n);
        assert_eq!((r, q.stalled(), rig.posted()), (Err(BlkError::Io), None, 0), "{n}");
    }
}
