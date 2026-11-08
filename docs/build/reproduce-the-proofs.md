# Reproduce the proofs

Every proof NØNOS claims can be run again on your own machine, from the tree, with pinned tools. This page lists the tools, the exact versions, the commands, and what each result means. The commands are the ones the `verify` and `lean` workflows run (`.github/workflows/verify.yml`, `.github/workflows/lean.yml`); if a command here and a workflow disagree, the workflow is right.

## What there is to check

| Layer | Where | Tool and pin | What a pass means |
|---|---|---|---|
| Lean specification | `verification/lean` | Lean 4.15.0, no mathlib | The theorems about capabilities, isolation, the spawn path, page-table permissions, attestation and the power-off wipe check, and none uses `sorry` or an axiom beyond the standard three. |
| Rust to Lean extraction | `verification/extraction` | Charon `nightly-2026.07.02`, Aeneas `nightly-2026.07.06-45061fa`, Rust `nightly-2026-06-01`, Lean `v4.30.0-rc2` with mathlib | The Lean in the tree is exactly what the extraction tools produce from the kernel's own source, and the refinement theorems about it check. |
| Verus | `verification/verus` | Verus `0.2026.06.28.1847ab3`, Rust 1.96.0 | The theorems about capability bits, page-table permission bits and IPC length gates, as restated in that crate, verify. |
| Kani | `userland/*_proofs`, `nonos-attest-path`, `nonos-bootloader/boot_proofs` | Kani 0.67.0 | The model-checking harnesses hold. |
| Host proof crates | `userland/*_proofs` | the pinned Rust nightly | Each crate runs the shipped source through `#[path]` and its tests pass. |

`verification/ASSUMPTIONS.md` and `verification/extraction/ASSUMPTIONS.md` say what these checks take on trust, and [Proving the kernel](../overview/proving-the-kernel.md) says what is and is not proved. A pass on this page is a pass on that scope and no wider.

## What you need

- An x86_64 Linux machine. The workflows use `ubuntu-latest`; the extraction tools are fetched as `linux-x86_64` builds. macOS on Apple silicon can run `nix flake check` (CI does) but not the extraction or Verus jobs described below.
- `git`, `curl`, `python3` (3.10 or newer), `rustup`, `gcc`/`clang` and a few GB of free disk. The extraction job has a 45-minute limit in CI.
- No release keys, and no network after the downloads.

Clone the tree and stay in it:

```sh
git clone https://github.com/NON-OS/microkernel.git
cd microkernel
```

## Everything at once, with Nix

```sh
nix flake check -L --keep-going
```

This builds and runs every proof crate and static check that does not need its own toolchain. It is the first job of the `verify` workflow. Kani, Verus, the Lean specification and the extraction pin their own toolchains and run separately, below.

## 1. The Lean specification

Install `elan`, the Lean toolchain manager, without a default toolchain; the directory's `lean-toolchain` file selects Lean 4.15.0 by itself.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://elan.lean-lang.org/elan-init.sh | sh -s -- -y --default-toolchain none
export PATH="$HOME/.elan/bin:$PATH"

cd verification/lean
lake build                      # builds every theorem; no output beyond "Build completed" means it checks
./proof-corpus-root.sh          # builds again, then refuses unless the axiom profile is clean
```

`proof-corpus-root.sh` fails if `AxiomProfile.lean` shows `sorryAx`, or any axiom outside `propext`, `Classical.choice` and `Quot.sound` (the `Nonos.` names are the project's own definitions). On success it prints the corpus root, the hash the kernel attestation binds to.

At this release the corpus has 201 modules and 1604 theorems (`verification/evidence/EVIDENCE.json`, `lean_specification`).

## 2. Rust to Lean extraction

This is the part that ties the Lean to the code that ships. Charon turns the kernel's own source (included by `#[path]`, never copied) into an intermediate form; Aeneas turns that into Lean; hand-written refinement theorems then say what the generated functions do.

Install the toolchain once:

```sh
# Rust nightly with the compiler internals Charon reads
rustup toolchain install nightly-2026-06-01 --profile minimal \
  --component rustc-dev,llvm-tools-preview,rust-src

# Charon and Aeneas, prebuilt
mkdir -p "$HOME/exttools" && cd "$HOME/exttools"
curl -fsSL -o charon.tar.gz \
  https://github.com/AeneasVerif/charon/releases/download/nightly-2026.07.02/charon-linux-x86_64.tar.gz
tar xzf charon.tar.gz
curl -fsSL -o aeneas.tar.gz \
  https://github.com/AeneasVerif/aeneas/releases/download/nightly-2026.07.06-45061fa/aeneas-linux-x86_64.tar.gz
tar xzf aeneas.tar.gz
export PATH="$HOME/exttools:$HOME/.elan/bin:$PATH"
cd - >/dev/null
```

Then, from the repository root:

```sh
# 1. Regenerate every extracted crate and fail on any difference from the tree.
python3 tools/extraction/regen.py --repo-root .
# last line: "391 crates, 1174 entry points, 0 failures"

# 2. The per-syscall rows of the capability-table refinement are current.
python3 tools/extraction/cap_table_rows.py --check

# 3. Build and check the refinement theorems.
cd verification/extraction/lean
lake exe cache get              # downloads the prebuilt mathlib
lake build 2>&1 | tee /tmp/extraction-build.log
cd ../../..

# 4. Every axiom the proofs rely on must be one the project states.
python3 tools/ratchets/stated_axioms.py --log /tmp/extraction-build.log --root .

# 5. The counts the tree is held to.
python3 tools/ratchets/proven_functions.py
```

What each step tells you:

1. `regen.py` regenerates the Lean from the kernel source and diffs it against the committed files. A difference means the kernel moved and the generated Lean was not refreshed; `--write` updates the files, `--only NAME` does one crate. `crates.json` in `verification/extraction` lists every crate and its entry points.
2. `cap_table_rows.py --check` fails if the generated per-syscall rows of the capability-table refinement no longer match the extracted code, which happens when a syscall or a capability is added without rerunning it.
3. `lake build` is the proof check. A theorem that does not hold is an error here, and the build stops.
4. `stated_axioms.py` reads the axiom profile out of the build log and fails if a theorem uses an axiom that `verification/extraction/ASSUMPTIONS.md` does not state, or any `sorryAx`. The Aeneas standard library carries `sorry` in modules these proofs do not import; the check is what confirms none reaches a theorem here.
5. `proven_functions.py` prints the extracted-function count against a floor that may only rise: at this release 1174 extracted, 1174 carrying a proof, 0 without, 533 with a property beyond "the wrapper is its method".

The pins, the scope and what extraction does not cover are in `verification/extraction/README.md`, `extraction-trusted-base-and.md` and `extraction-scope-honestly-stated.md` in that directory.

## 3. The evidence file

`verification/evidence/EVIDENCE.json` is a machine-readable inventory of all of the above: the counts, the pinned toolchains, the axiom policy. CI regenerates it from the tree and fails if it differs from the committed copy. To check it yourself:

```sh
bash verification/evidence/collect-evidence.sh > /tmp/evidence.json
diff verification/evidence/EVIDENCE.json /tmp/evidence.json && echo same
```

It needs `jq`. The file is an inventory of what is there, not a proof by itself; the green runs above are the proof that it checks.

## 4. Verus

```sh
VERUS=0.2026.06.28.1847ab3
rustup toolchain install 1.96.0-x86_64-unknown-linux-gnu
curl -fsSL -o verus.zip \
  "https://github.com/verus-lang/verus/releases/download/release/${VERUS}/verus-${VERUS}-x86-linux.zip"
unzip -q verus.zip -d "$HOME/verus"
export PATH="$HOME/verus/verus-x86-linux:$PATH"

cd verification/verus
verus --crate-type=lib src/lib.rs
```

The Verus crate mounts no kernel file. It restates the bit operations in its own specifications, so a kernel change does not reach it; `verification/README.md` says so.

## 5. Kani

```sh
cargo install --locked kani-verifier --version 0.67.0
cargo kani setup

for crate in kernel_proofs net_proofs driver_proofs usb_proofs nvme_proofs \
    usb_msc_proofs virtio_blk_proofs virtio_net_proofs e1000_proofs \
    rtl8139_proofs xhci_proofs sync_proofs mechanism_proofs wallet_proofs \
    arch_paging_proofs; do
  ( cd "userland/$crate" && cargo kani --output-format terse )
done
( cd nonos-attest-path && cargo kani --output-format terse )
( cd nonos-bootloader/boot_proofs && cargo kani --output-format terse )
```

## 6. Host proof crates

Each `userland/*_proofs` crate pulls the shipped source in through `#[path]` and runs it as ordinary tests:

```sh
cd userland/fs_proofs && cargo test --release
```

`nix flake check` (above) runs all of them. At this release there are 232 proof crates.

## When something fails

- **`regen.py` reports a difference.** The kernel source changed after the Lean was generated. Run `python3 tools/extraction/regen.py --repo-root . --write`, then rebuild the Lean (step 3) and fix any refinement theorem whose generated definition changed.
- **`lake build` reports an error in a `*Refinement.lean` file.** The generated definition it proves things about changed shape. The error names the theorem; the fix is in that file, never a weaker statement.
- **`stated_axioms.py` names an axiom.** A proof now relies on something not on the list. Either remove the dependency or state the axiom, with the reason, in `verification/extraction/ASSUMPTIONS.md`.
- **A floor drops.** `proven_functions.py` only allows the count to fall when code a proof covered was deleted; `tools/ratchets/proven_floors.py` records each such case.

Open an issue at <https://github.com/NON-OS/microkernel/issues> with the command and the last lines of its output if a step here does not reproduce on a clean machine. That is a bug in this page or in the tree, and we want to know.

## See also

- [Proving the kernel](../overview/proving-the-kernel.md)
- [Tests and proofs: Lean](../contributing/tests-and-proofs-lean.md)
- [Reproducible builds](reproducible-builds.md)
- [The seal](seal.md)
