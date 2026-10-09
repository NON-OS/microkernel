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

//! The registry is the one the verifier accepts, rebuilt from its own
//! transcript, or nothing is assembled.

use nonos_device_attest::{ek_id, RegistryError};

use crate::assemble::enrolled::TRANSCRIPT_MAX;
use crate::assemble::error::Refusal;
use crate::fixture::{area, device, registry, request, root_bytes, SECRET};
use crate::fixture::{NONCE, VERIFIER, WINDOW};

fn malformed(e: Result<impl Sized, Refusal>) -> bool {
    matches!(e, Err(Refusal::Transcript(RegistryError::Transcript(_))))
}

#[test]
fn a_registry_the_verifier_does_not_accept_is_refused() {
    let mut d = device();
    let mut moved = registry(&d.eks[1], SECRET);
    moved.enroll(ek_id(&[0xD0; 9]), [d.registry.root()[0]; 4]).expect("one more device");
    d.request = request(VERIFIER, WINDOW, &NONCE, &root_bytes(&moved));
    assert_eq!(d.assemble().err(), Some(Refusal::RootNotRequested));
    d.request = request(VERIFIER, WINDOW, &NONCE, &[0x17; 32]);
    assert_eq!(d.assemble().err(), Some(Refusal::RootNotRequested));
}

/// One word of a device line, or of the recorded root, moved by one bit.
#[test]
fn a_transcript_whose_entries_do_not_give_its_root_is_refused() {
    let text = String::from_utf8(device().transcript).expect("text");
    for prefix in ["device ", "root "] {
        let line = text.lines().find(|l| l.starts_with(prefix)).expect("a line");
        let mut words: Vec<String> = line.split(' ').map(String::from).collect();
        let at = words.len() - 1;
        words[at] = (words[at].parse::<u64>().expect("word") ^ 1).to_string();
        let mut d = device();
        d.transcript = text.replacen(line, &words.join(" "), 1).into_bytes();
        assert!(malformed(d.assemble()), "{prefix}: {:?}", d.assemble().err());
    }
}

#[test]
fn a_transcript_that_is_not_text_or_too_long_is_refused() {
    let mut d = device();
    d.transcript.insert(30, 0xFF);
    assert_eq!(d.assemble().err(), Some(Refusal::TranscriptText));
    d.transcript = vec![b'a'; TRANSCRIPT_MAX + 1];
    assert_eq!(d.assemble().err(), Some(Refusal::TranscriptSize));
}

#[test]
fn a_registry_listing_a_revoked_key_as_a_device_is_refused() {
    let mut d = device();
    let id = ek_id(area(&d.eks[1]));
    let hex: String = id.iter().map(|b| format!("{b:02x}")).collect();
    let text = String::from_utf8(d.transcript.clone()).expect("text");
    d.transcript = text.replacen("depth 4\n", &format!("depth 4\nrevoked {hex}\n"), 1).into_bytes();
    assert_eq!(d.assemble().err(), Some(Refusal::Transcript(RegistryError::Revoked)));
}
