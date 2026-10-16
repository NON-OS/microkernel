# Randomness and cryptography: the two generators

Part of [Randomness and cryptography](randomness-and-cryptography.md).

## Two generators

The kernel runs two ChaCha20 generators. The fast generator serves `CryptoRandom` to [capsules](../overview/glossary-boot-root-record.md#capsule), and the kernel generator, `GLOBAL_RNG`, serves the kernel's own keys and nonces. Both take their seeds from `collect_seed_entropy_secure`, which mixes RDSEED and RDRAND with the bytes the `entropy_pool` capsule has sent (`src/syscall/dispatch/crypto/random.rs:24-30`).

```mermaid
flowchart LR
    RDRAND["RDRAND"] --> EP["entropy_pool"]
    EP --> HB["held bytes, health-tested"]
    HB --> SP["seed pool"]
    RDRAND --> SP
    RDSEED["RDSEED"] --> SP
    SP --> FG["fast generator"]
    FG --> CR["CryptoRandom"]
    CR --> CAPS["capsules"]
    SP --> KG["kernel generator"]
    SEED["loader seed"] --> KG
    KG --> KEYS["kernel keys and nonces"]
```

Every seed of either generator hashes the CPU's RDSEED and RDRAND words with the capsule's held bytes, and the loader seed reaches only the kernel generator.

### The fast generator

`fill` keeps a 32-byte ChaCha20 key and a block counter, and answers each request with keystream under an all-zero nonce (`src/security/entropy_capsule/fast.rs:44-68`). After every request it runs `chacha20_block` once more and takes the next key from that block, which the caller never sees, so a key read out of memory later does not reveal earlier output (`src/security/entropy_capsule/fast.rs:69-71`).

It takes its first seed on the first request, and a new one once `RESEED_AFTER`, 1 MiB, has been served since the last (`src/security/entropy_capsule/fast.rs:30`). Each new key is SHA-256 over a domain string, the old key and a `collect_seed_entropy_secure` seed; when no seed can be made it keeps serving on its ratcheted key (`src/security/entropy_capsule/fast.rs:46-58`).

Before it takes its lock, `fill` calls `poll`. That drains the capsule's answers that have landed, then sends a request for `ASK_BYTES`, 256, bytes when one is due, and never waits for the answer (`src/security/entropy_capsule/client/ask.rs:41-73`). One request is out at a time, a reply counts only for that request from the capsule it went to, and a request unanswered for `GIVE_UP_MS`, 2 s, is given up (`src/security/entropy_capsule/client/ask_rule.rs:22-50`).

Each answer goes through the SP 800-90B repetition count and adaptive proportion tests, at a claimed 4 bits per byte: 6 equal bytes in a row, or 62 of a 512-byte window's first byte, is an alarm (`src/crypto/util/rng/entropy/health.rs:23-29`). The first 1024 bytes, and the first 1024 after an alarm, are tested and dropped, and an alarm drops what is held (`src/crypto/util/rng/entropy/accumulator.rs:44-61`). Up to 64 bytes are held until a seed takes them, and they count for at most 16 of its 32, so RDSEED and RDRAND must answer for the rest (`enough`, `src/crypto/util/rng/entropy/credit.rs:21-28`).

`handle_crypto_random` requires the `Crypto` [capability](../overview/glossary-boot-root-record.md#capability), refuses a null buffer or a request of 0 bytes or more than 4096 with `EINVAL`, and fills a kernel buffer from `fill` before copying it out (`src/syscall/dispatch/crypto/random.rs:31-41`). Linux programs reach it only through the [Linux personality](../overview/glossary-linux-personality.md#linux-personality): its `getrandom` hands out at most 256 bytes a call, and its random devices read through `crypto_random` too (`userland/capsule_linux/src/linux/call/thread.rs:47-55`, `userland/capsule_linux/src/linux/file/dev_io.rs:48`).

On x86_64 the capsule's source is RDRAND. Its `fill` checks CPUID leaf 1, ECX bit 30, and gives up a request after 32 failed attempts on one 64-bit word instead of using a weaker source (`userland/capsule_entropy/src/pool/source/x86_64.rs:23-44`). The capsule serves at most `MAX_RANDOM_BYTES`, 4096 bytes, a request, and answers `EMSGSIZE` above that and `EIO` when the source fails (`userland/capsule_entropy/src/server/handlers/getrandom.rs:27-42`). It keeps four counters and no samples (`Pool`, `userland/capsule_entropy/src/pool/types.rs:26-31`). Its reseed operation checks the length, at most 256 bytes, counts the call with `record_reseed` and mixes nothing in (`userland/capsule_entropy/src/server/handlers/reseed.rs:26-39`).

On aarch64, a [preview port](../architectures/aarch64.md), the capsule draws its bytes with `crypto_random`, which is `CryptoRandom` itself (`userland/capsule_entropy/src/pool/source/aarch64.rs:26-27`), so what it sends back is the fast generator's own output, counted like any capsule bytes. `fill` no longer waits for the capsule, so the capsule's own call does not meet a lock its requester holds. This path was not run on aarch64 in this release.

### The kernel generator

`ChaChaRng` is a ChaCha20 generator with a 64-byte block (`src/crypto/util/rng/csprng.rs:26-37`), due for a reseed after `RESEED_INTERVAL`, 1,048,576 blocks or 64 MiB (`src/crypto/util/rng/csprng.rs:39-40`). `init_rng` seeds it from `collect_seed_entropy_secure` (`src/crypto/util/rng/global/init.rs:38-65`). That function hashes four words each from RDSEED and RDRAND, the capsule's held bytes, 16 cycle-counter jitter samples, the stack pointer and a counter into one SHA-256, and fails unless the CPU's words and the capsule's bytes, counted for at most 16, come to 32 (`collect_seed_entropy_secure`, `src/crypto/util/rng/entropy/collect/pool.rs:37-75`).

Byte draws go through `draw`. Once the interval has passed it reseeds with a fresh `collect_seed_entropy_secure` seed hashed together with 32 bytes of its own output, and when the sources cannot answer it keeps its state and asks again on the next draw (`src/crypto/util/rng/global/reseed.rs:26-42`). `random_u64` takes words straight from the generator and never reseeds it (`src/crypto/util/rng/global/generate.rs:70-75`).

On x86_64 the boot seeds it in three steps:

1. `init_core_systems` ends in `init_platform_baseline` (`src/boot/main/core_init/init_core_systems.rs:68`). That logs which CPU source answers, draws the 16-byte boot session nonce with `init_boot_session_nonce`, then the 32-byte [capability token](../overview/glossary-boot-root-record.md#capability-token) key with `init_token_signing_key` (`src/kernel_core/init/platform/baseline.rs:56-60`). The first of these draws finds the generator unseeded and seeds it with `init_rng`.
2. `log_security_status` hands the handoff to `log_entropy`, which applies the loader's seed, when it is not all zeros, with `seed_from_bootloader`, then wipes it from the handoff with `wipe_boot_seed` (`src/entry/security.rs:17-51`). `seed_from_bootloader` XORs the seed with a fresh local seed and the clock, and seeds or reseeds the generator with the result (`src/crypto/util/rng/global/seed.rs:75-113`).
3. `init_core_services` calls `init_rng` again and [stops the boot](../overview/glossary-amnesic-boot.md#boot-stop) if it fails (`src/kernel_core/init/entry/init_core_services.rs:34-36`).

On aarch64 the order differs: `microkernel_init` runs `init_core_services`, and with it `init_rng`, before `init_platform_baseline` draws the nonce and the key (`src/kernel_core/init/entry/microkernel_init.rs:45-64`).

`get_bytes_secure`, which draws the token key and the session nonce (`src/security/boot_session.rs:37-38`), XORs the generator's output with one RDSEED or RDRAND word for every 8 bytes (`src/crypto/random_api/basic.rs:38-42`, `mix_hardware_entropy` in `src/crypto/random_api/hardware_mix.rs:19-44`). The data volume draws a fresh 12-byte nonce for each sector with `fill_random_bytes` (`src/fs/cryptoblock/seal.rs:37-38`), and the PQClean code gets its randomness from the same generator through `nonos_randombytes` (`src/crypto/pqclean_support/mod.rs:37-45`).

The keyring, which makes wallet keys, XORs 32 bytes from `CryptoRandom` with 32 bytes it reads from RDSEED or RDRAND itself, and fails only when both sources fail (`gather_secret`, `userland/capsule_keyring/src/entropy/gather.rs:32-63`). A TLS client draws its X25519 key share, its client random and its session id with `crypto_random` (`client_flight`, `userland/nonos_tls/src/client_flight.rs:19-32`), and its P-256 key share the same way (`generate`, `userland/nonos_tls/src/p256_share.rs:27-30`).
