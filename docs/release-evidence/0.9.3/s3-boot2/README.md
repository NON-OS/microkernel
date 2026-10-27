# S3 boot 2: standard-dev at s3/boot c529b412d1's parent, 2026-10-27

## The image and the machine

- **Image.** `make dev-image PROFILE=standard` of s3/boot (e12f1c6d5f).
  That is s3/cli at 66e5ff20f5:
  - the store's 30 s PROVE wait;
  - ?7728;
  - the relay's b3sum check.
  It also carries s3/security-wip's five recipes: gpg, yara, radare2,
  curl and wget.
- **Keys.** Throwaway keys from tools/nonos-dev-image.
- **Relay.** tools/nonos-mirror-relay inside `nix develop`, holding the
  image's 24 pins.
- **Machine.** QEMU TCG, 8 GiB, 4 vCPUs, intel-iommu, a fresh disk, an
  amnesic session. The steps are in drive.steps.

Terminal runs are private, so a program's output and its exit status are
in the screenshots, not on serial. Every result below was read from the
screenshot named beside it. Each run printed `[LINUX] policy bound:
from=market listing=<its listing> mask=...`, with the mask its recipe
asks.

## Works (booted)

| App | Run | Result | Shot |
|---|---|---|---|
| age | `age -p -a -o /root/a.age /etc/passwd`, passphrase typed twice; `age -d /root/a.age`, typed once | status 0, status 0; prints `root:x:0:0:root:/root:/bin/sh`; **the passphrase is not shown** (?7728, 717a125504) | age.png |
| flac | `flac --force-raw-format ... -o /root/o.flac - < in.raw`; `flac -d -f -o /root/back.raw /root/o.flac` | status 0; `o.flac: done`, status 0. Input from `< f` reaches it: the stdin fault of 2026-10-25 is gone | flac-3.png |
| minisign | `-G -W`; `-S -W ... -m /etc/passwd`; `-V ...` | each status 0; `Signature and comment signature verified` | minisign-3.png |
| git | `git init /root/r`; `git -C /root/r -c user.name=ek -c user.email=ek@nonos commit --allow-empty -m first` | both status 0; `[master (root-commit) 22a8fac] first` | git-2.png |
| vim | `vim -es -c 'put ="hello"' -c wq /root/a.txt`; `vim --version` | both status 0 | gpg-3.png (top) |
| yara | `yara /dev/stdin /etc/passwd < r.yar` (rule matches "root") | `r /etc/passwd`, status 0 | radare2-1.png |
| curl | `curl --version` | status 0, curl 8.14.1; the run asked for the network role (`[EXIT] code=5129556 app.linux.term`, then `app.linux.term.net`) and took 2 m 49 s on TCG | curl-int.png |
| curl | `curl -sS -o /root/e.html https://example.com/` | `curl: (6) Could not resolve host`, status 6. No Nym or Anyone directory is reachable from this container, and the request never went direct | radare2-1.png |

## Fails (booted)

- **gpg.** All three runs exit with status 2 and `gpg: Fatal: can't
  disable core dumps: Operation not permitted`: prlimit64 refused
  RLIMIT_CORE = 0 (gpg-3.png). Fixed in e4ff99ee3c, not booted.
- **lame.** Encoding and decoding both end on signal 11, after four ioctl
  calls and an mmap (minisign-3.png top). The kernel's line reads
  `Segmentation fault: user process accessed invalid address [ADDR]`, with
  the address redacted. Open.
- **radare2.** Corrected 2026-10-27: the first refusal below was the
  driver's fault, not radare2's. serial.log shows `linux radare2` started
  (line 7127) before its install ended (`[LINUX] installed`, line 7817), so
  its program was still held. That refusal is correct. Its words now say
  "still being installed" (f1f7190aeb). The libr_io line is still open.
  As first written:
  - `linux radare2 -q -c iI /bin/busybox`: `/usr/bin/radare2 does not run:
    its proof was refused (its policy is not vouched for)`
    (radare2-1.png);
  - `linux r2 -v`, the same file through its link: `Error loading shared
    library libr_io.so.5.9.0: Permission denied (needed by
    /usr/lib/libr_main.so.5.9.0)`, status 127 (r2b.png).
  Both are open, and radare2 is not listed.
- **git, minor.**
  - `error: cannot run maintenance: No such file or directory` after a
    commit.
  - `git log` started its pager, which drew `~` rows over the Terminal's
    scrollback.

## The machine slowed down (booted)

- From serial line ~7854, about 4 h 20 min into the boot, the logs show
  `[DIAG] ipc.call unanswered vfs_pool`. helix's install then failed:
  `the store never settled`, `socket got no reply`, `no package index`.
- micro's install hung after its community .apk. The relay had sent all
  5,016,347 bytes (relay.log: `200 5016347 ... pinned`).
- No prove job ran in this boot, so the prover is not the cause: boot 1's
  open question is narrowed. The Nym and Anyone retry loop ran the whole
  boot, as in boot 1.
- bat, eza, fzf, ncdu, kakoune, tig, lazygit, nnn, geth, profanity and
  monero were not reached.

## Not done

No proof was asked in this boot, so no app has a measured set from it.
