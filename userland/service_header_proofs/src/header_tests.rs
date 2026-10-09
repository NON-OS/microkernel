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

//! Each service's decode and refusal, described for the harness. The decode
//! and the encoders are the services' own; what each `refusal` assembles is
//! what its loop sends for a frame the decode refuses.

use crate::harness::{boundaries, fuzz, Len, Outcome, Spec, HDR_LEN, STATUS_LEN};

const ROUNDS: usize = 200_000;

/// The header-style services' refusal: their own header encoder and status
/// writer over a buffer the loop sends the first 24 bytes of.
macro_rules! header_refusal {
    ($m:ident) => {{
        fn refusal(op: u16, flags: u16, request_id: u32, status: i32) -> Vec<u8> {
            let req = crate::$m::Request { op, flags, request_id };
            let mut tx = vec![0u8; HDR_LEN + STATUS_LEN];
            crate::$m::response_header(&mut tx, &req, crate::$m::STATUS_LEN as u32);
            crate::$m::write_status(&mut tx, status);
            tx
        }
        refusal
    }};
}

/// The header-style services' decode, whichever order their error pair takes.
macro_rules! header_decode {
    ($m:ident, $err:pat => ($req:ident, $code:ident)) => {{
        fn decode(buf: &[u8]) -> Outcome {
            match crate::$m::parse(buf) {
                Ok((r, body)) => Outcome::Served { op: r.op, flags: r.flags, id: r.request_id, body: body.to_vec() },
                Err($err) => Outcome::Refused {
                    op: $req.op,
                    flags: $req.flags,
                    id: $req.request_id,
                    status: $code,
                },
            }
        }
        decode
    }};
}

/// The drivers' refusal: header, then the status written after it, sent to
/// the kernel's reply endpoint.
macro_rules! driver_refusal {
    ($m:ident) => {{
        fn refusal(op: u16, flags: u16, request_id: u32, status: i32) -> Vec<u8> {
            let req = crate::$m::Request { op, flags, request_id, payload_len: 0 };
            let mut tx = vec![0u8; crate::$m::RESP_HDR_LEN + crate::$m::STATUS_LEN];
            crate::$m::encode_response_header(&mut tx, &req, crate::$m::STATUS_LEN as u32);
            crate::$m::write_status(&mut tx[crate::$m::RESP_HDR_LEN..], status);
            tx
        }
        refusal
    }};
}

/// The drivers' decode: a request or nothing, refused by the loop under
/// zeros with E_INVAL.
macro_rules! driver_decode {
    ($m:ident) => {{
        fn decode(buf: &[u8]) -> Outcome {
            match crate::$m::decode_request(buf) {
                Some(r) => Outcome::Served { op: r.op, flags: r.flags, id: r.request_id, body: Vec::new() },
                None => Outcome::Refused { op: 0, flags: 0, id: 0, status: crate::$m::E_INVAL },
            }
        }
        decode
    }};
}

macro_rules! service_tests {
    ($test:ident, $seed:expr, $spec:expr) => {
        mod $test {
            use super::*;

            #[test]
            fn random_frames_are_served_or_answered_with_a_refusal() {
                fuzz(&$spec, $seed, ROUNDS);
            }

            #[test]
            fn boundary_frames() {
                boundaries(&$spec);
            }
        }
    };
}

service_tests!(login, 0x4C4F_4749_4E00_0001, Spec {
    name: "login",
    magic: crate::login::MAGIC,
    len: Len::Exact,
    echo: true,
    last_op: crate::login::OP_GET_STATE,
    statuses: &[crate::login::E_BAD_LEN, crate::login::E_BAD_MAGIC, crate::login::E_BAD_VERSION],
    rx_len: crate::login::HDR_LEN + crate::login::IPC_PAYLOAD_MAX,
    decode: header_decode!(login, (r, e) => (r, e)),
    refusal: header_refusal!(login),
});

service_tests!(desktop_shell, 0x4E44_5348_0000_0001, Spec {
    name: "desktop_shell",
    magic: crate::desktop_shell::MAGIC,
    len: Len::Exact,
    echo: true,
    last_op: crate::desktop_shell::OP_TAKE_OPEN_ARG,
    statuses: &[
        crate::desktop_shell::E_BAD_LEN,
        crate::desktop_shell::E_BAD_MAGIC,
        crate::desktop_shell::E_BAD_VERSION,
    ],
    rx_len: crate::desktop_shell::HDR_LEN + crate::desktop_shell::IPC_PAYLOAD_MAX,
    decode: header_decode!(desktop_shell, (e, r) => (r, e)),
    refusal: header_refusal!(desktop_shell),
});

service_tests!(wallpaper, 0x4E57_4C50_0000_0001, Spec {
    name: "wallpaper",
    magic: crate::wallpaper::MAGIC,
    len: Len::Exact,
    echo: true,
    last_op: crate::wallpaper::OP_FADE,
    statuses: &[crate::wallpaper::E_BAD_LEN, crate::wallpaper::E_BAD_MAGIC, crate::wallpaper::E_BAD_VERSION],
    rx_len: crate::wallpaper::HDR_LEN + crate::wallpaper::IPC_PAYLOAD_MAX,
    decode: header_decode!(wallpaper, (e, r) => (r, e)),
    refusal: header_refusal!(wallpaper),
});

service_tests!(power, 0x504F_5752_0000_0001, Spec {
    name: "power",
    magic: crate::power::MAGIC,
    len: Len::AtLeast,
    echo: true,
    last_op: crate::power::OP_SHUTDOWN,
    statuses: &[crate::power::E_BAD_LEN, crate::power::E_BAD_MAGIC, crate::power::E_BAD_VERSION],
    // The loop receives into a buffer of IPC_PAYLOAD_MAX bytes, header included.
    rx_len: crate::power::IPC_PAYLOAD_MAX,
    decode: header_decode!(power, (r, e) => (r, e)),
    refusal: header_refusal!(power),
});

service_tests!(attest, 0x4154_5354_0000_0001, Spec {
    name: "attest",
    magic: crate::attest::MAGIC,
    len: Len::AtLeast,
    echo: true,
    last_op: crate::attest::OP_PROOF_ROUTE,
    statuses: &[crate::attest::E_BAD_LEN, crate::attest::E_BAD_MAGIC, crate::attest::E_BAD_VERSION],
    // The loop receives into a buffer of IPC_PAYLOAD_MAX bytes, header included.
    rx_len: crate::attest::IPC_PAYLOAD_MAX,
    decode: header_decode!(attest, (r, e) => (r, e)),
    refusal: header_refusal!(attest),
});

/// entropy's magic, "NOEN". Its protocol module keeps it private; were this
/// copy wrong, every frame would be refused and the fuzz's served count fail.
const ENTROPY_MAGIC: u32 = 0x4E4F_454E;
/// entropy's cap on a request's length field, also private to its protocol.
const ENTROPY_PAYLOAD_MAX: u32 = 4096;

fn entropy_decode(buf: &[u8]) -> Outcome {
    match crate::entropy::decode_request(buf) {
        Ok(r) => Outcome::Served { op: r.op, flags: r.flags, id: r.request_id, body: r.payload.to_vec() },
        // The loop answers every refusal under zeros with EINVAL.
        Err(_) => Outcome::Refused { op: 0, flags: 0, id: 0, status: crate::entropy::EINVAL },
    }
}

fn entropy_refusal(op: u16, flags: u16, request_id: u32, status: i32) -> Vec<u8> {
    crate::entropy::encode_response(op, flags, request_id, status, &[])
}

service_tests!(entropy, 0x4E4F_454E_0000_0001, Spec {
    name: "entropy",
    magic: ENTROPY_MAGIC,
    len: Len::AtLeastUpTo(ENTROPY_PAYLOAD_MAX),
    echo: false,
    last_op: crate::entropy::OP_HEALTHCHECK,
    statuses: &[crate::entropy::EINVAL],
    // MAX_MSG in the loop.
    rx_len: 4608,
    decode: entropy_decode,
    refusal: entropy_refusal,
});

/// virtio_rng's and ps2_input's magics ("NORD", "NKBD"), private to their
/// protocol modules; a wrong copy fails the fuzz's served count.
const VIRTIO_RNG_MAGIC: u32 = 0x4E4F_5244;
const PS2_INPUT_MAGIC: u32 = 0x4E4B_4244;

service_tests!(virtio_rng, 0x4E4F_5244_0000_0001, Spec {
    name: "virtio_rng",
    magic: VIRTIO_RNG_MAGIC,
    len: Len::Unread,
    echo: false,
    last_op: crate::virtio_rng::OP_HEALTHCHECK,
    statuses: &[crate::virtio_rng::E_INVAL],
    // The loop receives into a buffer of one header.
    rx_len: crate::virtio_rng::HDR_LEN,
    decode: driver_decode!(virtio_rng),
    refusal: driver_refusal!(virtio_rng),
});

service_tests!(ps2_input, 0x4E4B_4244_0000_0001, Spec {
    name: "ps2_input",
    magic: PS2_INPUT_MAGIC,
    len: Len::Unread,
    echo: false,
    last_op: crate::ps2_input::OP_POLL_MOUSE,
    statuses: &[crate::ps2_input::E_INVAL],
    // The loop receives into a buffer of one header.
    rx_len: crate::ps2_input::HDR_LEN,
    decode: driver_decode!(ps2_input),
    refusal: driver_refusal!(ps2_input),
});
