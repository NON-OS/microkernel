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

//! The port gate: quiesce returns only once every access already
//! inside it has finished, so no port reaches a function in reset.

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use super::gate::{during, quiesce};

#[test]
fn quiesce_waits_for_an_access_in_flight() {
    let (inside, entered) = channel();
    let (finish, finished) = channel::<()>();
    let access = thread::spawn(move || {
        during(|| {
            inside.send(()).unwrap();
            finished.recv().unwrap();
        })
    });
    entered.recv().unwrap();
    let (quiet, quieted) = channel();
    let retire = thread::spawn(move || {
        quiesce();
        quiet.send(()).unwrap();
    });
    let early = quieted.recv_timeout(Duration::from_millis(200));
    assert!(early.is_err(), "quiesce returned with an access still in flight");
    finish.send(()).unwrap();
    access.join().unwrap();
    quieted.recv_timeout(Duration::from_secs(5)).expect("quiesce after the access");
    retire.join().unwrap();
}
