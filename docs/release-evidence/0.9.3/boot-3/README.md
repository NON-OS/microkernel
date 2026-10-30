# boot 3: standard-dev of boot 670529d6c7, 2026-10-27

## The image and the machine

- **Image.** `make dev-image PROFILE=standard` of boot 670529d6c7: cli
  at 6ee89b40ce, with main merged, the prlimit fix c5987c08f7 and the
  store's 30 s PROVE wait. It also carries four recipes: gpg, yara, curl
  and wget. Sealed with throwaway keys.
- **Relay.** tools/nonos-mirror-relay inside `nix develop`, holding 24
  pins.
- **Machine.** QEMU TCG, 8 GiB, 4 vCPUs, intel-iommu and the usb-tablet
  lane, a fresh disk, an amnesic session. Terminal results were read from
  the screenshots named beside them.

## The store's proof (booted, failed again)

- age encrypted `/etc/passwd`, status 0 (age1.png).
- In the Marketplace, age was selected and the proof asked (store-age.png).
  The serial log shows:

      [STORE-PROVE] asked: linux.age
      [STORE-PROVE] prover up
      [STORE-PROVE] failed: linux.age, -110
      [PROVE] syscall buckets ok=0bf601118800000100010000020000000800004000000004001400049006000002000002280000...

- The store gave up on PROVE at 30 s. The job printed its buckets 26 s
  later: it first claims a 2816 MiB heap, which took about a minute here.
  - The store now waits 180 s (115e2828a7).
  - The `[STORE-PROVE]` lines are now whole, which eb3f2c0d1c fixed and
    this boot shows.
- The buckets line is age's measured set for that run. It is not a
  verdict, since no `held` line followed.

## gpg (booted)

- With the prlimit fix, gpg passes "disable core dumps" (gpg-2.png):
  - `gpg --batch --passphrase x --pinentry-mode loopback -c -o
    /root/p.gpg /etc/passwd`: `error running '/usr/bin/gpg-agent':
    probably not installed`, status 2. Alpine's gpg does not pull
    gpg-agent.
  - `gpg ... -d /root/p.gpg`: `encrypted with 1 passphrase`, `root:x:0:0:...`,
    status 0.
  - Also: `WARNING: unsafe permissions on homedir '/root/.gnupg'`. Modes
    last one run (file/held/modes.rs), and the home lasts longer.

## monero (booted, the driver's fault)

- The install took over an hour: 24.7 MB over the dev mirror's direct
  path, while the prove job ran beside it.
- The driver typed the wallet commands before `[LINUX] installed`, so
  they were answered `does not run: its proof was refused (its policy is
  not vouched for)` (mon-5.png). The program was still held, which is
  right; the wording is now "still being installed" (6541f0cf83).
- The language prompt's first "1" was not reached in this boot.

## Files

serial.log, drive.steps, idrive.log, relay.log, and the screenshots named.
