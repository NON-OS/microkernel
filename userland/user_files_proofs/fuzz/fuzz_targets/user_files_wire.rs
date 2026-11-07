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


//! Any bytes as a client's request to a broker that holds files, then as a
//! stream of requests, then as a manifest body and a slot
//! (user_files_proofs::fuzz_input). The broker must answer every one with a
//! whole reply to it, and never panic.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_user_files::limits::DEFAULT_QUOTA;
use nonos_user_files::wire::flags::{CREATE, WRITE};
use nonos_user_files::Broker;
use user_files_proofs::doubles::{client, Desk, Volume};
use user_files_proofs::fuzz_input::{run, FUZZ_PID};

fuzz_target!(|b: &[u8]| {
    let desk = Desk::new(&[FUZZ_PID]);
    let mut broker = Broker::new(Volume::default(), DEFAULT_QUOTA);
    let mut c = client(&mut broker, &desk, FUZZ_PID);
    if let Ok((h, _)) = c.open(b"Documents/held", WRITE | CREATE, 0o644) {
        let _ = c.write(h, 1_048_570, b"across a chunk");
    }
    run(&mut broker, &desk, b);
});
