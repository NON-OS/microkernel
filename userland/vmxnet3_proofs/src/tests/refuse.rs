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

use crate::bench::{Bench, GRANTS};
use crate::init::finish;
use crate::model::{take, Device, Event};

/// Bring-up on a device tuned to fail, holding the driver to its cleanup:
/// quiesce, reset and the shared address cleared before any grant goes back,
/// then every grant and the claim, last taken first.
pub fn refused(tune: impl FnOnce(&mut Device)) -> (&'static str, Device) {
    let mut bench = Bench::new();
    let err = finish(bench.with(tune)).err().expect("bring-up must fail");
    let want = vec![
        ("dma_unmap", GRANTS[5]),
        ("dma_unmap", GRANTS[4]),
        ("dma_unmap", GRANTS[3]),
        ("mmio_unmap", GRANTS[2]),
        ("mmio_unmap", GRANTS[1]),
        ("device_release", GRANTS[0]),
    ];
    assert_eq!(nonos_libc::given_back(), want, "every grant back, last taken first");
    let device = take();
    let n = device.log.len();
    let tail = [
        Event::Command { cmd: 0xCAFE_0001, grants_back: 0 },
        Event::Command { cmd: 0xCAFE_0002, grants_back: 0 },
        Event::Shared(0),
    ];
    assert_eq!(device.log[n - 3..], tail);
    (err, device)
}
