//! A folder without a proof is no spend, and the settling mark goes with its spend.

use super::*;

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
