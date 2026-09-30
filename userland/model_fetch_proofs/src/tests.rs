// NONOS Operating System (AGPL-3.0-or-later)
/*
 * A catalogue written by tools/nonos-qwen-tier.py, under a key made for the
 * test, read as the fetcher reads it: the signature verifies with the
 * fetcher's Ed25519, every entry is a pin, byte for byte and in order, and
 * each file names the NONOS repository first.
 */

use std::path::Path;
use std::process::Command;
use std::{env, fs};

use nonos_ed25519::{pubkey_from_secret, verify, Signature};

use crate::catalogue::parse::parse;
use crate::catalogue::types::{File, Tier};
use crate::pins::{all, pin_of};

const BASE: &str = "https://repo.nonos.test/qwen";

fn written() -> Vec<u8> {
    let dir = env::temp_dir().join(format!("model_fetch_proofs_{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let (seed, key, out) = (dir.join("seed"), dir.join("pub"), dir.join("catalogue.bin"));
    fs::write(&seed, [7u8; 32]).unwrap();
    fs::write(&key, pubkey_from_secret(&[7u8; 32])).unwrap();
    let tool = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/nonos-qwen-tier.py");
    let mut run = Command::new("python3");
    run.arg(tool).args(["catalogue", "--serial", "5", "--nonos-mirror", BASE]);
    run.arg("--out").arg(&out).arg("--seed").arg(&seed).arg("--pubkey").arg(&key);
    assert!(run.status().unwrap().success());
    let blob = fs::read(&out).unwrap();
    fs::remove_dir_all(&dir).unwrap();
    blob
}

#[test]
fn the_host_tool_writes_what_the_fetcher_takes() {
    let blob = written();
    let (body, sig) = blob.split_at(blob.len() - 64);
    let sig = Signature::from_bytes(&sig.try_into().unwrap());
    assert!(verify(&pubkey_from_secret(&[7u8; 32]), body, &sig));
    let mut forged = body.to_vec();
    forged[30] ^= 1;
    assert!(!verify(&pubkey_from_secret(&[7u8; 32]), &forged, &sig));
    let cat = parse(body).expect("the fetcher reads it");
    assert_eq!((cat.serial, cat.base.as_str(), cat.tiers.len()), (5, BASE, 17));
    let row = |t: &Tier, f: &File| (t.word.clone(), f.volume_name(), f.bytes, f.sha256);
    let listed: Vec<_> =
        cat.tiers.iter().flat_map(|t| t.files.iter().map(move |f| row(t, f))).collect();
    let pinned: Vec<_> =
        all().map(|p| (p.tier.to_string(), p.name.to_vec(), p.bytes, p.sha256)).collect();
    assert_eq!(listed, pinned);
    for t in &cat.tiers {
        assert!(t.memory > t.bytes());
        for f in &t.files {
            assert_eq!(f.mirrors[0], format!("{BASE}/{}/{}", t.word, f.name));
            assert!(f.mirrors[1].starts_with("https://huggingface.co/Qwen/"));
            assert!(pin_of(&f.volume_name()).is_some());
        }
    }
    assert!(parse(&body[..body.len() - 1]).is_none());
}
