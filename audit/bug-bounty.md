# NØNOS security bug bounty (draft)

Draft policy for ek to review. Amounts, currency, dates and the final contact are ek's to set.
Items marked **proposed** are suggestions, not commitments the project has made.

## 1. Before you start

- NØNOS is pre-1.0 (`VERSION` 0.9.2). Fixes land on `main`. There is no long-term-support branch.
- **What is not yet shown to work.** Several protections exist in code but have no committed
 evidence of running on a booted system:
 - the bootloader's STARK check of the kernel at boot;
 - the kernel's spawn gate refusing bad capsules on `main`;
 - the kernel's check of the bootloader;
 - LocalSign;
 - the Linux personality.

 Breaking any of these still counts. A report that shows one of them does not do what its code
 says is exactly what this programme pays for.
- **No hardware runs yet.** Every result so far is from QEMU or host tests. A report that only
 reproduces on real hardware is welcome. Please give the machine model, firmware version and TPM
 interface.
- **Secure Boot signing does not exist yet.** "The firmware boots an unsigned loader" is a known
 gap, not a finding (section 3).

## 2. In scope

The same components as the audit scope (`security-scope.md`, section 4):

- **UEFI bootloader and its kernel gate:** `nonos-bootloader/`.
- **Attestation path and trailer parsers:** `nonos-attest-path/`.
- **Boot measurement** (TCG log, Authenticode, boot-root record): `nonos-boot-measure/`,
 `src/security/boot/loader_check/`.
- **Use of the STARK verifier at each gate:** the `nox_verify` call sites in
 `src/security/capsule_attest/`, `src/security/kernel_attest.rs`,
 `nonos-bootloader/src/kernel_verify/self_attest.rs` and `nonos-boot-measure/src/gate/`.
- **Enrollment and signing tools:** `nonos-stark-enroll/`, `nonos-sign/`,
 `tools/nonos-policy-approve`, `tools/nonos-key-ceremony`.
- **Capsule spawn gate and identity:** `src/security/capsule_attest/`,
 `src/kernel_core/process_spawn/`, `src/security/nonos_id_cert/`,
 `src/security/capsule_manifest/`, `src/security/nonos_trust_anchor/`,
 `src/security/attest_registry/`.
- **LocalSign:** `src/security/local_build/`, `src/security/dev_roots/`.
- **TPM and anti-rollback:** `src/security/tpm/`, `nonos-bootloader/src/security/tpm_nv/`,
 `nonos-bootloader/src/security/anti_rollback/`, `nonos-bootloader/src/boot/crypto/rollback/`.
- **Capabilities:** `src/capabilities/`, `src/syscall/caps/`, `src/syscall/contract/`,
 `abi/caps.toml`.
- **Syscall boundary:** `src/syscall/`, `src/usercopy/`, `abi/syscalls.toml`.
- **IPC and hardware broker:** `src/ipc/`, `src/syscall/microkernel/ipc/`,
 `src/hardware/broker/`.
- **Linux personality:** `userland/capsule_linux/`.

**Proposed, at ek's choice:** the STARK verifier itself (`NON-OS/STARKs`, the `nox_verify` crate
at the pinned rev `1b4b5a321f49af3dfe3b1b9f109e6c7032e0196b`), if the STARK lane agrees. A
soundness break there would be critical (section 4).

Test against the latest `main`, on QEMU or your own hardware. Build with scratch keys
(`bash nonos-ci/scratch-trust-bootstrap.sh`). Never test against keys you do not own.

## 3. Out of scope

- **The project's stated assumptions** (`verification/ASSUMPTIONS.md`). Examples:
 - `stated:physical`: bus, JTAG or cold-boot access;
 - `stated:firmware`: firmware that is dishonest before ExitBootServices;
 - `stated:dma-no-iommu`: DMA reach by a driver capsule on a machine without an IOMMU;
 - `stated:hash-collision`: a break of BLAKE3, SHA-2, SHA-3 or Poseidon, unless the break is new
 and practical. That is research, and ek decides case by case.
- **Physical attacks below the TPM boundary**, fault injection, analog side channels, and timing
 channels in code not marked constant-time.
- **Stolen keys.** Attacks that need a compromised trust anchor, release key, publisher key or
 Secure Boot key.
- **Known gaps.** These are open work, listed in and :
 - the bootloader is not signed for Secure Boot;
 - the production `boot_root.approval` is unsigned;
 - CI cannot fetch the STARK dependency.

 A report that turns a known gap into a concrete attack the gap does not obviously imply is in
 scope.
- **The loader's `dev-mode`.** It accepts an unsigned kernel on purpose; the kernel's STARK is
 still required. Showing that a production build can reach it is in scope and critical. The
 kernel has no such mode: every capsule's STARK is checked in every build.
- **Denial of service** through a capability the operator explicitly granted.
- Applications outside the trust path: browser rendering, wallet UI, desktop, media players,
 market front end, the local model. A capability or isolation break reached through one of them
 is in scope.
- Documentation errors, unless the error would lead a user or integrator to an insecure setup.
- `NON-OS/microkernel` (the public repository), the website, and third-party services.
- Findings from automated scanners without a working proof of concept.
- Social engineering, and attacks on project members, infrastructure or accounts.

## 4. Severity

Severity follows what an attacker gains on a production build (`nonos-production`, non-dev
loader, Standard mode or stricter), not the CVSS formula.

### Critical

Code runs that the trust chain should have refused, or ring 0 is taken.

- A capsule is admitted without a valid attestation: wrong measurement, no trailer, a forged or
 altered trailer, or a proof the gate should reject.
- A capsule is admitted with more capabilities than it was enrolled with.
- The bootloader jumps to a kernel that is not enrolled under its root, in any mode, or whose
 Ed25519 or ML-DSA-65 signature does not verify, in any mode other than `Development`.
- A production build can reach `Development` mode.
- The kernel starts userspace although its check of the bootloader refused it or found no
 boot-root record or trailer to check.
- An image older than the TPM rollback floor boots on a machine with a working TPM.
- Ring 3 code gets ring 0 execution, or arbitrary kernel memory read or write, through a syscall,
 IPC, the hardware broker or `src/usercopy`.
- A local build is reported to a remote party as vendor-attested (authority confusion).
- A STARK soundness break in `nox_verify` at the deployed parameters (if that repository is in
 scope).

### High

A boundary between capsules, or between a capsule and a secret, fails.

- A capsule calls a syscall its capability word does not grant.
- One capsule reads or writes another capsule's memory, or maps a device window without a broker
 claim.
- A Linux guest binary leaves the Linux personality capsule, or uses capabilities the capsule was
 not granted.
- TPM-bound secrets leave the TPM policy: the machine key, the device secret, or a PCR 9 binding
 that does not change with the kernel.
- A LocalSign tag is minted without consent, survives a reboot, or verifies on another machine.
- The kernel's loader check reports "measured" for a loader that was not measured. (A "refused"
 that the boot ignores is a known gap.)

### Medium

A protection is weakened without a full bypass, or a gate can be crashed.

- A panic, hang or unbounded read in any gate parser (v3 or v4 trailer, TCG log, PE digest,
 boot-root record, manifest, certificate) that stops a boot or a spawn. The gates must refuse with
 a typed error.
- A capsule can stop the kernel or other capsules without a capability that allows it.
- A rollback floor that can be lowered or skipped, but only with a precondition outside the
 attacker list (for example, an existing TPM fault).
- A cross-capsule information leak that is not a secret (pids, timing of other capsules, store
 sizes that identify a machine).
- A fingerprinting leak from a guest or capsule that the project says it masks.

### Low

- A wrong refusal code, or a log line that says "measured" or "verified" when nothing was checked,
 with no bypass.
- A mismatch between published ABI files (`abi/*.toml`) and the kernel that could mislead a
 toolchain, with no exploit shown. `docs/FINDINGS.md` F1 is already known.
- Hardening gaps with a plausible but unshown path to impact.

### Informational

No reward. Credit at ek's choice: best-practice notes, style, code that is dead but unreachable.

## 5. Rewards

| severity | reward | notes |
|---|---|---|
| Critical | set by ek | |
| High | set by ek | |
| Medium | set by ek | |
| Low | set by ek | |
| Informational | none | credit at ek's choice |

**Proposed rules for ek to confirm:**

- **Currency and payment:** set by ek. If paid in NOX, say so here and link the rewards page.
- **Duplicates:** the first complete report wins.
- **Known issues:** a report of an issue already listed in,
, or `docs/FINDINGS.md` on the day it arrives is a
 duplicate, unless it shows impact the listing does not.
- **Chains:** one reward per root cause, at the severity of the full chain.
- **Quality:** a working proof of concept on scratch keys moves a report up within its tier. A
 committed failing test in the style of the project's proof crates moves it further.
- **Eligibility:** current project members and their contractors are not eligible. ek may set
 further limits.

## 6. Safe harbour

When you research in good faith under this policy:

- we will not take legal action against you or ask anyone else to, for that research;
- we treat it as authorised under computer-misuse and anti-circumvention laws, as far as we can
 make that promise;
- if a third party takes action against you for it, we will say publicly that you acted under
 this policy.

Good faith means:

- you test only on machines and images you own or have permission to use, with keys you made;
- you do not access, change or delete other people's data;
- you do not degrade a service anyone else relies on;
- you stop and tell us as soon as you reach data or keys that are not yours;
- you give us the time in section 7 before you publish.

If you are unsure whether something is allowed, ask first at the address below.

## 7. Reporting and disclosure

**Where to report.** `SECURITY.md` says:

> Report suspected vulnerabilities privately to team@nonos.systems (or
> ek@nonos.systems). Please do not open a public issue for a security bug before
> it has been fixed.

`SECURITY.md` points to the full policy at
`https://github.com/NON-OS/nonos-docs/blob/main/security/reporting.md`. The in-tree copy is
`docs/legacy/security/reporting.md`. Once ek approves this draft, both should point to it.

**What to include** (from `docs/legacy/security/reporting.md`):

- the affected component (kernel, bootloader, a capsule, or the build and signing chain);
- the commit or release it was found on;
- a description and, ideally, a reproduction.

Also useful:

- the build profile and features;
- the loader security mode;
- QEMU or hardware details;
- the serial log.

**Encryption:** not offered yet. **Proposed:** publish a PGP key or another encrypted channel
in `SECURITY.md`.

**Timeline (proposed, for ek to confirm):**

| step | target |
|---|---|
| Acknowledge the report | 3 business days |
| First assessment and severity | 10 business days |
| Fix on `main` for critical and high | 30 days from assessment |
| Fix on `main` for medium and low | 90 days from assessment |
| Advisory and reward | when the fix is on `main` and, if a release exists, in a release |
| Public disclosure by the reporter | 90 days from the report, or earlier by agreement |

- If a fix needs longer, we say why and agree a new date with you before the deadline.
- If a report is being exploited in the wild, either side may disclose sooner, after telling the
 other.

**What happens after a fix**:

- the advisory lists the affected versions and the fixed one;
- a security fix may be released at a raised rollback index, so the bad release can no longer
 boot on a device with a working TPM once it has booted the fix;
- if a key is affected, the key is rotated and the change is described in the advisory.

**Credit:** in the advisory and the release notes, under the name you choose, or anonymously.
