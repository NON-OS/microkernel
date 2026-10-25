// NONOS Operating System (AGPL-3.0-or-later)

//! The serve stage fed hostile requests: random bytes, and each request of
//! both families mutated from a well-formed one, from net_core and from
//! strangers, on a radio up and on one down. Nothing panics, and every reply
//! fits the buffer the serve loop sends from.

use nonos_libc::set_net_core;
use nonos_wifi_core::netif::{wire, MAX_RESPONSE};

use super::answer::answer;
use super::connect::Session;
use super::host_radio::host_radio;
use super::radio::Radio;
use super::scanner::Scanner;
use super::Stage;
use crate::fuzz::{altered, Rng};

const NET_CORE: u32 = 40;
const RX_MAX: usize = wire::HDR_LEN + wire::MAX_FRAME;

fn nnet(op: u16, payload: &[u8]) -> Vec<u8> {
    let mut m =
        [&0x4E4E_4554u32.to_le_bytes()[..], &1u16.to_le_bytes(), &op.to_le_bytes()].concat();
    m.extend_from_slice(&[0; 4]);
    m.extend_from_slice(&7u32.to_le_bytes());
    m.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    [m, payload.to_vec()].concat()
}

fn wifi(op: u16, body: &[u8]) -> Vec<u8> {
    [&0x5749_4649u32.to_le_bytes()[..], &op.to_le_bytes(), &9u32.to_le_bytes(), body].concat()
}

fn seeds() -> Vec<Vec<u8>> {
    let eth = [[0xFF; 6], [0x02; 6]].concat();
    let connect = [&[3u8][..], b"net", &[10], b"password12", &[0]].concat();
    vec![nnet(2, &[]), nnet(3, &[]), nnet(4, &[eth, vec![0x08, 0x00, 1, 2]].concat()), nnet(5, &[])]
        .into_iter()
        .chain([wifi(1, &connect), wifi(2, &[]), wifi(3, &[]), wifi(4, &[]), wifi(5, &[])])
        .collect()
}

fn feed(radio: &mut Radio, rounds: usize, seed: u64) {
    let (mut session, scanner): (Option<Session>, _) = (None, Scanner::new());
    let (mut rng, seeds, mut tx) = (Rng::seeded(seed), seeds(), [0u8; MAX_RESPONSE]);
    let mut changed = 0;
    for i in 0..rounds {
        set_net_core(if i % 2 == 0 { NET_CORE } else { 0 });
        let sender = if i % 3 == 0 { NET_CORE } else { 41 };
        let mut req = match i % 4 {
            0 => rng.noise(RX_MAX),
            _ => {
                let seed = &seeds[rng.below(seeds.len())];
                let m = rng.mutate(seed);
                changed += usize::from(altered(seed, &m));
                m
            }
        };
        req.truncate(RX_MAX);
        let n = answer(&mut req, sender, radio, &mut session, &scanner, Stage::Ready, &mut tx);
        assert!(n <= tx.len(), "a reply of {n} bytes");
    }
    assert!(changed > rounds / 2, "the mutations reach past the seeds");
}

#[test]
fn hostile_requests_to_a_radio_that_is_up_never_panic() {
    feed(&mut host_radio().0, 1200, 0x5E7E_0001);
}

#[test]
fn hostile_requests_to_a_radio_that_is_down_never_panic() {
    feed(&mut Radio::Down, 1200, 0x5E7E_0002);
}
