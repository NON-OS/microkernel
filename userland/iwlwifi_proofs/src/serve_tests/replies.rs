// NONOS Operating System (AGPL-3.0-or-later)
//! The client reads the connect and link replies as the driver writes them.


use crate::client_join_wire::{parse_connect_reply, parse_link};
use crate::gen3::join::exchange::Progress;
use crate::join_wire::*;

#[test]
fn the_client_reads_the_connect_and_link_replies_as_written() {
    let mut out = vec![0u8; 64];
    let r = ConnectResult {
        code: CODE_REFUSED,
        progress: Progress { sent: 3, recv: 4, data: 5, eapol: 6, probe: 7, deauth: 1, to_us: 2, refused: 0, state: 2 },
        state: 2,
        akm: 8,
        ap_code: 17,
    };
    let n = encode_connect(&mut out, &r).unwrap();
    assert_eq!(n, 10 + 36);
    let c = parse_connect_reply(&out[10..n]).unwrap();
    assert_eq!((c.code, c.sent, c.recv, c.data, c.eapol, c.probe, c.deauth, c.to_us), (-5, 3, 4, 5, 6, 7, 1, 2));
    assert_eq!((c.state, c.akm, c.ap_code), (2, 8, 17));
    assert!(encode_connect(&mut out[..45], &r).is_none(), "too small a buffer is refused");

    let info = LinkInfo { ssid: b"home", akm: 8 };
    let n = encode_link(&mut out, Some(&info)).unwrap();
    let l = parse_link(&out[10..n]).unwrap();
    assert!(l.associated);
    assert_eq!((l.ssid(), l.akm), (&b"home"[..], 8));
    assert_eq!(&out[11..17], &[0; 6], "no BSSID is told");
    assert!(l.joined_with_sae(b"home"));
    let n = encode_link(&mut out, None).unwrap();
    assert!(!parse_link(&out[10..n]).unwrap().associated);
    let n = encode_code(&mut out, 0).unwrap();
    assert_eq!((n, &out[10..14]), (14, &[0u8; 4][..]));
}
