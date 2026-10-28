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

use super::loader::carried::carried;
use super::loader::send::pieces;
use super::loader::sfi::{layout, CSS, DATA, PKEY, SIGN};
use super::rig::{bootloader, run, SOLAR, TYP};

#[test]
fn a_section_goes_in_fragments_of_at_most_252_octets_kind_first() {
    let key: Vec<u8> = (0..=255).collect();
    let p: Vec<Vec<u8>> = pieces(PKEY, &key).collect();
    assert_eq!(p.iter().map(Vec::len).collect::<Vec<_>>(), [253, 5]);
    assert!(p.iter().all(|f| f[0] == PKEY));
    assert_eq!([&p[0][1..], &p[1][1..]].concat(), key);
    assert_eq!(pieces(DATA, &[]).count(), 0);
    assert_eq!(pieces(DATA, &[7; 252]).count(), 1);
}

#[test]
fn an_ecdsa_engine_takes_the_ecdsa_header_then_every_command() {
    let (r, m, _) = run(bootloader(TYP, 1), 0x0032);
    assert_eq!(r, Ok(()));
    let fw = carried(TYP.3).unwrap().sfi;
    let f = &m.fragments;
    assert_eq!((f[0][0], &f[0][1..]), (CSS, &fw[644..772]));
    assert_eq!((f[1][0], &f[1][1..]), (PKEY, &fw[772..868]));
    assert_eq!((f[2][0], &f[2][1..]), (SIGN, &fw[868..964]));
    assert!(f[3..].iter().all(|d| d[0] == DATA && (d.len() - 1) % 4 == 0));
    assert_eq!(f[3..].iter().flat_map(|d| d[1..].to_vec()).collect::<Vec<_>>(), fw[964..]);
}

#[test]
fn an_rsa_engine_takes_the_rsa_header_in_five_fragments() {
    let (r, m, _) = run(bootloader(SOLAR, 0), 0x0033);
    assert_eq!(r, Ok(()));
    let fw = carried(SOLAR.3).unwrap().sfi;
    let kinds: Vec<(u8, usize)> = m.fragments[..5].iter().map(|f| (f[0], f.len() - 1)).collect();
    assert_eq!(kinds, [(CSS, 128), (PKEY, 252), (PKEY, 4), (SIGN, 252), (SIGN, 4)]);
    let header: Vec<u8> = m.fragments[..5].iter().flat_map(|f| f[1..].to_vec()).collect();
    assert_eq!(header, [&fw[..384], &fw[388..644]].concat());
    assert_eq!(m.fragments[5][1..], fw[964..964 + m.fragments[5].len() - 1]);
}

#[test]
fn a_file_without_the_headers_its_engine_needs_is_refused() {
    let mut fw = vec![0u8; 1000];
    assert_eq!(layout(&fw, 0x17, 0).err(), Some("firmware has no RSA header"));
    fw[8..12].copy_from_slice(&0x0001_0000u32.to_le_bytes());
    assert_eq!(layout(&fw, 0x13, 0).unwrap().payload, 644);
    assert_eq!(layout(&fw, 0x13, 1).err(), Some("secure boot engine is not RSA"));
    assert_eq!(layout(&fw, 0x17, 1).err(), Some("firmware has no ECDSA header"));
    fw[644] = 0x06;
    fw[652..656].copy_from_slice(&0x0002_0000u32.to_le_bytes());
    assert_eq!(layout(&fw, 0x17, 0).unwrap().payload, 964);
    assert_eq!(layout(&fw, 0x17, 1).unwrap().headers[1], (PKEY, 772, 96));
    assert_eq!(layout(&fw, 0x17, 2).err(), Some("secure boot engine unknown"));
    assert_eq!(layout(&fw[..964], 0x17, 1).err(), Some("firmware ends in its header"));
}
