// NONOS Operating System (AGPL-3.0-or-later)
/*
 * A catalogue written by tools/nonos-qwen-tier.py, under a key made for the
 * test, read as the fetcher reads it: the signature verifies with the
 * fetcher's Ed25519, every entry is a pin, byte for byte and in order, and
 * each file names the NONOS repository first.
 */

use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::process::Command;
use std::{env, fs};

use nonos_ed25519::{pubkey_from_secret, verify, Signature};

use crate::catalogue::admit::admit;
use crate::catalogue::parse::parse;
use crate::catalogue::types::{File, Tier};
use crate::pins::{all, pin_of};

const BASE: &str = "https://repo.nonos.test/qwen";

/* Each written catalogue in its own directory: the tests run at once. */
static RUNS: AtomicUsize = AtomicUsize::new(0);

pub fn written() -> Vec<u8> {
    let run = RUNS.fetch_add(1, Ordering::Relaxed);
    let dir = env::temp_dir().join(format!("model_fetch_proofs_{}_{run}", std::process::id()));
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
            let published = published(&f.name);
            assert_eq!(f.mirrors[0], format!("{BASE}/{}/{published}", t.word));
            assert!(f.mirrors[1].starts_with("https://huggingface.co/Qwen/"));
            assert!(f.mirrors[1].ends_with(&format!("/resolve/main/{published}")));
            assert!(pin_of(&f.volume_name()).is_some());
        }
    }
    assert!(parse(&body[..body.len() - 1]).is_none());
}

/* A pin with a digit that is not lowercase hex decodes to zero and admits nothing. */
#[test]
fn every_pin_decodes_to_a_digest() {
    assert!(all().all(|p| p.sha256 != [0; 32]));
    assert_eq!(crate::hex::hex32(&[b'G'; 64]), [0; 32]);
}

/*
 * The name a file is published and downloaded by: the parts of the larger
 * Coder tiers are kept under their published name less "-instruct", which
 * tools/nonos_qwen_tier/upstream.py undoes for every download.
 */
fn published(kept: &str) -> String {
    let long = ["7b", "14b", "32b"].iter().any(|b| kept.starts_with(&format!("qwen2.5-coder-{b}-q4")));
    if long {
        kept.replacen("-q4_k_m-", "-instruct-q4_k_m-", 1)
    } else {
        kept.to_string()
    }
}

/* The host tool's catalogue is admitted as the fetcher admits it at load. */
#[test]
fn the_catalogue_is_admitted_and_a_name_too_long_to_keep_is_refused_at_load() {
    let blob = written();
    let mut cat = parse(&blob[..blob.len() - 64]).unwrap();
    assert_eq!(admit(&cat), Ok(()));
    assert!(cat.tiers.iter().flat_map(|t| &t.files).all(File::keepable));
    let small = cat.tiers.iter_mut().find(|t| t.word == "small").unwrap();
    small.memory += 1;
    assert_eq!(admit(&cat), Err("the model catalogue gives a tier a memory need its shape does not"));
    let mut cat = parse(&blob[..blob.len() - 64]).unwrap();
    let coder = cat.tiers.iter_mut().find(|t| t.word == "coder-7b").unwrap();
    coder.files[0].name = published(&coder.files[0].name);
    assert_eq!(coder.files[0].name.len(), 52);
    assert_eq!(admit(&cat), Err("the model catalogue names a file the data volume cannot keep"));
}

/* The tiers named by hand: their files fit, and every Coder part does now. */
#[test]
fn the_small_4b_and_coder_files_are_kept() {
    for tier in ["small", "qwen3-0.6b", "qwen3-4b", "coder-7b", "coder-14b", "coder-32b"] {
        let files: Vec<_> = all().filter(|p| p.tier == tier).collect();
        assert!(!files.is_empty(), "{tier}");
        for p in files {
            assert!(crate::pins::keepable(p.name), "{tier}");
            assert!(p.name.len() <= 49, "{tier}");
        }
    }
}
