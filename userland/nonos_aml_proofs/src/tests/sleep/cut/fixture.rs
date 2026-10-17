// NONOS Operating System (AGPL-3.0-or-later)
//! A scene as AML. Markers to Debug: \_TTS 10 and 11 around its loop;
//! \_PTS 20 after storing its argument, 21 after its loop; \_S5 30.

use super::gen::Scene;
use crate::enc::*;

fn mark(n: u64) -> Vec<u8> {
    store(&int(n), &op(0x5B31))
}

fn spin(n: u8) -> Vec<u8> {
    let less = e(0x95, &[&local(0), &int(u64::from(n))]);
    cat(&[&store(&int(0), &local(0)), &pkg(0xA2, &cat(&[&less, &e(0x75, &[&local(0)])]))])
}

fn held(what: u8) -> Option<&'static str> {
    [None, Some("\\MUTX"), Some("\\_GL")][what as usize]
}

pub fn source(sc: &Scene) -> Vec<u8> {
    let mut src = cat(&[&name_obj("\\PTSV", &int(0)), &cat(&[&op(0x5B01), &name("\\MUTX"), &[0]])]);
    src.extend(name_obj("\\SPKG", &package(2, &[int(0), int(0)])));
    if let Some(n) = sc.tts {
        src.extend(method("\\_TTS", 1, &cat(&[&mark(10), &spin(n), &mark(11)])));
    }
    if let Some((n, what)) = sc.pts {
        let m = held(what);
        let take = m.map_or(Vec::new(), |m| e(0x5B23, &[&name(m), &[0xFF, 0xFF]]));
        let give = m.filter(|_| sc.releases).map_or(Vec::new(), |m| e(0x5B27, &[&name(m)]));
        let body =
            cat(&[&take, &store(&arg(0), &name("\\PTSV")), &mark(20), &spin(n), &mark(21), &give]);
        src.extend(method("\\_PTS", 1, &body));
    }
    let at = |i: u64| e(0x88, &[&name("\\SPKG"), &int(i), &[0]]);
    let take = sc
        .pts
        .and_then(|(_, w)| held(w))
        .map_or(Vec::new(), |m| e(0x5B23, &[&name(m), &[0xFF, 0xFF]]));
    let fill = cat(&[&store(&name("\\PTSV"), &at(0)), &store(&name("\\PTSV"), &at(1))]);
    src.extend(match sc.s5_method {
        true => method("\\_S5", 0, &cat(&[&take, &fill, &mark(30), &ret(&name("\\SPKG"))])),
        false => name_obj("\\_S5", &package(2, &[int(3), int(3)])),
    });
    src
}
