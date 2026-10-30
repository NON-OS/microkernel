# S3 boot 1: standard-dev at d17da66a6e plus nine recipes, 2026-10-27

Relay log: the relay as last restarted, inside nix develop.

## The image and the machine

- **Image.** `make dev-image PROFILE=standard` of s3/cli, built with the
  nine security recipes of 59e2c2458f: 28 catalogue apps, each with its
  guest policy. It was sealed with tools/nonos-dev-image's throwaway keys.
- **Relay.** tools/nonos-mirror-relay, run from the image's own tree copy
  so that it held all 28 pins, on 127.0.0.1:8080.
- **Machine.** QEMU TCG, 8 GiB, 4 vCPUs, intel-iommu, fresh disk, driven by
  nonos-drive's steps fed one at a time.
- **Setup.** 13 steps, "Also software installed here" at step 11. The
  desktop came up (desk.png).

## age (booted)

- **The relay's first answer failed.** It was run outside `nix develop`,
  with no b3sum, so the pinned request died in its handler. The guest
  logged `GET .../age-1.2.1-r0.apk gave no whole 200 reply` and then
  `package is not the one the market listed`, and the install failed. The
  relay now refuses to start without b3sum (dadc9d8a73).
- **Install, second try.** The log shows:

      [LINUX-INSTALL] started, in its release's box linux.age
      [LINUX] mirror 10.0.2.2: /alpine/v3.20/community/x86_64/age-1.2.1-r0.apk ... pinned (relay)
      [LINUX] installed

- **Workload.** All three runs exited with status 0 (age2.png, age3.png):
  - `linux age -p -a -o /root/a.age /etc/passwd`, with the passphrase typed
    twice;
  - `linux age -d -o /root/back.txt /root/a.age`;
  - `linux age --version` printed 1.2.1.
  Each run printed `policy bound: from=market ... mask=0x108 FILES_HOME TTY`.
- **Found: the passphrase was shown.** It appeared in the scrollback both
  times it was typed (age2.png). x/term's ReadPassword turns ECHO off and
  stays canonical, and only ?7727 (canonical) reached the Terminal. Fixed
  in 80f06ce20c and 900bd7f6f3 (?7728). Not booted yet.
- **Found: age's "Enter passphrase" prompt was not shown.** It writes the
  prompt to /dev/tty. Not yet run down.

## The proof (booted, failed)

- In the Marketplace, selecting age offered "Prove how age ran?". The
  offer was drawn twice (store4.png), which is for S2. Clicking "Prove it"
  did nothing: the button is keyboard-only. `p` then `y` asked for the
  proof.
- The serial log shows:

      [STORE-PROVE] asked: linux.age
      [STORE-PROVE] prover up
      [STORE-PROVE] failed: linux.age, -110
      [PROVE] syscall buckets ok=0bf6001088000001000100000200000008000040000000040014000490040000020000000000000010000000000000000000000000000000000000000000000000

  The store gave up on PROVE after 2 s. The job went on, read the sealed
  run and measured it, so the run was used up and nobody fetched its
  proof. A second ask got EBUSY (-16). The store now waits 30 s
  (58b69bdaff). Not booted yet.
- That buckets line is age's measured set from one run, the encryption,
  but it is not a verdict, because no `held` line followed. It is not
  written into the recipe.

## Everything after the proof (booted, starved)

- **The queued installs.** jq, openssl, ripgrep, sqlite, minisign, flac,
  lame, git, vim and gpg were queued in the Terminal.
  - jq was refused, correctly: `refused member usr/bin/jq: the image
    carries it`. The image already ships jq, openssl, rg and sqlite3, so
    those four recipes are dropped (e166be34c2).
  - Every later install failed with `no package index`. The relay served
    the index (relay.log shows 200), but the guest logged `connect got no
    reply`, `socket got no reply` and `the store never settled`.
- **Why.** From 00:48 on, the guest logged `[DIAG] ipc.call unanswered
  net.sockets` and then `unanswered vfs_pool`, 116 and more, and those
  services stopped answering in time. The prove job was still running and
  never exited. capsule_prove proves on one core, so one job should not
  starve three others. The Nym and Anyone retry loop (no directory is
  reachable from this container) ran the whole time.
  - The cause is open: the job, the retry loop, or host load, since a
    nix build was running beside QEMU.
- **The log stops at 01:21.** The container's disk allowance ran out, and
  QEMU's serial writes after that were lost.

## Files

serial.log, idrive.log (the steps as run), relay.log, and the screenshots
named above.
