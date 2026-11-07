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

//! The link's server name: the relay's address in the shipped profile, and
//! a random tor-shaped host with `random-sni`. RFC 6066 3: "Literal IPv4
//! and IPv6 addresses are not permitted in HostName."

use crate::link::sni::write;
use crate::link::sni_random::{random_name, NAME_MAX};

fn is_ipv4_literal(name: &[u8]) -> bool {
    let parts: Vec<&[u8]> = name.split(|&b| b == b'.').collect();
    parts.len() == 4 && parts.iter().all(|p| !p.is_empty() && p.iter().all(u8::is_ascii_digit))
}

#[test]
fn the_shipped_name_is_the_relays_address_as_a_literal() {
    let mut quad = [0u8; 15];
    assert_eq!(write(&mut quad, [93, 184, 216, 34]), b"93.184.216.34");
    assert!(is_ipv4_literal(write(&mut quad, [255, 255, 255, 255])));
}

#[test]
fn a_random_name_has_tors_shape_at_every_length() {
    for first in 0..=255u8 {
        let mut seed = [0x5au8; 21];
        seed[0] = first;
        let mut out = [0u8; NAME_MAX];
        let name = random_name(&mut out, &seed).to_vec();
        let letters = name.len() - 8;
        assert!((8..=20).contains(&letters), "{letters} letters");
        assert!(name.starts_with(b"www.") && name.ends_with(b".net"));
        assert!(name[4..4 + letters].iter().all(|b| b.is_ascii_lowercase() || (b'2'..=b'7').contains(b)));
        assert!(!is_ipv4_literal(&name));
    }
}

#[test]
fn every_letter_of_the_alphabet_is_reachable_and_names_differ_by_seed() {
    let mut seen = [false; 256];
    let mut names = std::collections::BTreeSet::new();
    for v in 0..=255u8 {
        let mut seed = [v; 21];
        seed[0] = 12;
        let mut out = [0u8; NAME_MAX];
        let name = random_name(&mut out, &seed).to_vec();
        for &b in &name[4..24] {
            seen[usize::from(b)] = true;
        }
        names.insert(name);
    }
    let reached = seen.iter().filter(|s| **s).count();
    // 26 letters, 2..7, and the 'w', 'n', 'e', 't' of the fixed parts are
    // outside the window read above.
    assert_eq!(reached, 32);
    assert_eq!(names.len(), 32, "a name is a function of its seed's low five bits");
}

#[test]
fn print_the_server_name_extensions_for_the_branch_notes() {
    let mut quad = [0u8; 15];
    let mut out = [0u8; NAME_MAX];
    let seed = [7u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    for name in [write(&mut quad, [93, 184, 216, 34]).to_vec(), random_name(&mut out, &seed).to_vec()] {
        let mut ext = vec![0, 0];
        ext.extend_from_slice(&((name.len() + 5) as u16).to_be_bytes());
        ext.extend_from_slice(&((name.len() + 3) as u16).to_be_bytes());
        ext.push(0);
        ext.extend_from_slice(&(name.len() as u16).to_be_bytes());
        ext.extend_from_slice(&name);
        let hex: String = ext.iter().map(|b| format!("{b:02x}")).collect();
        println!("sni {} {hex}", String::from_utf8_lossy(&name));
    }
}
