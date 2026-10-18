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

//! A level refused, as one that needs more security than the HID reports
//! would be, passes the client on to the next.

use crate::battery_refuse_tests::still_types;
use crate::sim::devices::{NOTIFY, READ};
use crate::sim::{Db, Peripheral, Run};
use nonos_bt_hogp::host::Battery;

#[test]
fn a_refused_level_passes_to_the_next() {
    let db = Db::default()
        .service(0x1812)
        .char(READ | NOTIFY, 0x2a22, &[0; 8])
        .desc(0x2902, &[0, 0])
        .service(0x180f)
        .char(READ | NOTIFY, 0x2a19, &[30])
        .desc(0x2902, &[0, 0])
        .service(0x180f)
        .char(READ, 0x2a19, &[60]);
    let mut peer = Peripheral::new(db);
    peer.refuse_read = Some((peer.db.handle_of(0x2a19), 0x05));
    let mut run = Run::with(peer);
    run.go();
    assert_eq!(run.batteries, [Battery { instance: 1, percent: 60 }]);
    still_types(&mut run);
}
