//! The release bundle: what CI may publish for a tag. It is the flake's
//! reproducible build, unsigned, exactly as `nix build` leaves it in
//! `result/` (or `NONOS_RESULT`), with the public trust roots the tree holds
//! and checksums over all of it. The sealed image is not here: enrollment and
//! signing are ek's step with ek's keys (`nix run .#seal`), and anyone can
//! check that the sealed kernel and loader are these bytes by rebuilding.

use crate::report::{Report, Status};
use crate::sh::capture_stdout;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Files every bundle must hold, relative to the build output.
const BUILT: &[&str] = &[
    "nonos-build.json",
    "nonos.cdx.json",
    "kernel/nonos-kernel",
    "bootloader/nonos_boot.efi",
    "capsules/catalogue.json",
];

/// The public roots the kernel and loader were built against, from the tree.
const ROOTS: &[&str] = &[
    "nonos-data/trust/MANIFEST.sha256",
    "nonos-data/trust/policy/nonos_trust_anchor.policy.bin",
    "nonos-data/trust/policy/zk_capsule_policy_root.bin",
    "nonos-data/trust/policy/kernel_attest_root.bin",
];

/// The whole committed enrollment: every capsule's STARK trailer, manifest
/// and certificate, and every policy root with its transcript and the
/// boot-root record, so the bundle carries the proofs verifier.wasm checks,
/// not only the roots they check against.
const PROOF_DIRS: &[&str] = &["nonos-data/trust/policy", "nonos-data/trust/capsules"];

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("release", true);
    // The assets go to release/ in the tree, where ci-release-artifacts.yml
    // sums and uploads them; only this lane's report goes under `root`.
    let out = Path::new("release").to_path_buf();
    // The roots and proofs are the tree's own, read from where it is checked out.
    let tree = Path::new(".");
    let bundle = out.join("bundle");
    std::fs::create_dir_all(&bundle)?;
    let built =
        PathBuf::from(std::env::var("NONOS_RESULT").unwrap_or_else(|_| "result".to_string()));
    rpt.check(
        "release-build",
        st(built.join("nonos-build.json").is_file()),
        format!("the flake's build output at {}", built.display()),
    );

    let mut rows = Vec::new();
    for rel in files(&built) {
        rows.push(take(
            &built.join(&rel),
            &bundle.join("build").join(&rel),
            &format!("build/{rel}"),
        )?);
    }
    for rel in BUILT {
        if !built.join(rel).is_file() {
            rows.push(row(&format!("build/{rel}"), "missing", 0, "", ""));
        }
    }
    for rel in ROOTS {
        rows.push(take(&tree.join(rel), &bundle.join(rel), rel)?);
    }
    for dir in PROOF_DIRS {
        for rel in files(&tree.join(dir)) {
            let rel = format!("{dir}/{rel}");
            if ROOTS.contains(&rel.as_str()) {
                continue;
            }
            rows.push(take(&tree.join(&rel), &bundle.join(&rel), &rel)?);
        }
    }
    rows.sort_by(|a, b| a["path"].as_str().cmp(&b["path"].as_str()));

    std::fs::write(
        out.join("release-manifest.json"),
        serde_json::to_string_pretty(&rows).unwrap(),
    )?;
    std::fs::write(out.join("SHA256SUMS"), sums(&rows, "sha256"))?;
    std::fs::write(out.join("BLAKE3SUMS"), sums(&rows, "blake3"))?;
    std::fs::write(
        out.join("provenance.json"),
        serde_json::to_string_pretty(&provenance(&rows)).unwrap(),
    )?;
    let packed = Command::new("tar")
        .args(["--sort=name", "--owner=0", "--group=0", "--numeric-owner", "--mtime=@0", "-czf"])
        .arg(out.join("nonos-release-bundle.tar.gz"))
        .arg("-C")
        .arg(&out)
        .arg("bundle")
        .output()?;
    std::fs::write(out.join("tar.log"), join(&packed.stdout, &packed.stderr))?;
    rpt.check("bundle-tar", st(packed.status.success()), "release bundle archive written");
    let complete = !rows.is_empty() && rows.iter().all(|m| m["status"] == "present");
    rpt.check(
        "artifact-manifest",
        st(complete),
        "release manifest holds the whole build, its bill of materials and the public roots",
    );
    rpt.finish(root)
}

/// Every file under the build output, relative to it, in a stable order.
fn files(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if let Ok(rel) = p.strip_prefix(dir) {
                out.push(rel.to_string_lossy().into_owned());
            }
        }
    }
    out.sort();
    out
}

fn take(src: &Path, dst: &Path, name: &str) -> std::io::Result<serde_json::Value> {
    if !src.is_file() {
        return Ok(row(name, "missing", 0, "", ""));
    }
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // A file out of the read-only nix store keeps its mode in the copy; make
    // it writable so a second run can replace it.
    if dst.exists() {
        std::fs::remove_file(dst)?;
    }
    std::fs::copy(src, dst)?;
    let mut perms = std::fs::metadata(dst)?.permissions();
    #[allow(clippy::permissions_set_readonly_false)]
    perms.set_readonly(false);
    std::fs::set_permissions(dst, perms)?;
    let bytes = std::fs::read(src)?;
    Ok(row(
        name,
        "present",
        bytes.len() as u64,
        &sha256(src),
        blake3::hash(&bytes).to_hex().as_ref(),
    ))
}

fn row(path: &str, status: &str, bytes: u64, sha256: &str, b3: &str) -> serde_json::Value {
    serde_json::json!({ "path": path, "status": status, "bytes": bytes, "sha256": sha256, "blake3": b3 })
}

fn sha256(path: &Path) -> String {
    capture_stdout("sha256sum", &[path.to_str().unwrap_or("")])
        .1
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string()
}

fn sums(rows: &[serde_json::Value], field: &str) -> String {
    let mut out = String::new();
    for row in rows {
        if row["status"] == "present" {
            out.push_str(row[field].as_str().unwrap_or(""));
            out.push_str("  ");
            out.push_str(row["path"].as_str().unwrap_or(""));
            out.push('\n');
        }
    }
    out
}

fn provenance(artifacts: &[serde_json::Value]) -> serde_json::Value {
    serde_json::json!({
        "schema": "nonos.release.provenance.v2",
        "commit": capture_stdout("git", &["rev-parse", "HEAD"]).1.trim(),
        "ref": std::env::var("GITHUB_REF").unwrap_or_else(|_| "local".to_string()),
        "run_id": std::env::var("GITHUB_RUN_ID").unwrap_or_else(|_| "local".to_string()),
        "source_date_epoch": std::env::var("SOURCE_DATE_EPOCH").unwrap_or_else(|_| "unset".to_string()),
        "built_by": "nix build (flake.lock pins every input); unsigned, reproducible",
        "sealed_by": "ek, with nix run .#seal; the sealed image is published beside this bundle",
        "artifacts": artifacts,
    })
}

fn join(stdout: &[u8], stderr: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(stdout.len() + stderr.len());
    buf.extend_from_slice(stdout);
    buf.extend_from_slice(stderr);
    buf
}

fn st(ok: bool) -> Status {
    if ok {
        Status::Pass
    } else {
        Status::Fail
    }
}
