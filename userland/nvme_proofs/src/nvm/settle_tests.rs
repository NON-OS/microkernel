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

//! The settle rule alone: a completion of any status retires the command it
//! names; only no completion leaves it out.

use super::settle_rule::settled;
use crate::error::NvmeError;

#[test]
fn any_completion_retires_and_the_next_settle_waits_for_nothing() {
    for cid in [0, 1, 5, u16::MAX] {
        for seen in [Ok(()), Err(NvmeError::AdminCommandFailed)] {
            let mut out = Some(cid);
            let mut asked = None;
            let r = settled(&mut out, |c| {
                asked = Some(c);
                seen
            });
            assert!(r.is_ok(), "cid {cid}: {seen:?} is not this request's failure");
            assert_eq!((out, asked), (None, Some(cid)));
            assert!(settled(&mut out, |_| panic!("nothing is out")).is_ok());
        }
    }
}

#[test]
fn no_completion_leaves_the_command_out() {
    for e in [NvmeError::ControllerTimeout, NvmeError::ClockFailed] {
        let mut out = Some(5);
        let r = settled(&mut out, |_| Err(e));
        assert!(matches!(r, Err(x) if core::mem::discriminant(&x) == core::mem::discriminant(&e)));
        assert_eq!(out, Some(5), "{e:?} saw no completion, so the buffer stays the device's");
    }
}
