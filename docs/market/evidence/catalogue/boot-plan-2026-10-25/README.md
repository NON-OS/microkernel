# The catalogue boot plan, booted on the relay image, 2026-10-25

These are docs/market/catalogue-boot-plan.md's checks, booted on a qemu-dev image
of m12/dev-set 4e3fbeffb.

- **Keys.** The image was sealed with tools/nonos-dev-image's throwaway keys.
- **Packages.** Installs came from tools/nonos-mirror-relay on 8080, each held to
  its lock's BLAKE3.
- **Machine.** QEMU without KVM, from a fresh disk each boot, with IOMMU
  enforcing=1.
- **Files.** Each folder holds serial.log, relay.log, plan.txt (what was typed)
  and drive.steps.
- **Status.** All three fail-closed checks are booted. decoy/ and cutoff/ ran
  on a second image, described below.

## What booted (booted)

- **vim-box: fail-closed 2, vim and never gvim.**
  - The closure fetched vim-9.1.0707-r0, libncursesw, musl,
    ncurses-terminfo-base, xxd and vim-common. gvim appears 0 times.
  - The install did not finish. While vim-common unpacked, the install role
    panicked with `memory allocation of 29501714 bytes failed` and exited
    (`[EXIT] code=134 app.linux.install`), with no reason given to the
    person. This is a bug, reported to the developers.
- **age-box and flac-box: fail-closed 3a.**
  - `linux /usr/bin/age-keygen` and `linux /usr/bin/metaflac`: "does not run:
    its proof was refused (its policy is not vouched for)".
- **git-box.**
  - A clone fails closed at git's held https helper: "unable to find remote
    helper for 'https'".
- **All three boxes.**
  - The program the listing names runs, exit 0: `age --version` is 1.2.1,
    `flac --version` is 1.4.3, and git init and config run.
  - The person's /home/nonos files are not visible ("no such file").
  - Writes under /tmp and /home land in the run's own private root
    (capsule_linux file/private/names.rs), never the shared tree.

- **decoy: fail-closed 1, a policy naming another program.**
  - The image was a qemu-dev of linux/install-oom 4933bd606: the heap counters,
    and none of Unit 1's fix. It was built with one local edit that was never
    committed: tools/nonos_market_catalogue/catalogue.py gave linux.age's
    policy age-keygen's program hash.
  - The log shows:
    `[LINUX] refused the market's policy: its policy names another program`,
    `age: took out 2 of 2 file(s)` and `install failed`.
  - market info: "nothing was installed (reason 36)". `linux age`: "command
    not found". See age_refused.png.
  - The same boot installed linux.flac. Its encode from the Terminal's stdin
    (`-` with `< hello.txt`) printed flac's short help, so the Terminal or the
    personality did not hand flac its `-` or its stdin. That is found, and not
    yet run down.
- **cutoff: fail-closed 3b, an install cut off part way, in one session.**
  - Linux installs are kept in RAM until restart, even on an installed disk
    (capsule_linux install/place/keep.rs:50-53; docs/using/files.md). So 05's
    "power off, boot again" can only end in "not found", and it cannot be run
    as written.
  - The cut is booted in one session instead. vim's install died while it
    unpacked vim-common (`memory allocation of 29501714 bytes failed`,
    code 134) after vim itself had landed.
  - `linux /usr/bin/vim --version` and `linux vim --version` both gave "does
    not run: its proof was refused (its policy is not vouched for)": held,
    never the default box. See vim_refused.png.
  - `market uninstall linux.vim` then gave "uninstalled: what its install put
    here is gone".
  - The heap lines show why the unpack died. The install role ran in a
    65536 KiB heap, the run heap, not the 320 MiB install heap. The fix is
    Unit 1 on linux/install-oom.

## Found

That private root is cleared when the run ends, so a home does not last from
one `linux <app>` run to the next. /home/age/a.age, written with exit 0, was
"no such file" in the next run. the plan calls FILES_HOME "its own
/home/<app>", so the developers are to rule on it.

## Not run here

The 17-workload verdict. It needs a `[PROVE] syscall buckets` line, and nothing
asks nonos.prove for a PROVE job until I-B9a, the store's call, lands.
