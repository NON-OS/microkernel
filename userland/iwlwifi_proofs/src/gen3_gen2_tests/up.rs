// NONOS Operating System (AGPL-3.0-or-later)

//! A booted 22000-family part comes up speaking what its image lists: one
//! LMAC and SCAN_REQ_UMAC version 15, so its sweep's channels carry their
//! band in `v2.band` (the byte after the channel number), not in their
//! flags; the request is otherwise byte for byte the version 17 one.

use super::super::gen3_boot_tests::answers::firmware_answers;
use super::rig::{polls, rig, run, QU_B, QU_B0};
use crate::gen3::cmds::{LONG_GROUP, SCAN_REQ_UMAC};
use crate::gen3::dev::Speaks;
use crate::gen3::scan::request;
use crate::gen3::select::transport;
use crate::gen3::sweep::Sweep;
use crate::gen3::ucode::Ucode;
use crate::gen3::up::{up, SCAN_IF_ADDR};

#[test]
fn a_version_15_request_differs_from_17_only_in_where_the_band_goes() {
    let (v15, v17) =
        (request(&[1, 6, 36, 165], 9, 15).unwrap(), request(&[1, 6, 36, 165], 9, 17).unwrap());
    let ch = |r: &[u8], i: usize| {
        (u32::from_le_bytes(r[48 + i * 8..52 + i * 8].try_into().unwrap()), r[53 + i * 8])
    };
    let bands = |r: &[u8]| (0..4).map(|i| ch(r, i)).collect::<Vec<_>>();
    assert_eq!(bands(&v15), [(0, 1), (0, 1), (0, 0), (0, 0)], "v2.band: 2.4 GHz 1, 5 GHz 0");
    assert_eq!(bands(&v17), [(1 << 30, 0), (1 << 30, 0), (0, 0), (0, 0)], "flags bits 30-31");
    let differ: Vec<usize> = (0..v15.len()).filter(|&i| v15[i] != v17[i]).collect();
    assert_eq!(differ, [48 + 3, 48 + 5, 56 + 3, 56 + 5], "nothing else");
}

#[test]
fn a_booted_qu_part_comes_up_on_one_lmac_and_sweeps_with_version_15() {
    let r = rig(QU_B, QU_B0, false);
    r.model.s.borrow_mut().responder = Some(firmware_answers(true));
    let t = transport(0x02F0).unwrap();
    let (alive, got) = run(&r, &mut polls(), |dev, c, u| {
        up(dev, c, u, &t, SCAN_IF_ADDR).expect("up");
        let speaks = dev.speaks;
        let started = Sweep::new().start(dev, c, &[1, 36], 0, &mut |_| {});
        (speaks, started.is_ok())
    });
    alive.expect("alive");
    assert_eq!(got, Some((Speaks { cdb: false, scan: 15 }, true)));
    let s = r.model.s.borrow();
    let scan = s.commands.iter().find(|c| (c.group, c.cmd) == (LONG_GROUP, SCAN_REQ_UMAC));
    let scan = &scan.expect("a scan request").payload;
    assert_eq!(
        (&scan[48..52], scan[53], &scan[56..60], scan[61]),
        (&[0u8; 4][..], 1, &[0u8; 4][..], 0)
    );
}

// The image with capability `bit` cleared in its ENABLED_CAPABILITIES TLV.
fn without(blob: &[u8], bit: u32) -> Vec<u8> {
    let (mut b, mut at) = (blob.to_vec(), 88);
    let word = |b: &[u8], at: usize| u32::from_le_bytes(b[at..at + 4].try_into().unwrap());
    while at + 8 <= b.len() {
        let (ty, len) = (word(&b, at), word(&b, at + 4) as usize);
        if ty == 30 && word(&b, at + 8) == bit / 32 {
            let v = word(&b, at + 12) & !(1 << (bit % 32));
            b[at + 12..at + 16].copy_from_slice(&v.to_le_bytes());
        }
        at += 8 + len.next_multiple_of(4);
    }
    b
}

#[test]
fn an_image_without_the_mld_api_is_not_run() {
    let blob = without(QU_B, 110);
    let u = Ucode::parse_for(&blob, super::rig::G2).expect("parses");
    assert_eq!(Speaks::of(&u), Err("firmware without the MLD API"));
    assert!(Speaks::of(&Ucode::parse_for(QU_B, super::rig::G2).unwrap()).is_ok(), "only that");
}
