// attest staleness: a module report counts only when it was produced for the
// commit under test. A report from another commit, or with no commit, is stale.

use serde_json::Value;

pub fn report_commit(report: &Value) -> &str {
    report.get("commit").and_then(|c| c.as_str()).unwrap_or("")
}

pub fn is_fresh(report: &Value, commit: &str) -> bool {
    let theirs = report_commit(report);
    !theirs.is_empty() && theirs != "unknown" && theirs == commit
}

pub fn entry(module: &str, report: &Value) -> Value {
    serde_json::json!({ "module": module, "commit": report_commit(report) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::{Report, Status};
    use serde_json::json;

    const HEAD: &str = "69b7f2f55445b52a5cd0b779e11f33785fb0a4b8";

    #[test]
    fn only_the_exact_commit_is_fresh() {
        assert!(is_fresh(&json!({ "commit": HEAD }), HEAD));
        assert!(!is_fresh(&json!({ "commit": "5a44935e326ff4ebf36a5d3564a911d164eb8ccf" }), HEAD));
        assert!(!is_fresh(&json!({ "commit": "69b7f2f55" }), HEAD));
        assert!(!is_fresh(&json!({ "status": "pass" }), HEAD));
        assert!(!is_fresh(&json!({ "commit": "unknown" }), "unknown"));
        assert!(!is_fresh(&json!({ "commit": "" }), ""));
    }

    fn write(root: &std::path::Path, module: &str, commit: &str) {
        let dir = root.join(module);
        std::fs::create_dir_all(&dir).unwrap();
        let v = json!({ "module": module, "commit": commit, "status": "pass", "blocking": true });
        std::fs::write(dir.join(format!("{module}-report.json")), v.to_string()).unwrap();
    }

    #[test]
    fn stale_report_is_not_counted_and_required_goes_missing() {
        let root = std::env::temp_dir().join(format!("nonos-attest-stale-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let head = Report::new("t", true).commit;
        for m in ["build", "supply-chain", "adversarial", "evidence"] {
            write(&root, m, &head);
        }
        write(&root, "trust-chain", "e6a31389c56c40e094c188264ab84b8b1479510a");
        let status = crate::attest::run(root.to_str().unwrap()).unwrap();
        let bytes = std::fs::read(root.join("attest/ci-attestation.json")).unwrap();
        let a: Value = serde_json::from_slice(&bytes).unwrap();
        let _ = std::fs::remove_dir_all(&root);
        let names: Vec<&str> =
            a["modules"].as_array().unwrap().iter().filter_map(|m| m["module"].as_str()).collect();
        assert!(!names.contains(&"trust-chain"));
        assert_eq!(names.len(), 4);
        assert_eq!(a["stale_reports"][0]["module"], "trust-chain");
        assert_eq!(a["stale_reports"][0]["commit"], "e6a31389c56c40e094c188264ab84b8b1479510a");
        assert_eq!(a["missing_modules"], json!(["trust-chain"]));
        assert_eq!(a["blocking_failure"], true);
        assert!(status == Status::Fail);
    }
}
