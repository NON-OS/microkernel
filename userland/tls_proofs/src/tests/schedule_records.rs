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

//! The derived keys reproduce the records RFC 8448 publishes.

use super::rfc8448_flight::hs_keys;
use super::schedule_rfc8448::th_finished;
use crate::fixtures::rfc8448::{CLIENT_DATA_RECORD, CLIENT_FINISHED_RECORD};
use crate::fixtures::rfc8448::{SERVER_DATA_RECORD, SERVER_FLIGHT_RECORD, SERVER_MESSAGES};

#[test]
fn the_derived_keys_reproduce_the_published_records() {
    let hs = hs_keys();
    let (key, iv) = (&hs.server_key, &hs.server_iv);
    let opened =
        crate::record_open::open(hs.suite, key, iv, 0, SERVER_FLIGHT_RECORD).expect("open");
    assert_eq!(&opened[..opened.len() - 1], SERVER_MESSAGES);
    let fin = crate::client_finished::client_finished(&hs, &th_finished()).expect("finished");
    assert_eq!(fin, CLIENT_FINISHED_RECORD, "client Finished, byte for byte");
    let app = crate::app_keys::app_keys(&hs, &th_finished()).expect("app keys");
    let data: alloc::vec::Vec<u8> = (0u8..50).collect();
    let (ck, civ) = (&app.client_key, &app.client_iv);
    let sealed = crate::record_seal::seal(app.suite, ck, civ, 0, 23, &data).expect("seal");
    assert_eq!(sealed, CLIENT_DATA_RECORD, "client application data, byte for byte");
    let (sk, siv) = (&app.server_key, &app.server_iv);
    let back = crate::record_open::open(app.suite, sk, siv, 1, SERVER_DATA_RECORD).expect("open");
    assert_eq!(back, [&data[..], &[23]].concat(), "after the ticket at sequence zero");
}
