// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! The shipping nox-pull modules, mounted flat as siblings, plus a `term` stand-in.

#[path = "../../../capsule_terminal/src/command/builtin/nox/pull/scan.rs"]
pub mod scan;

#[path = "../../../capsule_terminal/src/command/builtin/nox/pull/ipv4.rs"]
pub mod ipv4;

#[path = "../../../capsule_terminal/src/command/builtin/nox/pull/target.rs"]
pub mod target;

#[path = "../../../capsule_terminal/src/command/builtin/nox/pull/args.rs"]
pub mod args;

#[path = "../../../capsule_terminal/src/command/builtin/nox/pull/http.rs"]
pub mod http;

#[path = "../../../capsule_terminal/src/command/builtin/nox/pull/framing.rs"]
pub mod framing;

#[path = "../../../capsule_terminal/src/command/builtin/nox/pull/recurse.rs"]
pub mod recurse;

#[path = "../../../capsule_terminal/src/command/builtin/nox/pull/verify.rs"]
pub mod verify;

#[path = "../../../capsule_terminal/src/command/builtin/nox/pull/redirect.rs"]
pub mod redirect;

pub mod term {
    pub mod util {
        pub fn format_u64(mut n: u64, out: &mut [u8]) -> usize {
            if n == 0 {
                out[0] = b'0';
                return 1;
            }
            let mut tmp = [0u8; 24];
            let mut i = 0;
            while n > 0 {
                tmp[i] = b'0' + (n % 10) as u8;
                n /= 10;
                i += 1;
            }
            for j in 0..i {
                out[j] = tmp[i - 1 - j];
            }
            i
        }
    }
}

#[path = "../../../capsule_terminal/src/command/builtin/nox/pull/progress.rs"]
pub mod progress;
