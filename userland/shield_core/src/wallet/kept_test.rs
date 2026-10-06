use super::*;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("nox-kept-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn hand_off(export: &Path, tag: &[u8]) -> Result<(), WalletError> {
    let dir = export.join(HANDOFF);
    std::fs::create_dir_all(&dir).map_err(|_| StoreError::Io)?;
    std::fs::write(dir.join(PROOF), tag).map_err(|_| StoreError::Io)?;
    Ok(())
}

fn proof_of(dir: &Path) -> Vec<u8> {
    std::fs::read(dir.join(PROOF)).unwrap_or_default()
}

#[test]
fn each_new_spend_keeps_the_last_one_in_order() -> Result<(), WalletError> {
    let export = scratch("order");
    assert_eq!(keep_last(&export)?, None, "nothing to keep on a new store");
    for tag in [b"a", b"b", b"c"] {
        keep_last(&export)?;
        hand_off(&export, tag)?;
    }
    let kept = earlier(&export);
    let ids: Vec<&str> = kept.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(ids, ["0", "1"], "the first two are kept, the third is the last");
    let proofs: Vec<Vec<u8>> = kept.iter().map(|(_, d)| proof_of(d)).collect();
    assert_eq!(proofs, [b"a".to_vec(), b"b".to_vec()]);
    assert_eq!(proof_of(&export.join(HANDOFF)), b"c".to_vec());
    std::fs::remove_dir_all(&export).ok();
    Ok(())
}

#[test]
fn a_forgotten_spend_leaves_the_numbering_going_up() -> Result<(), WalletError> {
    let export = scratch("forget");
    for tag in [b"a", b"b", b"c"] {
        keep_last(&export)?;
        hand_off(&export, tag)?;
    }
    if let Some(dir) = earlier_dir(&export, "1") {
        forget(&dir);
    }
    assert!(earlier_dir(&export, "1").is_none());
    assert_eq!(keep_last(&export)?, Some(String::from("1")), "after 0, the highest kept");
    keep_last(&export)?;
    hand_off(&export, b"d")?;
    assert_eq!(keep_last(&export)?, Some(String::from("2")));
    std::fs::remove_dir_all(&export).ok();
    Ok(())
}

#[test]
fn a_folder_without_a_proof_is_not_a_spend() -> Result<(), WalletError> {
    let export = scratch("empty");
    std::fs::create_dir_all(export.join(EARLIER).join("4")).map_err(|_| StoreError::Io)?;
    std::fs::create_dir_all(export.join(EARLIER).join("notes")).map_err(|_| StoreError::Io)?;
    assert!(earlier(&export).is_empty());
    std::fs::remove_dir_all(&export).ok();
    Ok(())
}

#[test]
fn a_spend_its_owner_settles_is_marked() -> Result<(), WalletError> {
    let export = scratch("settle");
    hand_off(&export, b"a")?;
    let dir = export.join(HANDOFF);
    assert!(!settling(&dir));
    mark_settling(&dir)?;
    assert!(settling(&dir));
    keep_last(&export)?;
    let kept = earlier_dir(&export, "0").ok_or(StoreError::Io)?;
    assert!(settling(&kept), "the mark goes with the spend");
    std::fs::remove_dir_all(&export).ok();
    Ok(())
}
