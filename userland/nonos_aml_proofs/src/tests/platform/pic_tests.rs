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

//! \_PIC is called with the interrupt model, when firmware has one.

use super::run::evaluator;
use crate::consts::at;
use crate::enc::*;
use crate::platform::set_pic;
use crate::probe::Probe;
use crate::run::{boot, num};

#[test]
fn pic_is_called_with_the_model_and_absent_pic_is_none() {
    let src = cat(&[
        &name_obj("\\PICM", &int(9)),
        &method("\\_PIC", 1, &store(&arg(0), &name("\\PICM"))),
    ]);
    let (mut aml, _) = boot(&src, 2);
    let mut probe = Probe::default();
    assert_eq!(set_pic(&mut aml, &mut evaluator(&mut probe), 1), Some(Ok(())));
    let picm = at(&aml, "\\PICM").unwrap();
    assert_eq!(num(&evaluator(&mut probe)(&mut aml, picm, vec![]).unwrap()), 1);
    let (mut none, _) = boot(&[], 2);
    assert_eq!(set_pic(&mut none, &mut evaluator(&mut probe), 1), None);
}
