// NONOS Operating System (AGPL-3.0-or-later)
//! Every card the kernel treats as a network card is also left out of the
//! Air-Gapped image: its capsule's feature is in `networkFeatures`
//! (tools/nix/config.nix). igc and netvsc were in NICS and not there, so an
//! Air-Gapped build that carried them would have shipped a network driver.

use super::nics::NICS;

fn read(rel: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join(rel)).expect("readable")
}

/// The feature of the catalogue entry whose handle is `name`.
fn feature_of(catalogue: &str, name: &str) -> Option<String> {
    let entry = catalogue.split("\n {").find(|e| e.contains(&format!("\"handle\": \"{name}\"")))?;
    let rest = entry.split("\"feature\": \"").nth(1)?;
    Some(rest.split('"').next()?.to_string())
}

#[test]
fn every_card_is_dropped_from_the_air_gapped_image() {
    let catalogue = read("tools/nix/capsules.json");
    let config = read("tools/nix/config.nix");
    let block = config.split("networkFeatures = [").nth(1).expect("the list");
    let dropped = block.split("];").next().expect("closed");
    for (card, _) in NICS {
        let feature = feature_of(&catalogue, card).unwrap_or_else(|| panic!("{card}: no capsule"));
        assert!(dropped.contains(&format!("\"{feature}\"")), "{card} ({feature}) ships air-gapped");
    }
}
