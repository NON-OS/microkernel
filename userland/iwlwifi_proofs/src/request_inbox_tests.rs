// NONOS Operating System (AGPL-3.0-or-later)
//! Every frame a client can put in driver.iwlwifi0's inbox, through the real
//! router and control answer: a link frame goes to the gate, a whole control
//! header is answered within the reply buffer and echoes its header, and
//! anything else, the deleted NIWF tag included, routes nowhere.

use nonos_wifi_core::scan_list::ScanResults;

use crate::control::{answer, route, Heard, Route, View, REPLY_MAX};

const WIFI: u32 = 0x5749_4649;
const NNET: u32 = 0x4E4E_4554;
const NIWF: u32 = 0x4E49_5746;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn check(f: &[u8], can_join: bool, view: &View<'_>) -> Route {
    let r = route(f, can_join);
    let magic = f.get(..4).map(|m| u32::from_le_bytes(m.try_into().unwrap()));
    let mut out = vec![0u8; REPLY_MAX];
    let got = answer(f, view, &mut out);
    match r {
        Route::Link => assert_eq!(magic, Some(NNET)),
        Route::Join(op) => assert!(can_join && matches!(op, 1 | 2 | 5) && f.len() >= 10),
        Route::Control => {
            let n = got.expect("a whole control header is answered");
            assert!((14..=REPLY_MAX).contains(&n), "{n}");
            assert_eq!(out[..10], f[..10], "the reply names its request");
        }
        Route::Other => {
            assert!(magic != Some(NNET) && (magic != Some(WIFI) || f.len() < 10));
            assert_eq!(got, None, "nothing is answered");
        }
    }
    r
}

#[test]
fn random_frames_route_once_and_a_control_reply_stays_in_its_buffer() {
    let list = ScanResults::new();
    let heard = Heard { list: &list, sweeps: 2, stalls: 1, frames: 9, beacons: 3 };
    let views = [
        View { stage: 0, step: 0, detail: 0, heard: None },
        View { stage: 9, step: 3, detail: u32::MAX, heard: Some(heard) },
    ];
    let mut s = 0x4E49_5746_0000_0001u64;
    let mut seen = [0usize; 4];
    for i in 0..200_000usize {
        let len = (xorshift(&mut s) % 40) as usize;
        let mut f: Vec<u8> = (0..len).map(|_| xorshift(&mut s) as u8).collect();
        let tag = [WIFI, NNET, NIWF, xorshift(&mut s) as u32][i % 4].to_le_bytes();
        let n = f.len().min(4);
        f[..n].copy_from_slice(&tag[..n]);
        if f.len() >= 6 && i % 3 == 0 {
            f[4..6].copy_from_slice(&((xorshift(&mut s) % 9) as u16).to_le_bytes());
        }
        let k = match check(&f, i % 2 == 0, &views[i / 2 % 2]) {
            Route::Link => 0,
            Route::Join(_) => 1,
            Route::Control => 2,
            Route::Other => 3,
        };
        seen[k] += 1;
    }
    assert!(seen.iter().all(|&n| n > 3_000), "every route reached: {seen:?}");
    let edges = [&[][..], &NIWF.to_le_bytes(), &WIFI.to_le_bytes()[..3], &[0u8; 64]];
    edges.iter().for_each(|f| assert_eq!(check(f, true, &views[0]), Route::Other));
}
