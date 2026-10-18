# Syscalls: Foreign processes

Part of [Syscalls](syscalls.md).

### Foreign processes

A supervising capsule that holds `ForeignExec`, such as the [Linux
personality](../overview/glossary-linux-personality.md#linux-personality), uses these to host
programs the kernel has not verified. They exist on x86_64 only. On aarch64 and
riscv64 the `foreign` module is built from `foreign_absent.rs` instead
(`src/process/mod.rs:26-30`), and every call in it returns `ERRNO_NOSYS`, -38
(`src/process/foreign_absent.rs:26-33`).

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `MFSP` | `0x5053464D` | `MkForeignSpawn` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Create a process with no capabilities, supervised by the caller. |
| `MFST` | `0x5453464D` | `MkForeignStart` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Give such a process an entry point and make it runnable. |
| `MFWT` | `0x5457464D` | `MkForeignWait` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Wait for a guest to make a refused syscall; take its registers. |
| `MFRP` | `0x5052464D` | `MkForeignReply` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Answer one parked guest with the value its `rax` receives. |
| `MFCX` | `0x5843464D` | `MkForeignContext` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Copy a parked guest's registers out, in `struct sigcontext` order. |
| `MFSG` | `0x4753464D` | `MkForeignSignal` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Answer a parked guest with a context: a signal handler, or its return. |
| `MFIN` | `0x4E49464D` | `MkForeignInterrupt` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Stop a running guest thread at its next timer tick and hand it over parked. |
| `MPMP` | `0x504D504D` | `MkPeerMap` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Back a span of a guest's address space with fresh frames. |
| `MPCP` | `0x5043504D` | `MkPeerCopy` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Copy bytes between the caller and a guest it supervises. |
| `MPPT` | `0x5450504D` | `MkPeerProtect` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Set the protection of pages a guest already has. |
| `MFTH` | `0x4854464D` | `MkForeignThread` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Start a thread inside a guest, sharing its address space. |
| `MPTL` | `0x4C54504D` | `MkPeerTls` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Set the thread pointer a guest thread wakes with. |
| `MFFK` | `0x4B46464D` | `MkForeignFork` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Make a second process holding a guest's register state. |
| `MPUN` | `0x4E55504D` | `MkPeerUnmap` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Take pages back from a guest. |
| `MFEX` | `0x5845464D` | `MkForeignExec` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:69` | Replace the program a parked guest is running. |
| `MLRN` | `0x4E524C4D` | `MkLinuxRunNet` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_device.rs:71` | Ask init to start this Linux run again with Network; only the run role, for its own pid. |

### Crypto

`handle_crypto_random` serves `CRND` in the kernel, at most 4096 bytes a call,
from the generator the entropy capsule seeds, and falls back to the hardware
generator (`src/syscall/dispatch/crypto/random.rs:33-47`). `CKEC` and `CMKY`
also run in the kernel. The hash, AEAD, X25519, HMAC and HKDF calls go to the
crypto capsule, and `map_capsule_error` turns its failures into errnos
(`src/syscall/dispatch/crypto/error.rs:26-43`). [Randomness and
cryptography](../security/randomness-and-cryptography.md#the-crypto-system-calls)
follows each call to the code that serves it.

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `CRND` | `0x444E5243` | `CryptoRandom` | Crypto | `can_crypto` `src/syscall/contract/cap_table/crypto.rs:33` | Up to 4096 random bytes. |
| `CHSH` | `0x48534843` | `CryptoHash` | Crypto | `can_crypto` `src/syscall/contract/cap_table/crypto.rs:33` | Hash a buffer of up to 1 MiB with BLAKE3, SHA-256, SHA-512 or SHA3-256. |
| `CENC` | `0x434E4543` | `CryptoEncrypt` | Crypto | `can_crypto` `src/syscall/contract/cap_table/crypto.rs:33` | Encrypt with ChaCha20-Poly1305 or AES-256-GCM. |
| `CDEC` | `0x43454443` | `CryptoDecrypt` | Crypto | `can_crypto` `src/syscall/contract/cap_table/crypto.rs:33` | Decrypt with ChaCha20-Poly1305 or AES-256-GCM. |
| `CEAD` | `0x44414543` | `CryptoEncryptAad` | Crypto | `can_crypto` `src/syscall/contract/cap_table/crypto.rs:33` | AEAD encrypt with additional data, from a frame. |
| `CDAD` | `0x44414443` | `CryptoDecryptAad` | Crypto | `can_crypto` `src/syscall/contract/cap_table/crypto.rs:33` | AEAD decrypt with additional data, from a frame. |
| `CXPK` | `0x4B505843` | `CryptoX25519Public` | Crypto | `can_crypto` `src/syscall/contract/cap_table/crypto.rs:33` | The X25519 public key for a private key. |
| `CXSH` | `0x48535843` | `CryptoX25519Shared` | Crypto | `can_crypto` `src/syscall/contract/cap_table/crypto.rs:33` | An X25519 shared secret. |
| `CHMC` | `0x434D4843` | `CryptoHmacSha256` | Crypto | `can_crypto` `src/syscall/contract/cap_table/crypto.rs:33` | HMAC-SHA256. |
| `CHKF` | `0x464B4843` | `CryptoHkdfSha256` | Crypto | `can_crypto` `src/syscall/contract/cap_table/crypto.rs:33` | HKDF-SHA256. |
| `CKEC` | `0x43454B43` | `CryptoKeccak256` | Crypto | `can_crypto` `src/syscall/contract/cap_table/crypto.rs:33` | Keccak-256. |
| `CMKY` | `0x594B4D43` | `CryptoMachineKey` | Crypto | `can_crypto` `src/syscall/contract/cap_table/crypto.rs:33` | A 32-byte key bound to this TPM and boot state, named by a label. |
