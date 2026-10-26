# linux.profanity through the dev mirror, booted at 313c78ca0, 2026-10-25

Main at 313c78ca0, which carries Profanity's listing and M-12's development
mirror (9e5416752). Installed from the market, downloaded through
tools/nonos-mirror-relay, and run in the Terminal.

## The image and the machine

- **Image.** `make dev-image` (qemu-dev) of 313c78ca0, so capsule_linux is
  the development capsule set's, with `dev-mirror`. Sealed with the throwaway
  keys tools/nonos-dev-image makes in its own copy of the tree; the
  checkout's .keys/ was never read.
- **Relay.** `python3 tools/nonos-mirror-relay --cache target/mirror-cache`
  on 127.0.0.1:8080 (10.0.2.2:8080 to the guest), fetching through this
  machine's HTTPS proxy, 18 locked apps.
- **Machine.** QEMU without KVM (TCG), `tools/nonos-drive
  --qemu-arg=--fresh --qemu-arg=--lane --qemu-arg=iommu=intel`, fresh disk,
  amnesic session. The steps are in drive.steps.

## What booted (booted)

From serial.log and the screenshots:

    [LINUX-INSTALL] market says ready=1 index=1 url=1 publisher=1 validated=1 arch=1 attest=1, release: pinned
    [LINUX-INSTALL] started, in its release's box linux.profanity
    [LINUX] mirror 10.0.2.2 is on the local network: reached directly
    [LINUX] mirror 10.0.2.2: /alpine/v3.20/community/x86_64/profanity-0.14.0-r1.apk, 414700 bytes
    [LINUX] provenance Verified: index signature and checksums match
    [LINUX] installed

- **The download.** The installer fetched Alpine's two signed indexes and
  every package of Profanity's closure from the relay. The listed package is
  414,700 bytes, the size the lock pins.
- **Provenance.** "provenance Verified" was logged 31 times, once for each
  package fetched. "reached directly" was logged 36 times, every one naming
  10.0.2.2, the development mirror, as M-12 designs: only a dev-mirror build
  takes that path.
- **Profanity ran.** version.png shows `linux profanity --version`:
  "Profanity, version 0.14.0 … OMEMO support: Enabled, OTR support: Enabled
  (libotr 4.1.1)", then "[LINUX] process 2 exited, status 0".
- **Its interface.** ui.png shows `linux profanity -f /etc/nonos-box/p.log`:
  Profanity's own interface, first-run text, status "offline". quit.png
  shows that `/quit` ended it with status 0.
- **Its policy.** Both runs logged "[LINUX] its policy grants NET_ANON; this
  run has no network". The run read the market policy's mask for
  /usr/bin/profanity; NET_ANON is in no default box.

## Not shown, and what it means

- **No refusal is on serial.** The `-f /etc/nonos-box/p.log` run was meant
  to show a write outside FILES_HOME refused. Profanity started and ended
  normally, and no line names a refused write: write refusals are tallied
  in the box report of a `linux --box` run (boxed/notes.rs), and a market
  policy's run printed none. Whether the mkdir under /etc was refused is
  not shown by this boot. An explicit refusal is still owed.
- **A Terminal run has no network.** userland/capsule_linux/src/linux/run_net.rs:
  a run whose box has NET_ANON asks the kernel to start it again as
  app.linux.run.net, and a terminal's run is not started again. So
  Profanity started from the Terminal never reaches the network. It cannot
  connect from there, and docs/using/privacy-apps.md now says so. A launch
  path that gives it the network is not shown here.
- **Nothing lasts past the run.** The home is removed at every exit
  (catalogue Unit 2), so its account and OMEMO keys last one run.
- The system-call set stays UNMEASURED: no proof job asks the prover yet
  (I-B9a).

## attempt1/

The first run of these steps typed `profanity` without `linux`, which the
Terminal answers with "profanity: not found". Its install half is the same
as above. Its serial log and screenshot are kept there.
