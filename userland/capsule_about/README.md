# capsule_about

## Role

`capsule_about` is the "About" window: the machine's account of itself. It
is an app on `nonos_app_skeleton`, 1000 by 680, with seven sections:
Overview, Proofs, System, Trust, Verify, Display and Licenses
(`src/about/section.rs`). The handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

| Section | What it shows |
|---|---|
| Overview | product, version, the admission badge and the headline tiles |
| Proofs | a live board: is this session attested, and is it anonymous (`src/about/data/proofs/`) |
| System | build (the toolchain is what `rustc -V` said at build time), runtime, memory and uptime, from `MkProcStat` |
| Trust | the capability word the kernel recorded for this window, decoded, and the trust chain |
| Verify | checks the machine can run on itself (below) |
| Display | the framebuffer size from `GraphicsDisplayDimensions`, and the present path the compositor uses |
| Licenses | the AGPL-3.0 text and the third-party licences |

## What Verify and Proofs read

- `MkCapCheck` on its own pid compares the word it holds with the one it
  was built to ask for (`src/about/data/verify/own_mask.rs`).
- `MkAttestStatus` gives the boot verdicts, and `MkAttestEntries` the
  attested capsules with their measurements.
- On opening, `MkAttestDoc` asks once for a TPM-signed attestation
  document over a fresh challenge (`src/about/data/attest_doc.rs`); a
  machine without a TPM shows the refusal as a refusal.
- The Proofs board reads the session's attestation from the kernel, the
  chosen network route from the policy store, and the latest route proof
  from the `attest` service's board, where only the transports post. The
  window asks no transport anything itself. Each half is Holds only when
  everything under it holds, Broken when any piece is broken, and Unknown
  otherwise; Unknown is never drawn as a pass (`src/about/data/proofs/session.rs`).
- A separate card names the claims this window cannot settle and where
  each is settled instead.

## What the window says about itself

- The Overview badge is this window's entry in the attestation registry
  (`src/about/data/admission.rs`): "Verified", drawn as a pass, only for
  an admission under a proof; "Signed, no proof", "Not admitted" and
  "Unknown" (registry unreadable) never are.
- The Overview capability tile and the Trust pills count the word the
  process table holds for this pid (`src/about/data/own.rs`), not the mask
  the build declares; the declared `MASK` is only Verify's yardstick. A
  table that does not list the window says so.
- The process table is read whole (`src/about/data/verify/procs.rs`), sized
  from the kernel's live count, so the Runtime count, the live checks and the
  proof board cover every process. Memory in use is the kernel's total less
  free.
- Display names the path the compositor presents through: the virtio-gpu
  driver when `driver.virtio_gpu0` is registered, otherwise the kernel's
  blit to the firmware framebuffer (`src/about/data/present.rs`).
- An unanswered clock reads "unavailable" on every Uptime row.

## Microkernel contract

- The window, input and frame loop come from `nonos_app_skeleton::run`.
- `MkAttestStatus`, `MkAttestEntries`, `MkAttestDoc`, `MkCapCheck`,
  `MkProcStat` and `MkGetPid` feed the screens above.
- `MkServiceLookup` and `MkIpcCall` reach `policy` and `attest`; a lookup
  of `driver.virtio_gpu0` (no call) tells Display which present path is live.
- `MkTimeMillis` and `MkTimeMonotonic` give the clock and the uptime.

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x80001819`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x0001 | CoreExec | run user code |
| 0x0008 | IPC | policy, attest and the window services |
| 0x0010 | Memory | heap |
| 0x0800 | GraphicsDisplayQuery | the framebuffer size for the Display section |
| 0x1000 | GraphicsSurfaceCreate | its window surface |
| 0x80000000 | AttestRead | the attestation entries and every live capsule's mask from `MkProcStat` |

`Debug` is absent, and the capsule makes no `MkDebug` call. No Network,
FileSystem, Crypto, Driver, Admin or RegisterService capability is
requested: the window draws conclusions from what it reads and never
reaches a transport.

## Privacy and persistence

The capsule reads no user data and writes nothing. The attestation
challenge is fresh each time the window opens.

## Build and verification

- Build: `make nonos-mk-about`; sign: `make nonos-mk-about-sign`. The kernel
  mirror is `src/userspace/capsule_about`.
- `userland/attest_doc_proofs` holds the attestation-document format the
  kernel signs and the Proofs board's verdict rule on the host.
- `userland/apps_proofs` `about_tests` holds the badge rule, the capability
  count and the present path.
