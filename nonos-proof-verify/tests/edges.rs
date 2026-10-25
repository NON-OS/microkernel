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

//! The edges a mutation run found untested: what the report names, the
//! argument edges, the JSON escaping, the bounds, the slot reader and the
//! transcript reader's lines.

mod common;

use common::*;
use nonos_proof_verify::{
    hex, json_string, opens, read_file, read_file_within, read_transcript, Opens, Refused, MAX_FILE,
};

fn plain_hex(b: &[u8]) -> String {
    b.iter().fold(String::new(), |mut s, x| {
        s.push_str(&format!("{x:02x}"));
        s
    })
}

#[test]
fn the_report_names_the_root_and_measurement_exactly() {
    let (c, v) = json(&[
        "capsule",
        "--root",
        &fx("caps_root.bin"),
        "--image",
        &fx("a.elf"),
        "--caps",
        "18",
        &fx("a.trailer"),
    ]);
    assert_eq!(c, 0);
    assert_eq!(v["root"]["hex"], plain_hex(&root(&fx("caps_root.bin"))));
    let want = plain_hex(blake3::hash(&bytes(&fx("a.elf"))).as_bytes());
    assert_eq!(v["results"][0]["measurement"], want);
    // In a transcript, the slot's own measurement and capabilities.
    let (_, v) = json(&["transcript", &fx("caps_root.bin.transcript"), &fx("b.trailer")]);
    assert_eq!(
        v["results"][0]["measurement"],
        plain_hex(blake3::hash(&bytes(&fx("b.elf"))).as_bytes())
    );
    assert_eq!(hex(&[0x00, 0x0f, 0xa5, 0xff]), "000fa5ff");
}

#[test]
fn the_kind_is_named_even_when_the_trailer_cannot_be_read() {
    for (verb, img, kind) in [("capsule", "a.elf", "capsule"), ("kernel", "kernel.image", "kernel")]
    {
        let root = fx("caps_root.bin");
        let mut a = vec![verb, "--root", &root];
        let image = fx(img);
        a.extend(["--image", &image]);
        if verb == "capsule" {
            a.extend(["--caps", "18"]);
        }
        a.push("/nonexistent/t");
        let (c, v) = json(&a);
        assert_eq!(c, 2);
        assert_eq!(v["results"][0]["kind"], kind);
        assert!(v["results"][0]["measurement"].is_string());
    }
}

#[test]
fn the_text_says_what_kind_of_pass_it_was() {
    let tr = trust("policy/zk_capsule_policy_root.bin.transcript");
    let t = trust("capsules/about.zk_trailer.bin");
    let (c, text) = cli(&["transcript", &tr, &t]);
    assert_eq!(c, 0);
    assert!(text.contains("verdict: VERIFIED, 1 of 1, release root, STARK proofs"), "{text}");
    // The same slot's path lifted out: a development trailer under the release
    // root passes with --allow-dev, and is not a release.
    let d = scratch("edges_lifted");
    let lifted = write(&d, "about.dev", &bytes(&t)[13..13 + 266]);
    let (c, text) = cli(&["transcript", &tr, "--allow-dev", &lifted]);
    assert_eq!(c, 0, "{text}");
    assert!(text.contains("NOT A RELEASE (development trailers, no STARK)"), "{text}");
    let (_, v) = json(&["transcript", &tr, "--allow-dev", &lifted]);
    assert_eq!(v["release"], false);
    assert_eq!(v["root"]["release"], true);
    let (_, text) = cli(&[
        "kernel",
        "--root",
        &fx("kernel_root.bin"),
        "--image",
        &fx("kernel.image"),
        &fx("kernel.trailer"),
    ]);
    assert!(text.contains("NOT A RELEASE (development root)"), "{text}");
}

#[test]
fn a_wrong_release_root_is_named_only_when_its_kind_differs() {
    let kernel_root = trust("policy/kernel_attest_root.bin");
    let capsule_root = trust("policy/zk_capsule_policy_root.bin");
    let run = |r: &str| {
        cli(&["capsule", "--root", r, "--image", &fx("a.elf"), "--caps", "18", &fx("a.trailer")]).1
    };
    let text = run(&kernel_root);
    assert!(
        text.contains("(this root is the kernel attestation root, which holds no capsule slot)"),
        "{text}"
    );
    let text = run(&capsule_root);
    assert!(text.contains("FAILED") && !text.contains("holds no"), "{text}");
}

#[test]
fn a_usage_error_names_its_refusal() {
    let (c, text) = cli(&["kernel", "--image", &fx("kernel.image"), &fx("kernel.trailer")]);
    assert_eq!(c, 2);
    assert!(text.contains("refused: kernel needs --root"), "{text}");
    let (_, v) = json(&["kernel", "--image", &fx("kernel.image"), &fx("kernel.trailer")]);
    assert_eq!(v["refusal"], "kernel needs --root");
}

#[test]
fn options_end_at_the_double_dash() {
    let d = scratch("edges_dashes");
    let odd = write(&d, "-a.trailer", &bytes(&fx("a.trailer")));
    let json_named = write(&d, "--json", &bytes(&fx("a.trailer")));
    let base = ["capsule", "--root", &fx("caps_root.bin"), "--image", &fx("a.elf"), "--caps", "18"];
    let mut a: Vec<&str> = base.to_vec();
    a.extend(["--", &odd]);
    assert_eq!(cli(&a).0, 0, "a trailer whose name starts with a dash, after --");
    // Past "--", "--json" and "--help" are files, not options.
    let mut a: Vec<&str> = base.to_vec();
    a.extend(["--", "--help", "--json"]);
    let (c, text) = cli(&a);
    assert_eq!(c, 2, "{text}");
    assert!(text.starts_with("nonos-proof-verify "), "the text report, not help or JSON: {text}");
    assert!(text.contains("cannot read --help"), "{text}");
    // A lone "-" is a file's name, not an option.
    let mut a: Vec<&str> = base.to_vec();
    a.push("-");
    let (c, v) = json(&a);
    assert_eq!(c, 2);
    assert!(v["refusal"].is_null(), "{v}");
    assert!(v["results"][0]["reason"].as_str().unwrap().contains("cannot read -"));
    let _ = json_named;
}

#[test]
fn the_capability_word_is_strict_hex() {
    let run = |caps: &str| {
        cli(&[
            "capsule",
            "--root",
            &fx("caps_root.bin"),
            "--image",
            &fx("a.elf"),
            "--caps",
            caps,
            &fx("a.trailer"),
        ])
        .0
    };
    for bad in
        ["", "0x", "+18", "-18", "0x+18", "00000000000000018", "0x00000000000000018", "18 ", "1_8"]
    {
        assert_eq!(run(bad), 2, "{bad:?}");
    }
    for good in ["18", "0x18", "0X18", "0000000000000018", "0x0000000000000018"] {
        assert_eq!(run(good), 0, "{good:?}");
    }
    assert_eq!(run("ffffffffffffffff"), 1, "sixteen digits is a word, refused by the path");
}

#[test]
fn the_file_bound_is_inclusive() {
    let d = scratch("edges_bound");
    let p = write(&d, "ten", &[7u8; 10]);
    assert_eq!(read_file_within(&p, 10).unwrap(), vec![7u8; 10]);
    assert!(read_file_within(&p, 9).unwrap_err().contains("larger than 9 bytes"));
    assert!(read_file(d.to_str().unwrap()).unwrap_err().contains("is not a file"));
    // A sparse file past the real bound is refused unread.
    let big = d.join("big");
    std::fs::File::create(&big).unwrap().set_len(MAX_FILE + 1).unwrap();
    assert!(read_file(big.to_str().unwrap()).unwrap_err().contains("larger than"));
}

#[test]
fn json_strings_escape_every_control_and_separator() {
    for c in (0u32..0x20).chain([0x7f, 0x2028, 0x2029]) {
        let ch = char::from_u32(c).unwrap();
        let q = json_string(&ch.to_string());
        let want = match ch {
            '\n' => "\"\\n\"".to_string(),
            '\r' => "\"\\r\"".to_string(),
            '\t' => "\"\\t\"".to_string(),
            _ => format!("\"\\u{c:04x}\""),
        };
        assert_eq!(q, want, "U+{c:04X}");
        assert_eq!(serde_json::from_str::<String>(&q).unwrap(), ch.to_string());
    }
    for raw in [" ", "~", "é", "\u{2027}", "\u{202a}", "\u{80}"] {
        assert_eq!(json_string(raw), format!("\"{raw}\""), "{raw:?} is not escaped");
    }
    assert_eq!(json_string("a\"b\\c"), "\"a\\\"b\\\\c\"");
}

#[test]
fn the_slot_reader_takes_only_a_path_of_the_gates_depth() {
    let dev = bytes(&fx("b.dev"));
    assert_eq!(opens(&dev), Some(Opens { kind: None, index: 1 }));
    let v4 = bytes(&fx("b.trailer"));
    assert_eq!(opens(&v4), Some(Opens { kind: Some(nonos_attest_path::Kind::Capsule), index: 1 }));
    let mut deeper = dev.clone();
    deeper[8] = 7;
    assert_eq!(opens(&deeper), None, "another depth byte, same length");
    let mut longer = dev.clone();
    longer.push(0);
    assert_eq!(opens(&longer), None, "the right depth byte, another length");
    assert_eq!(opens(&dev[..dev.len() - 1]), None);
    let mut all = dev.clone();
    *all.last_mut().unwrap() = 0xff;
    assert_eq!(opens(&all).map(|o| o.index), Some(255));
    for bit in 0..8 {
        let mut one = dev.clone();
        *one.last_mut().unwrap() = 1 << bit;
        assert_eq!(opens(&one).map(|o| o.index), Some(1 << bit));
    }
}

#[test]
fn transcript_errors_name_the_line_and_blank_lines_are_allowed() {
    let text = String::from_utf8(bytes(&fx("caps_root.bin.transcript"))).unwrap();
    let with_blanks = text.replacen("depth 8\n", "depth 8\n\n", 1) + "\n";
    assert!(read_transcript(&with_blanks).is_ok());
    let lines: Vec<&str> = text.lines().collect();
    let at = lines.iter().position(|l| l.starts_with("slot 1 ")).unwrap();
    let bad = text.replacen("slot 1 ", "slot 7 ", 1);
    let e = read_transcript(&bad).unwrap_err();
    assert!(e.starts_with(&format!("line {}: ", at + 1)), "{e}");
    let e = read_transcript(&text.replacen("depth 8", "depth 9", 1)).unwrap_err();
    assert!(e.starts_with("line 4: "), "{e}");
    assert!(read_transcript(&text.replacen("depth 8\n", "depth 8\ndepth 8\n", 1))
        .unwrap_err()
        .contains("given twice"));
    assert!(read_transcript(&text.replacen("depth 8\n", "", 1))
        .unwrap_err()
        .contains("not recorded"));
    assert!(read_transcript(&text.replacen("depth 8\n", "depth 8\nbogus line\n", 1))
        .unwrap_err()
        .contains("not a transcript line"));
    assert!(read_transcript(&format!("{text}slot 2 kernel {}\n", "00".repeat(32)))
        .unwrap_err()
        .contains("after the root"));
}

#[test]
fn every_refusal_has_its_own_stage_code_and_reason() {
    use nox_verify::Refusal;
    let all = [
        Refused::Malformed,
        Refused::PathOnlyRefused,
        Refused::Path,
        Refused::Root,
        Refused::Publics,
        Refused::Stark(Refusal::Proof, Refusal::Proof.reason()),
    ];
    let codes: Vec<u32> = all.iter().map(|r| r.code()).collect();
    assert_eq!(codes, [300, 301, 302, 303, 304, 407]);
    let stages: Vec<&str> = all.iter().map(|r| r.stage()).collect();
    assert_eq!(stages, ["parse", "development", "path", "root", "publics", "stark"]);
    for r in all {
        assert!(!r.reason().is_empty());
    }
    assert_eq!(Refused::Stark(Refusal::Length, Refusal::Length.reason()).code(), 402);
    assert_eq!(
        Refused::Stark(Refusal::Length, Refusal::Length.reason()).reason(),
        "the STARK was refused: the proof's length is out of bounds"
    );
    assert_eq!(
        Refused::Stark(Refusal::Proof, "FRI query 3 does not open").reason(),
        "the STARK was refused: the proof does not verify (FRI query 3 does not open)"
    );
    assert_eq!(Refused::Path.reason(), "the path does not fold to the root for this measurement");
    assert_eq!(Refused::Malformed.reason(), "not a v4 trailer of this kind");
}

#[test]
fn the_transcript_reader_is_strict_on_every_field() {
    let text = String::from_utf8(bytes(&fx("caps_root.bin.transcript"))).unwrap();
    for line in ["depth 8\n", "policy_epoch 1\n", "boot_epoch 1\n"] {
        let e = read_transcript(&text.replacen(line, "", 1)).unwrap_err();
        assert!(e.contains("not recorded"), "{line:?}: {e}");
    }
    let refused = [
        ("depth 8", "depth +8"),
        ("depth 8", "depth 08x"),
        ("policy_epoch 1", "policy_epoch +1"),
        ("boot_epoch 1", "boot_epoch "),
        ("slot 1 ", "slot +1 "),
        ("caps 0x0000000000000059", "caps 0x"),
        ("caps 0x0000000000000059", "caps 0x00000000000000059"),
        ("caps 0x0000000000000059", "caps 59"),
        ("caps 0x0000000000000059", "caps 0x+59"),
    ];
    for (from, to) in refused {
        assert!(text.contains(from), "{from}");
        assert!(read_transcript(&text.replacen(from, to, 1)).is_err(), "{to:?} was accepted");
    }
    // Sixteen digits or fewer is a capability word; the root then decides.
    let short = text.replacen("caps 0x0000000000000059", "caps 0x59", 1);
    assert!(read_transcript(&short).is_ok(), "a short capability word is the same word");
}

#[test]
fn a_tree_holds_one_to_two_hundred_fifty_six_slots() {
    use nonos_proof_verify::{rebuild, Slot};
    let seed = [0x5a; 32];
    assert!(rebuild(&[], &seed).is_err());
    let full: Vec<Slot> = (0..256u32).map(|i| Slot::Capsule([i as u8; 32], u64::from(i))).collect();
    assert!(rebuild(&full, &seed).is_ok());
    assert!(rebuild(&full[..1], &seed).is_ok());
    let mut over = full.clone();
    over.push(Slot::Kernel([0; 32]));
    assert!(rebuild(&over, &seed).unwrap_err().contains("257 slots"));
}

/// Review finding: help or the version anywhere but the command's place, or
/// as an option's value, once exited 0 without checking anything.
#[test]
fn help_and_version_never_stand_in_for_a_verification() {
    let base = [
        "capsule",
        "--root",
        &fx("caps_root.bin"),
        "--image",
        &fx("b.elf"),
        "--caps",
        "0x18",
        &fx("a.trailer"),
    ];
    for extra in ["-h", "--help", "-V", "--version"] {
        let mut a = base.to_vec();
        a.push(extra);
        let (c, text) = cli(&a);
        assert_eq!(c, 2, "{extra} after the command: {text}");
    }
    let (c, _) = cli(&[
        "kernel",
        "--root",
        &fx("kernel_root.bin"),
        "--image",
        "--help",
        &fx("kernel.trailer"),
    ]);
    assert_eq!(c, 2, "--help as the image's name");
    let (c, _) = cli(&[
        "kernel",
        "--root",
        &fx("kernel_root.bin"),
        "--image",
        &fx("kernel.image"),
        "--version",
    ]);
    assert_eq!(c, 2);
    assert_eq!(cli(&["--help"]).0, 0);
    assert_eq!(cli(&["-V"]).0, 0);
}

/// Review finding: a file's name with a line break once added lines to the
/// text report. Each name and reason is now one escaped line.
#[test]
fn a_file_name_cannot_add_a_line_to_the_report() {
    let d = scratch("edges_forged");
    let forged = "x\nVERIFIED      capsule slot 0  b.trailer  (path and STARK)\nverdict: VERIFIED, 1 of 1, release root, STARK proofs\n#";
    let p = d.join(forged);
    std::fs::copy(fx("b.trailer"), &p).unwrap();
    let (c, text) = cli(&[
        "capsule",
        "--root",
        &fx("caps_root.bin"),
        "--image",
        &fx("a.elf"),
        "--caps",
        "18",
        p.to_str().unwrap(),
    ]);
    assert_eq!(c, 1);
    assert_eq!(text.lines().filter(|l| l.starts_with("verdict:")).count(), 1, "{text}");
    assert!(!text.lines().any(|l| l.starts_with("VERIFIED")), "{text}");
    assert!(text.contains("x\\nVERIFIED"), "the name shown escaped: {text}");
    assert_eq!(nonos_proof_verify::shown("a\tb\u{7f}\u{2028}c"), "a\\tb\\u{7f}\\u{2028}c");
    assert_eq!(nonos_proof_verify::shown("plain é"), "plain é");
}

/// Review finding: an argument that is not UTF-8 once panicked (101).
#[test]
fn an_argument_that_is_not_utf8_cannot_be_checked() {
    use std::os::unix::ffi::OsStrExt;
    let bad = std::ffi::OsStr::from_bytes(b"\xff.trailer");
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_nonos-proof-verify"))
        .args(["capsule", "--root", &fx("caps_root.bin"), "--image", &fx("a.elf"), "--caps", "18"])
        .arg(bad)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("not UTF-8"));
}

/// The last mutation run's survivors: the counts in the verdict line, the
/// library's own `run`, the image named, and why a trailer was not read.
#[test]
fn the_report_counts_and_names_what_it_checked() {
    let tr = fx("caps_root.bin.transcript");
    let (c, text) =
        cli(&["transcript", &tr, &fx("a.trailer"), &fx("b.trailer"), &fx("kernel.trailer")]);
    assert_eq!(c, 1);
    assert!(text.contains("verdict: FAILED, 1 of 3 refused"), "{text}");
    let (c, text) = cli(&["transcript", &tr, &fx("a.trailer"), &fx("b.trailer"), "/nonexistent/t"]);
    assert_eq!(c, 2);
    assert!(text.contains("verdict: COULD NOT CHECK, 1 of 3 not checked"), "{text}");
    let (_, v) = json(&["transcript", &tr, "/nonexistent/t"]);
    assert!(
        v["results"][0]["reason"].as_str().unwrap().contains("cannot read /nonexistent/t"),
        "{v}"
    );
    let image = fx("a.elf");
    let (_, v) = json(&[
        "capsule",
        "--root",
        &fx("caps_root.bin"),
        "--image",
        &image,
        "--caps",
        "18",
        &fx("a.trailer"),
    ]);
    assert_eq!(v["results"][0]["image"], image.as_str());
    let req = nonos_proof_verify::Request {
        mode: nonos_proof_verify::Mode::Transcript { file: tr.clone() },
        root: None,
        trailers: vec![fx("a.trailer")],
        allow_dev: false,
    };
    let r = nonos_proof_verify::run(&req);
    assert_eq!(r.verdict(), nonos_proof_verify::Verdict::Verified);
    assert_eq!(r.subjects.len(), 1);
}
