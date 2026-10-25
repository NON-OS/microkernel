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

//! The shipping binary: its three exit codes, its v1 JSON document, its text,
//! and that nothing it could not check ever reads as verified.

mod common;

use common::*;

const VERIFIED_WORDS: [&str; 2] = ["VERIFIED", "verdict: VERIFIED"];

fn never_verified(code: i32, text: &str) {
    assert_ne!(code, 0, "{text}");
    for w in VERIFIED_WORDS {
        assert!(!text.contains(w), "{w:?} in a run that did not verify:\n{text}");
    }
}

#[test]
fn verified_runs_exit_zero_in_every_mode() {
    let (c, v) = json(&[
        "capsule",
        "--root",
        &fx("caps_root.bin"),
        "--image",
        &fx("a.elf"),
        "--caps",
        "0x18",
        &fx("a.trailer"),
    ]);
    assert_eq!((c, v["verdict"].as_str()), (0, Some("verified")), "{v}");
    assert_eq!(v["results"][0]["caps"], "0x0000000000000018");
    assert_eq!(v["results"][0]["slot"], 0);
    let (c, v) = json(&[
        "kernel",
        "--root",
        &fx("kernel_root.bin"),
        "--image",
        &fx("kernel.image"),
        &fx("kernel.trailer"),
    ]);
    assert_eq!(c, 0, "{v}");
    assert_eq!(v["results"][0]["kind"], "kernel");
    let f = |n: &str| top(&format!("nonos-boot-measure/src/tests/fixture/{n}"));
    let (c, v) = json(&[
        "bootloader",
        "--root",
        &f("root.bin"),
        "--image",
        &f("loader.efi"),
        &f("trailer.bin"),
    ]);
    assert_eq!(c, 0, "{v}");
    assert_eq!(v["results"][0]["kind"], "bootloader");
    let (c, v) =
        json(&["transcript", &fx("caps_root.bin.transcript"), &fx("b.trailer"), &fx("a.trailer")]);
    assert_eq!(c, 0, "{v}");
    assert_eq!(v["results"][0]["slot"], 1);
    assert_eq!(v["results"][0]["caps"], "0x0000000000000059");
    assert_eq!(v["transcript"]["slots"], 2);
}

#[test]
fn a_development_root_is_named_as_one() {
    let (c, text) = cli(&[
        "kernel",
        "--root",
        &fx("kernel_root.bin"),
        "--image",
        &fx("kernel.image"),
        &fx("kernel.trailer"),
    ]);
    assert_eq!(c, 0);
    assert!(text.contains("development root, not a release"), "{text}");
    assert!(text.contains("NOT A RELEASE"), "{text}");
    let (_, v) = json(&[
        "kernel",
        "--root",
        &fx("kernel_root.bin"),
        "--image",
        &fx("kernel.image"),
        &fx("kernel.trailer"),
    ]);
    assert_eq!(v["release"], false);
    assert_eq!(v["root"]["release"], false);
    assert_eq!(v["root"]["note"], "development root, not a release");
}

#[test]
fn a_development_trailer_needs_allow_dev_and_is_never_a_release() {
    let base = [
        "capsule",
        "--root",
        &fx("dev_root.bin"),
        "--image",
        &fx("a.elf"),
        "--caps",
        "18",
        &fx("a.dev"),
    ];
    let (c, v) = json(&base);
    assert_eq!(c, 1, "{v}");
    assert_eq!(v["results"][0]["stage"], "development");
    let mut allowed = base.to_vec();
    allowed.push("--allow-dev");
    let (c, v) = json(&allowed);
    assert_eq!(c, 0, "{v}");
    assert_eq!(v["results"][0]["proof"], "path-only");
    assert_eq!(v["release"], false);
    let (_, text) = cli(&allowed);
    assert!(text.contains("no STARK, never a release"), "{text}");
}

#[test]
fn refusals_exit_one() {
    let cases: [Vec<String>; 4] = [
        // a swapped image
        [
            "capsule",
            "--root",
            &fx("caps_root.bin"),
            "--image",
            &fx("b.elf"),
            "--caps",
            "18",
            &fx("a.trailer"),
        ]
        .map(String::from)
        .to_vec(),
        // a changed capability bit
        [
            "capsule",
            "--root",
            &fx("caps_root.bin"),
            "--image",
            &fx("a.elf"),
            "--caps",
            "19",
            &fx("a.trailer"),
        ]
        .map(String::from)
        .to_vec(),
        // the wrong root
        [
            "kernel",
            "--root",
            &fx("caps_root.bin"),
            "--image",
            &fx("kernel.image"),
            &fx("kernel.trailer"),
        ]
        .map(String::from)
        .to_vec(),
        // a capsule trailer offered as the kernel's
        ["kernel", "--root", &fx("caps_root.bin"), "--image", &fx("a.elf"), &fx("a.trailer")]
            .map(String::from)
            .to_vec(),
    ];
    for args in &cases {
        let a: Vec<&str> = args.iter().map(String::as_str).collect();
        let (c, text) = cli(&a);
        assert_eq!(c, 1, "{args:?}\n{text}");
        assert!(text.contains("verdict: FAILED"), "{text}");
        never_verified(c, &text);
        let (_, v) = json(&a);
        assert_eq!(v["verdict"], "failed");
        assert!(v["results"][0]["code"].as_u64().unwrap() >= 300);
    }
}

#[test]
fn a_flipped_or_cut_trailer_file_exits_one() {
    let d = scratch("cli_mutants");
    let honest = bytes(&fx("a.trailer"));
    let mut flipped = honest.clone();
    let n = flipped.len();
    flipped[n / 2] ^= 0x10;
    let cut = &honest[..n - 1];
    for (name, b) in [("flipped", &flipped[..]), ("cut", cut), ("empty", &[][..])] {
        let p = write(&d, name, b);
        let (c, text) = cli(&[
            "capsule",
            "--root",
            &fx("caps_root.bin"),
            "--image",
            &fx("a.elf"),
            "--caps",
            "18",
            &p,
        ]);
        assert_eq!(c, 1, "{name}: {text}");
        never_verified(c, &text);
    }
}

#[test]
fn what_cannot_be_checked_exits_two_and_never_reads_as_verified() {
    let d = scratch("cli_unchecked");
    let short = write(&d, "short.bin", &[0u8; 31]);
    let wide = write(&d, "noncanonical.bin", &[0xff; 32]);
    let not_pe = write(&d, "not_pe.efi", b"MZ but nothing else");
    let caps = fx("caps_root.bin");
    let cases: Vec<Vec<String>> = vec![
        vec![
            "capsule".into(),
            "--root".into(),
            caps.clone(),
            "--image".into(),
            "/nonexistent/x".into(),
            "--caps".into(),
            "18".into(),
            fx("a.trailer"),
        ],
        vec![
            "capsule".into(),
            "--root".into(),
            caps.clone(),
            "--image".into(),
            fx("a.elf"),
            "--caps".into(),
            "18".into(),
            "/nonexistent/t".into(),
        ],
        vec![
            "capsule".into(),
            "--root".into(),
            short,
            "--image".into(),
            fx("a.elf"),
            "--caps".into(),
            "18".into(),
            fx("a.trailer"),
        ],
        vec![
            "capsule".into(),
            "--root".into(),
            wide,
            "--image".into(),
            fx("a.elf"),
            "--caps".into(),
            "18".into(),
            fx("a.trailer"),
        ],
        vec![
            "capsule".into(),
            "--root".into(),
            "/nonexistent/root".into(),
            "--image".into(),
            fx("a.elf"),
            "--caps".into(),
            "18".into(),
            fx("a.trailer"),
        ],
        vec![
            "bootloader".into(),
            "--root".into(),
            caps.clone(),
            "--image".into(),
            not_pe,
            fx("a.trailer"),
        ],
        vec![
            "capsule".into(),
            "--root".into(),
            caps.clone(),
            "--image".into(),
            d.to_str().unwrap().into(),
            "--caps".into(),
            "18".into(),
            fx("a.trailer"),
        ],
        vec!["transcript".into(), "/nonexistent/transcript".into(), fx("a.trailer")],
        // usage errors
        vec![],
        vec!["frobnicate".into()],
        vec![
            "capsule".into(),
            "--root".into(),
            caps.clone(),
            "--image".into(),
            fx("a.elf"),
            fx("a.trailer"),
        ],
        vec![
            "capsule".into(),
            "--root".into(),
            caps.clone(),
            "--image".into(),
            fx("a.elf"),
            "--caps".into(),
            "0x1g".into(),
            fx("a.trailer"),
        ],
        vec![
            "capsule".into(),
            "--root".into(),
            caps.clone(),
            "--image".into(),
            fx("a.elf"),
            "--caps".into(),
            "11112222333344445".into(),
            fx("a.trailer"),
        ],
        vec!["kernel".into(), "--image".into(), fx("kernel.image"), fx("kernel.trailer")],
        vec!["kernel".into(), "--root".into(), caps.clone(), "--image".into(), fx("kernel.image")],
        vec![
            "kernel".into(),
            "--root".into(),
            caps.clone(),
            "--root".into(),
            caps.clone(),
            "--image".into(),
            fx("kernel.image"),
            fx("kernel.trailer"),
        ],
        vec![
            "kernel".into(),
            "--root".into(),
            caps.clone(),
            "--image".into(),
            fx("kernel.image"),
            "--bogus".into(),
            fx("kernel.trailer"),
        ],
        vec!["transcript".into(), fx("caps_root.bin.transcript")],
        vec![
            "transcript".into(),
            fx("caps_root.bin.transcript"),
            "--image".into(),
            fx("a.elf"),
            fx("a.trailer"),
        ],
        vec!["kernel".into(), "--root".into()],
    ];
    for args in &cases {
        let a: Vec<&str> = args.iter().map(String::as_str).collect();
        let (c, text) = cli(&a);
        assert_eq!(c, 2, "{args:?}\n{text}");
        assert!(text.contains("COULD NOT CHECK"), "{args:?}\n{text}");
        never_verified(c, &text);
        let (_, v) = json(&a);
        assert_eq!(v["verdict"], "could-not-check", "{args:?}");
        assert_eq!(v["release"], false);
    }
}

#[test]
fn a_mixed_run_reports_its_worst() {
    let caps = fx("caps_root.bin");
    let tr = fx("caps_root.bin.transcript");
    // verified and could-not-check: could-not-check
    let (c, v) = json(&["transcript", &tr, &fx("a.trailer"), "/nonexistent/t"]);
    assert_eq!((c, v["verdict"].as_str()), (2, Some("could-not-check")));
    // verified, could-not-check and failed: failed
    let (c, v) =
        json(&["transcript", &tr, &fx("a.trailer"), "/nonexistent/t", &fx("kernel.trailer")]);
    assert_eq!((c, v["verdict"].as_str()), (1, Some("failed")));
    assert_eq!(v["results"][2]["stage"], "slot");
    // a transcript for another root than the one given: failed
    let (c, v) = json(&["transcript", &tr, "--root", &fx("kernel_root.bin"), &fx("a.trailer")]);
    assert_eq!(c, 1);
    assert!(v["refusal"].as_str().unwrap().contains("not the root given"));
    let (c, _) = json(&["transcript", &tr, "--root", &caps, &fx("a.trailer")]);
    assert_eq!(c, 0);
}

#[test]
fn a_tampered_transcript_is_refused() {
    let d = scratch("cli_transcript");
    let text = String::from_utf8(bytes(&fx("caps_root.bin.transcript"))).unwrap();
    let swaps = [
        ("caps 0x0000000000000018", "caps 0x0000000000000019"),
        ("slot 1", "slot 2"),
        ("depth 8", "depth 9"),
        ("policy_epoch 1", "policy_epoch 2"),
        ("nonos-policy-transcript v2", "nonos-policy-transcript v1"),
    ];
    for (from, to) in swaps {
        assert!(text.contains(from), "{from}");
        let p = write(&d, "t", text.replacen(from, to, 1).as_bytes());
        let (c, out) = cli(&["transcript", &p, &fx("a.trailer")]);
        assert!(c == 1, "{from} -> {to}: {c}\n{out}");
        never_verified(c, &out);
    }
    let without_root: String =
        text.lines().filter(|l| !l.starts_with("root ")).map(|l| format!("{l}\n")).collect();
    let p = write(&d, "t", without_root.as_bytes());
    assert_eq!(cli(&["transcript", &p, &fx("a.trailer")]).0, 1);
    let p = write(&d, "t", format!("{text}root {}\n", "00".repeat(32)).as_bytes());
    assert_eq!(cli(&["transcript", &p, &fx("a.trailer")]).0, 1, "a second root");
}

#[test]
fn a_trailer_opening_a_padding_slot_is_refused() {
    let d = scratch("cli_padding");
    let mut t = bytes(&fx("a.dev"));
    let last = t.len() - 1;
    t[last] = 0x80; // slot 128: padding in a two-slot tree
    let p = write(&d, "pad.dev", &t);
    let (c, v) = json(&["transcript", &fx("dev_root.bin.transcript"), "--allow-dev", &p]);
    assert_eq!(c, 1, "{v}");
    assert_eq!(v["results"][0]["slot"], 128);
    assert_eq!(v["results"][0]["code"], 305);
}

#[test]
fn json_is_one_well_formed_v1_document_whatever_the_file_names() {
    let d = scratch("cli_names");
    let odd = d.join("a \"quoted\"\\name\twith\nnewline\u{1}.bin");
    std::fs::copy(fx("a.trailer"), &odd).unwrap();
    let odd = odd.to_str().unwrap();
    let (c, v) = json(&[
        "capsule",
        "--root",
        &fx("caps_root.bin"),
        "--image",
        &fx("a.elf"),
        "--caps",
        "18",
        odd,
    ]);
    assert_eq!(c, 0, "{v}");
    assert_eq!(v["results"][0]["trailer"], odd);
    assert_eq!(v["format"], "nonos-proof-verify");
    assert_eq!(v["version"], 1);
    for key in [
        "tool_version",
        "verdict",
        "exit_code",
        "release",
        "root",
        "transcript",
        "refusal",
        "results",
    ] {
        assert!(v.get(key).is_some(), "{key}");
    }
    for key in [
        "trailer",
        "image",
        "kind",
        "slot",
        "measurement",
        "caps",
        "verdict",
        "proof",
        "stage",
        "code",
        "reason",
    ] {
        assert!(v["results"][0].get(key).is_some(), "results[].{key}");
    }
}

#[test]
fn help_and_version() {
    let (c, text) = cli(&["--help"]);
    assert_eq!(c, 0);
    for w in [
        "capsule",
        "kernel",
        "bootloader",
        "transcript",
        "--json",
        "--allow-dev",
        "exit status",
        "could not check",
    ] {
        assert!(text.contains(w), "{w}");
    }
    let (c, text) = cli(&["--version"]);
    assert_eq!(c, 0);
    assert_eq!(text.trim(), format!("nonos-proof-verify {}", env!("CARGO_PKG_VERSION")));
}
