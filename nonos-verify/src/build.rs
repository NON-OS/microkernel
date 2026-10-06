// build: compile proof. Formatting, clippy, the production desktop GUI profile,
// section sizes, and a binary symbol scan on the built release ELF. x86_64 is
// blocking; other arches are gaps until a kernel build lane exists.

use crate::report::{Report, Status};
use crate::sh::{capture, have, run_logged};
use std::path::Path;

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("build", true);
    let out = Path::new(root).join("build");
    std::fs::create_dir_all(&out)?;

    // rustfmt on nonos-verify, the crate the engine owns. The kernel tree is
    // not checked here (rustfmt cannot resolve its cfg-gated arch modules), and
    // nonos-sign's formatting is gated by its own repo CI with its own config.
    let f = run_logged(
        "cargo",
        &["fmt", "--manifest-path", "nonos-verify/Cargo.toml", "--", "--check"],
        &out.join("rustfmt-nonos-verify.txt"),
    );
    rpt.check("rustfmt", st(f), "cargo fmt --check (nonos-verify)");

    let ok = run_logged(
        "cargo",
        &[
            "clippy",
            "--manifest-path",
            "nonos-sign/Cargo.toml",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
        &out.join("clippy.txt"),
    );
    rpt.check("clippy-nonos-sign", st(ok), "clippy -D warnings (nonos-sign)");

    // Compile the microkernel-capsules kernel. The signing key and scratch trust
    // anchor are provisioned by the workflow before this runs.
    let ok = run_logged("make", &["nonos-mk-capsules"], &out.join("build-x86_64.txt"));
    rpt.check("build-x86_64-capsules", st(ok), "make nonos-mk-capsules");

    // What ring 0 is, from the dep-info of the kernel just built. It may not
    // grow past its budget; a PR that raises the budget has to say why.
    let tcb = [
        "tools/nonos-tcb",
        "--by-module",
        "--baseline",
        "nonos-ci/baselines/tcb-x86_64-capsules.txt",
    ];
    let ok = run_logged("python3", &tcb, &out.join("tcb-budget.txt"));
    rpt.check(
        "tcb-budget",
        st(ok),
        "ring 0 lines within nonos-ci/baselines/tcb-x86_64-capsules.txt",
    );

    // The share of ring 0 under a theorem over extracted code; may not shrink.
    let proof =
        ["tools/nonos-proof-coverage", "--baseline", "scripts/baselines/proof-coverage.txt"];
    let ok = run_logged("python3", &proof, &out.join("proof-coverage.txt"));
    rpt.check(
        "proof-coverage",
        st(ok),
        "extracted-code theorem lines at or above scripts/baselines/proof-coverage.txt",
    );

    let kbin = "target/x86_64-nonos/release/nonos-kernel";
    if Path::new(kbin).exists() {
        let (_, sz) = capture("size", &[kbin]);
        let (_, se) = capture("readelf", &["-S", kbin]);
        std::fs::write(out.join("section-size-report.txt"), format!("{sz}\n---\n{se}"))?;
        let _ = std::fs::copy(kbin, out.join("nonos-kernel.x86_64"));
        rpt.check("section-size", Status::Pass, "size + readelf -S captured on release ELF");

        let scan = out.join("symbol-scan.txt");
        if Path::new("nonos-ci/scan-microkernel-symbols.sh").exists() {
            let ok = run_logged("bash", &["nonos-ci/scan-microkernel-symbols.sh"], &scan);
            rpt.check(
                "symbol-scan",
                st(ok),
                "binary symbol scan (nonos-ci/scan-microkernel-symbols.sh)",
            );
        } else {
            let nm = if have("llvm-nm") { "llvm-nm" } else { "nm" };
            let (_, syms) = capture(nm, &["--demangle", kbin]);
            let hits: Vec<&str> = syms
                .lines()
                .filter(|l| l.contains("core::panicking") || l.contains("rust_begin_unwind"))
                .collect();
            std::fs::write(&scan, hits.join("\n"))?;
            rpt.check(
                "symbol-scan",
                if hits.is_empty() { Status::Pass } else { Status::Fail },
                "panic-machinery scan on release ELF",
            );
        }
    } else {
        rpt.check("section-size", Status::Skip, "kernel ELF absent (build did not produce it)");
    }

    // aarch64 is built by `make nonos-mk-arm` in ci-build-aarch64 and booted
    // in ci-boot-aarch64; riscv64 has no kernel target and ships in no release.
    let mk = std::fs::read_to_string("mk/20-build.mk").unwrap_or_default();
    let arm = Path::new("aarch64-nonos.json").exists() && mk.contains("\nnonos-mk-arm:");
    rpt.check(
        "build-aarch64",
        if arm { Status::Pass } else { Status::Gap },
        if arm { "lane wired: make nonos-mk-arm (ci-build-aarch64, ci-boot-aarch64)" }
        else { "aarch64-nonos.json or the nonos-mk-arm target is missing" },
    );
    rpt.check(
        "build-riscv64",
        if Path::new("riscv64-nonos.json").exists() { Status::Gap } else { Status::Skip },
        "no riscv64 kernel target; not a release architecture",
    );

    rpt.finish(root)
}

fn st(ok: bool) -> Status {
    if ok {
        Status::Pass
    } else {
        Status::Fail
    }
}
