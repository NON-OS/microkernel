# capsule_prove

`capsule_prove` is nonos.prove, the anonymous device proof. On the booted OS it proves to a
verifier that this machine started from an enrolled bootloader, runs an enrolled kernel and is an
enrolled device, without saying which bootloader, which kernel or which device. The proof is
`nonos-device-attest`'s, built without std and proven on one core. It is `no_std` and built on
`nonos_app_skeleton`.

## Role

The capsule has two halves:

- **`src/assemble/`, the pure half.** It never calls the kernel. From the verifier's request, the
 registry transcript, the `MkBootSlots` record, the EK answers and the 32-byte secret, it builds
 the `Statement` and the `Witness`, or it returns the `Refusal` that names the failed invariant.
 It also writes and reads the output file. `userland/prove_proofs` mounts these files as they
 ship.
- **`src/app/`, the window.** It calls the kernel and runs one step per tick, so each step shows
 before the next one starts.

The steps, in order:

1. **Memory.** The heap is sized in `_start`, before the first allocation
 (`src/app/memory.rs`). The machine must have `HEAP_MIB + SPARE_MIB` (3072 MiB) free, as
 `MkProcStat`'s header reports it. The prover peaks near 2.6 GB. Below that floor the window
 says so and asks the TPM for nothing.
2. **The request.** The window reads `/prove.request` and shows the verifier and the window.
 Nothing more happens until the person presses Enter.
3. **The registry.** The window reads `/registry.transcript`, gets the EK public area
 (P-256 first, then RSA 2048) and runs `enrolled`. That function recomputes the registry, holds
 its root to the one in the request, and finds this device by `ek_id`. The transcript and the
 registry are then dropped.
4. **The boot slots.** The window calls `MkBootSlots` and reads the record with libc's own
 reader.
5. **The secret.** The window calls `MkDeviceSecret` last, then runs `assemble`. The secret's
 buffer is wiped inside this step.
6. **The proof.** It uses 64 bytes from `crypto_random`. The step is refused when the kernel has
 no entropy. The entropy and the witness are wiped as soon as `prove` returns.
7. **The check.** The output is encoded, read back with `decode`, and checked with `verify`
 against the statement read back. What is saved is exactly what was checked.
8. **Saving.** The output goes to `/proof-<16 hex>` on the data volume. The name is the first
 eight bytes of its SHA-256.

## Capabilities

From `Capsule.mk`:

```make
CAPSULE_REQUIRED_CAPS := 0xC00001879
```

That is CoreExec, IPC, Memory, Crypto, FileSystem, GraphicsDisplayQuery, GraphicsSurfaceCreate,
StreamImport and DeviceSecret. The comment above it gives each bit's use.

The capsule holds no Network capability. The person places the request and the transcript on the
data volume; `nonos.prove.fetch`, the capsule meant to fetch them, does not exist yet. It also
holds no Debug capability.

## Files

All three files sit on the data volume. All integers in them are little-endian. A "word" is a
u64 below p, the STARK field's modulus.

**The request, `/prove.request`** (`src/assemble/request.rs`). It is at most `REQUEST_MAX`
(209) bytes, and it must be exactly 81 + n bytes long.

| offset | bytes | field |
|---|---|---|
| 0 | 8 | `NZKDREQ1` |
| 8 | 8 | the window |
| 16 | 32 | the context nonce: four words |
| 48 | 32 | the device root the verifier accepts: four words |
| 80 | 1 | n, the verifier id's length, 1 to 128 |
| 81 | n | the verifier id, bytes 0x21 to 0x7E |

**The registry transcript, `/registry.transcript`.** This is `Registry::transcript()` text, at
most `TRANSCRIPT_MAX` (192 MiB).

**The proof, `/proof-<16 hex>`** (`src/assemble/output.rs`). It is at most `OUTPUT_MAX`
(1 MiB).

| offset | bytes | field |
|---|---|---|
| 0 | 8 | `NZKDPRF1` |
| 8 | 32 | boot_root |
| 40 | 32 | kernel_root |
| 72 | 32 | device_root |
| 104 | 4 | device_depth, 1 to 20 |
| 108 | 16 | scope, two words |
| 124 | 32 | context |
| 156 | 32 | tag |
| 188 | 4 | L, the proof's length |
| 192 | L | the proof, `nonos-device-attest` wire bytes |

To check a proof, a verifier runs `decode` and then `verify`. It must also compare the roots
with the published ones and the scope and context with its own.

## How the device is keyed

The device is looked up by `ek_id` over the EK's TPM2B_PUBLIC, size field included. These are
the bytes `tpm2_createek` writes and the registrar hands to `MakeCredential`. The registrar must
use the same bytes when it calls `Registry::enroll`.

## Refusals

Every refusal is a `Refusal` variant (`src/assemble/error.rs`). Each one has its own words in the
window (`error_text.rs`). After any refusal nothing is proven, and everything the run made is
dropped.

## Build and test

```sh
cd userland/capsule_prove
cargo check --release --target../x86_64-nonos-user.json \
 -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem
cd../prove_proofs && cargo test
```

Run them inside `nix develop`, which supplies the pinned nightly. `nix flake check` runs
`prove_proofs` as `proofs-prove_proofs`. Both build against `nonos-device-attest`, which takes the
no_std prover from STARKs main at the commit the flake's `starks` input locks (`a41bb8b` at the
time of writing). The stripped release ELF was about 1.7 MB when it was built against `be00975`.

## Image and status

The feature `nonos-capsule-prove` is in `microkernel-desktop-offline`, so every desktop image
carries it. The kernel spawns it on demand only, one instance at a time. The device anonymity proof is built: a real proof has been made and verified on
the host, and the device secret has run on swtpm.

See [the device proof page](../../docs/handbook/trust/device-proof.md).
