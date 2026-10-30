# FILES_HOME: a home that outlives one run

Design note, no code, for the Lead and the privacy session. Each claim is
marked (design) or (host-read: file:line). Paths under capsule_linux are
relative to userland/capsule_linux/src/linux/.

## The finding

- On qemu-dev, a file `linux age -o /home/age/a.age` wrote (exit 0) was "no
  such file" in the next run; `git -C /home/git/r commit` after `git init`
  gave "cannot change to". (booted)
- /home and /root are private prefixes, counted only with FILES_HOME
  (host-read: file/private/rule.rs:23-35; app/family.rs:32-34). A private
  path is stored under /linux-private/<16-hex id> (file/private/names.rs:30,
  46-50; file/root.rs:38-42). Each run draws a new id (private/life.rs:26-28;
  start.rs:54) and removes the root at exit (life.rs:42-44; start.rs:66, 71).
  So the home lives one run. (host-read)
- plan/30-packaging.md:37 promises "its own /home/<app>". (host-read) The
  guest's HOME is /root (env.rs:25), so the home must cover /root. (design)

## Scope

- The home is the app's /home and /root; /tmp, /dev/shm, /run, /var/tmp stay
  per run. No other app sees it, nor is it /home/nonos, which needs
  FILES_USER (host-read: userland/nonos_linux_policy/src/mask.rs:28-30).
  git's exec'd helpers share git's box (app/family.rs:20-21), so its home.

## Identity

- The family id changes every run (names.rs:36-43). app_id hashes program
  and policy (nonos_linux_policy/src/identity.rs:37-43), so it changes per
  release. listing_id is stable across releases (policy.rs:27-30). (host-read)
- Key = hex BLAKE3("NONOS.linux.home.v1" | listing_id). A new release keeps
  the home. Distinct listings give distinct keys. A zero id is never a key.
- An unlisted (O1) program has a zero listing_id but FILES_HOME (host-read:
  nonos_linux_policy/src/unlisted.rs:28-37). Its home is keyed by
  distribution and package name, lives for the boot only, and is never kept
  on disk in any mode. (design)
- Reinstall: after an uninstall the home is gone and starts empty; after a
  reboot that dropped the package, it waits. Dropping on uninstall is safer:
  nothing of an app the person removed stays behind. (design)

## Amnesic boot

- Keeping is off until the policy capsule turns it on; until then a store
  write may only erase (host-read: src/security/zerostate/keep.rs:30-74;
  keep_rule.rs:40-58); vfs refuses a persist with EACCES
  (userland/capsule_vfs/src/server/handlers/persist_gate.rs:37-46).
- The home lives in the RAM store at /linux-home/<key>, outside /linux and
  /linux-private, so the per-run clear leaves it. It lasts the boot. (design)
- Power-off goes only through terminate, which wipes every process's memory
  and page tables, the heap and the key vault (host-read:
  src/security/zerostate/terminate.rs:47-64;
  src/security/hardening/memory_sanitization/api.rs:59-80). capsule_vfs's
  store is process memory, so the wipe takes the home. (host-read, design)
- Never the data volume here. On an installed disk the volume is the disk's
  (src/fs/blockfs_volume/open_machine.rs:66-80), and only
  store_write.rs:58 asks the keep rule; cryptoblock, blockfs and
  blockfs_volume never ask keeping(). (host-read: grep) (design)

## Persistent install

- A Terminal `keep` goes to the package store, unencrypted (host-read:
  docs/using/files.md:112); Linux installs stay in RAM for that reason
  (install/place/keep.rs:50-53; files.md:119). The home must not use it.
- The data volume is sealed per sector with ChaCha20-Poly1305
  (src/fs/cryptoblock/seal.rs:19) under a TPM-derived key never stored
  (open_machine.rs:19-23, 38, 75). Qwen models live there
  (files.md:107-108). (host-read)
- On a boot where keeping() is true, the home lives on the volume at
  homes/<key>/. capsule_linux asks keeping() before it opens the home, as
  vfs does (persist_gate.rs:32-34). (design)
- Uninstall today removes the recorded files, link lines and program record,
  and keeps shared files and dependencies (host-read:
  install/remove/package.rs:40-73). It will also remove homes/<key>/ and
  /linux-home/<key>, and fail as WHY_REMOVE_PARTIAL
  (userland/market_proto/src/reason/removal.rs:26-27) if any stays, so
  "uninstall again" finishes it. (design)

## RAM-only packages, kept home

- After a reboot the package is gone; the home waits, keyed by listing id.
  `linux age` says it is not installed and its home is kept; a reinstall
  from the same listing finds it. The store offers Install or Erase home
  (uninstall's home step). Nothing in a home starts without a proved
  install. (design)

## Bounds

- Today all private prefixes share 16 MiB and 128 names, ENOSPC past that;
  the store has 2048 names and a 320 MiB heap for everyone (host-read:
  file/system/declared/sizes.rs:60-69; file/system/space/quota/allow.rs:24-37).
- RAM home: 16 MiB and 128 names per app, 64 MiB for all RAM homes. Volume
  home: 256 MiB and 4096 names per app, within the volume's free space.
  Scratch keeps its own 16 MiB. (design)
- Full: the guest gets ENOSPC, as allow.rs does now; a shrink or unlink is
  always taken. A full volume today is a plain IoError (host-read:
  src/fs/vfs/map_volume_err.rs:47; src/fs/blockfs/error.rs:31), which would
  be EIO; it must be ENOSPC (file/held/store_err.rs:27). (design)
- The person is told once per run, on [LINUX] and in the Terminal: "<app>'s
  home is full (16 MiB). Remove files, or erase its home." (design)

## Proofs and mutants

- rule.rs is mounted as is by userland/capsule_linux_proofs/src/lib.rs:135-136
  (tests: tests/calls/private_rule_tests.rs). zerostate_proofs mounts
  keep_rule.rs (userland/zerostate_proofs/src/lib.rs:22-23). (host-read)
- Add (design): the key is a pure function of listing_id, zero has none,
  /home and /root map to it and scratch never does; distinct ids give
  disjoint prefixes and no path of app A resolves under B's key; with
  keeping off no home selects the volume; uninstall leaves no name under the
  key; growth past the bound is ENOSPC, a shrink passes.
- Mutants, each must fail a test (design): key from app_id or the random
  id; zero id accepted; volume chosen with keeping off; uninstall skips the
  home; quota check removed; volume write allowed with keeping off.

## Boot plan

1. Persistent: run 1 `age -p -o /root/a.age notes.txt`, run 2 `age -d`;
   compare. Run 1 `git init /root/r` and commit, run 2 `git log`. The same
   under /home/<app>. (design)
2. Reboot: reinstall; both pass again; a disk scan finds no plaintext.
3. Uninstall, reinstall: the home is empty. Age cannot open git's files.
4. Amnesic: runs 1 and 2 pass within the boot. Then
   tools/nonos-zero-state-check with a steps file that writes {secret} into
   the home (host-read: tools/nonos_zero_state/__main__.py:33;
   steps/setup.steps:9): in RAM before Power Off, gone after, absent from
   disk and next boot. scripts/amnesic_boot_check.sh still finds the disk
   unchanged (host-read: lines 18-30). (design)

## Release changes (design)

- capsule_linux: a stable home root beside the random one; RAM or volume by
  keeping(); the quota and its message; uninstall and Erase home.
- Data volume: write, rename, list and remove for homes/ only, reachable by
  capsule_linux; OutOfSpace as ENOSPC; a keeping check on volume writes.
- Zero-state check: a steps file that puts the secret in a Linux home.

## Open questions (design)

- Lead: listing_id alone, or listing plus publisher? Move HOME from /root to
  /home/<app>? The quota numbers? Does an orphan home last until erased?
- Privacy session: may a volume write ever happen with keeping off, for a
  model or anything? This note assumes no. Should a kept home ask for the
  passphrase as the volume may (open_machine.rs:71-74)?

## Rulings, 2026-10-25 (the Lead; Privacy on the volume)

1. Key = BLAKE3("NONOS.linux.home.v1" | listing_key | listing_id). Binding the signing key means a second publisher who names the same listing id gets a different home. It needs a proof that two publishers with one listing_id get disjoint homes, and a mutant that drops the key.
2. HOME stays /root, matching passwd for uid 0. /root and /home/<app> both map to the one home.
3. Quotas: in RAM, 16 MiB and 128 names per app, 64 MiB in all; on the volume, 256 MiB and 4096 names per app. They are declared where check_declared_limits sees them, and written in docs/using.
4. An orphan home lasts until it is erased. Every reboot orphans every home, since packages are RAM-only. The store shows "Erase home" for a listing that has a home and no package.
5. No disk-volume write ever happens with keeping off. The kernel fix is Privacy's: (A) with keeping off, the session volume is in RAM, as on a live stick; (B), the backstop, refuses such a write with persist_gate's answer. homes/<key> on the volume waits for that unit; the RAM home does not.
6. A kept home asks for no passphrase. It inherits the volume's protection.
