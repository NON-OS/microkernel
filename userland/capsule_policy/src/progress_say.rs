// NONOS Operating System (AGPL-3.0-or-later)

//! A job's progress, said on serial: its first late ask, and its finish.

use nonos_libc::mk_debug;

use crate::progress::Progress;

/// `[POLICY] <job>: vfs gave no answer in time at pass N; asking again`
pub fn first_late(job: &[u8], pass: u32) {
    say(&[b"[POLICY] ", job, b": vfs gave no answer in time at pass "]);
    number(pass);
    say(&[b"; asking again\n"]);
}

/// `[POLICY] <job>: done after N passes, the first late at pass M` (or
/// `, none late`).
pub fn done(job: &[u8], progress: Progress) {
    say(&[b"[POLICY] ", job, b": done after "]);
    number(progress.passes);
    if progress.first_late == 0 {
        say(&[b" passes, none late\n"]);
        return;
    }
    say(&[b" passes, the first late at pass "]);
    number(progress.first_late);
    say(&[b"\n"]);
}

fn say(parts: &[&[u8]]) {
    for part in parts {
        let _ = mk_debug(part.as_ptr(), part.len());
    }
}

fn number(mut n: u32) {
    let mut digits = [0u8; 10];
    let mut at = digits.len();
    loop {
        at -= 1;
        digits[at] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    say(&[&digits[at..]]);
}
