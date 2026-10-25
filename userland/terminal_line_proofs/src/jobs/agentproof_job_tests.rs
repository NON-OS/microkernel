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

//! The headless run's job: exactly one Terminal line, `linux --box <box>`
//! and at most five words, each from a set with no quote, redirect, pipe or
//! separator; anything else refused, whatever bytes the image holds.

use super::agentproof_job::{line_of, JOB_MAX, JOB_PATH, RUN_WORDS};

fn job(b: &str, r: &str) -> Vec<u8> {
    format!("nonos-agentproof-job 1\nbox {b}\nrun {r}\n").into_bytes()
}

#[test]
fn a_job_is_one_line_the_terminal_runs() {
    assert_eq!(JOB_PATH, b"/linux/usr/share/agentproof/job");
    let line = line_of(&job("strict", "python3 /usr/share/agentproof/agent.py")).unwrap();
    assert_eq!(line, b"linux --box strict python3 /usr/share/agentproof/agent.py");
    let line = line_of(&job("allow:/tmp/out,/home/w", "sh -c x")).unwrap();
    assert_eq!(line, b"linux --box allow:/tmp/out,/home/w sh -c x");
    assert!(line_of(&job("offline", "a b c d e")).is_ok());
    assert_eq!(RUN_WORDS, 5);
}

#[test]
fn every_refusal() {
    let r = |t: &[u8]| line_of(t).unwrap_err();
    assert_eq!(r(b"box strict\nrun sh\n"), "the first line is `nonos-agentproof-job 1`");
    assert_eq!(
        r(b"nonos-agentproof-job 2\nbox strict\nrun sh\n"),
        "the first line is `nonos-agentproof-job 1`"
    );
    assert_eq!(r(b"nonos-agentproof-job 1\nrun sh\n"), "no `box` line");
    assert_eq!(r(b"nonos-agentproof-job 1\nbox strict\n"), "no `run` line");
    assert_eq!(r(b"nonos-agentproof-job 1\nbox strict\nrun sh\nrun sh\n"), "a job has three lines");
    assert_eq!(r(&job("strict", "a b c d e f")), "`run` is at most five words");
    assert_eq!(r(&vec![b'#'; JOB_MAX + 1]), "a job is at most 1024 bytes");
    let word = "a word holds only letters, digits and / . _ - = : , + @ %";
    for bad in
        ["sh -c 'x'", "sh > /x", "a|b", "a;b", "a  b", "$HOME", "a\tb", "\"x\"", "a<b", "`x`"]
    {
        assert_eq!(r(&job("strict", bad)), word, "{bad}");
    }
    for bad in ["strict x", "", "st'rict"] {
        assert!(line_of(&job(bad, "sh")).is_err(), "{bad}");
    }
}
