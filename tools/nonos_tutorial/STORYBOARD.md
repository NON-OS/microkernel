# NØNOS 0.9.3: the film

A product film of NØNOS 0.9.3, cut from one recording of a real boot. Each
chapter is a story a person cares about, told by the machine doing it. It
claims nothing the screen does not show.

- **Recording.** `tour.steps`, filmed by `tools/nonos-drive --record` on a
  KVM host (README.md). Captions are burned in as the steps run.
- **Cut.** `post/film.json`, rendered by `python3 tools/nonos_tutorial/post`.
  It adds cards, cross-fades between chapters, eased zooms, lower-thirds and
  sped-up waits marked with their speed, then writes a 1080p60 master and a
  1080p30 web cut with chapter marks.
- **Length.** About 12 minutes with every chapter, and about 9 without the
  three that wait on main.

## The copy

- Short sentences in the present tense. Say what the machine does, not what
  it is.
- Every caption names something on screen at that moment. The tour's
  captions were checked against the code, the docs and screenshots of a
  boot (README.md says how).
- Anything not shown is not said. Never "secure" or "unbreakable". Say
  what is refused, what is checked, or what never leaves. A word a screen
  itself uses, such as "anonymous" on About's Proofs page, may be quoted.
- One line, at most 80 characters. `test_record.py` holds every caption to
  that.

## The look

- **Colour.** The desktop's own palette
  (`userland/capsule_desktop_shell/src/render/palette.rs`): BAR `#04080C`
  behind the cards, PANEL `#101A22` under lower-thirds, ACCENT `#22C3F0`
  for rules, labels and speed tags, and TEXT `#E6F1F6` and TEXT_DIM
  `#7A8D99` for words.
- **Type.** Geist SemiBold for titles, Geist for lines, and JetBrains Mono
  for labels and speed tags (`userland/assets/fonts`). These are the
  desktop's own faces.
- **Captions.** White Geist SemiBold at 40 px on a translucent dark box,
  centred just above the dock. A window opened alone sits above it.
- **Cards.** The title, one per chapter and an end card. Each has a
  vignette, a mono label ("CHAPTER 03"), the title, a line under it, and an
  accent rule that draws itself out. The words fade up.
- **Motion.** Chapters cross-fade over 0.8 s. A zoom eases in over 0.9 s
  onto the line that matters, holds, and eases out. A lower-third slides in
  from the left and fades out. A wait plays at ×8 with the tag in the top
  right corner.
- **The boot menu.** It is drawn at 1280x800, and the recording stretches
  it to 1920x1080. The cut puts it back to 16:10, with bars.

## Title: NØNOS 0.9.3

Label: THE TOUR. Line: "Private by default." The card holds for 4 s, then
cross-fades into chapter 1.

## 1. Every boot checks itself

*Before anything runs, the loader proves the kernel.* 1:10. Rehearsed.

| Beat | On screen | Caption |
|---|---|---|
| 1 | Power-on, then the firmware | This is NØNOS 0.9.3, from the moment the machine is switched on. |
| 2 | The boot menu: Esc holds the countdown | The boot menu. Esc holds its countdown, so we can read each entry. |
| 3 | Standard selected | Standard boots when the kernel is signed, attested and current. |
| 4 | Hardened: REFUSED HERE: NO SECURE BOOT, PK, DB | Hardened also needs Secure Boot and a TPM. Here it says it would refuse. |
| 5 | Air-Gapped | Air-Gapped starts no network driver and no network service at all. |
| 6 | Recovery | Recovery is a Terminal and Files, with no network and no setup. |
| 7 | Back to Standard, Enter | We start Standard. |
| 8 | The loader's ten checks | The loader measures the kernel and checks its two signatures before it runs. |
| 9 | The splash's boot-chain panel | The splash lists each link of the boot chain, and says what did not pass. |

Post: 16:10 restored on the menu and the loader. A zoom onto the Hardened
verdict. A lower-third, "Verified boot / Ed25519 and ML-DSA-65, then the
hand-off". A zoom onto the splash's attestation panel.

## 2. No account. Thirteen answers.

*First-boot setup, by keyboard, with no password.* 3:00. Rehearsed.

Every step of setup, each with one or two captions: the keyboard, the
name, the time zone, the Mode step (Amnesic, USB live and Install), the
network (no network is the default), the route (Nym or Anyone), privacy,
appearance, the Qwen model, apps, installed software, the computer's name
and review. The words are in `tour.steps`, section 2.

Post: one zoom on the Mode step's "RAM only. Nothing is written to any
disk", and one on Anyone's description.

## 3. Your computer forgets you

*Amnesic by default: RAM only, no disk written.* 0:40. Rehearsed.

| Beat | On screen | Caption |
|---|---|---|
| 1 | The live-session notice on the new desktop | A live session: nothing is kept. Files, settings and models go when it stops. |
| 2 | `write notes.txt ...`, `cat notes.txt` in the Terminal | A note, in memory. An amnesic boot writes no disk: at power off it is gone. |

Chapter 12 closes the story: the power-off wipes memory, and the note is
gone with it.

Post: a zoom onto the notice's text. Beat 2 is cut in from the Terminal
part of the recording.

## 4. A desktop that shows its work

*Every window says where it came from.* 1:30. Rehearsed.

The dock and the menu bar. The status area (no battery, the network mark,
search, the clock). The first menu, with About this system and Power Off.
The Launchpad. Files. Settings ("never sent on a network"). About, with
its Verified badge, and its Proofs page. On a development image, Proofs
answers "Not attested: a check below failed", and a dev-image caption
says so: its boot chain is development, not release. A sealed image's
answer has not been filmed yet, so the sealed cut says only the question.

Post: a lower-third, "Settings / Names never sent on a network". A zoom
onto About's Verified badge.

## 5. Who can reach what

*The kernel's own account, not the apps'.* 1:00. Rehearsed.

| Beat | On screen | Caption |
|---|---|---|
| 1 | Ctrl+Alt+T: the Terminal's first tab | The Terminal opens with Ctrl+Alt+T, from anywhere on the desktop. |
| 2 | The splash: Standard boot, signed by vendor | A new tab says what this is: a Standard boot, and who signed the Terminal. |
| 3 | `help` | help lists every command, in groups. |
| 4 | `audit` and its counts | audit: which capsules can reach the network, storage and devices, right now. |
| 5 | The same | Every word comes from the kernel, not from what a capsule says about itself. |
| 6 | `log VT-D` | log VT-D: the kernel's own lines about the IOMMU, which fences device memory. |
| 7 | "blocked until claimed", "confined to its capsule's domain" | Each device is blocked until its driver claims it, then kept in that domain. |

Post: a zoom onto audit's counts, and one onto the VT-D lines.

## 6. Apps in a box

*What a Linux app may do, and what it may not.* 1:10. Rehearsed.

| Beat | On screen | Caption |
|---|---|---|
| 1 | `linux perl -e 'print ...'` | A Linux program: Perl, run by NØNOS's Linux personality, in a box of its own. |
| 2 | `[LINUX] policy bound: ... FILES_HOME EXEC TTY` | Its policy line: its own files, starting programs, the terminal. No network. |
| 3 | A ptrace attach | Now it tries to attach to another process with ptrace, as spyware would. |
| 4 | `refused ptrace: a guest does not inspect or steer another` | Refused: a guest does not inspect or steer another. The refusal is counted. |
| 5 | A connect to 1.1.1.1:443 | And it tries to reach the internet directly. |
| 6 | The refusal | Refused too: its policy grants it no network. |
| 7 | `audit --agent app.file_manager` | audit for one app: Files can reach storage, and cannot reach the network. |

Post: a lower-third, "Linux personality / Each program held to its own
policy". A zoom onto the policy line and onto each refusal.

## 7. A store that checks every byte

*Signed, pinned, and proved on this machine.* 1:30. Needs the network; on a
development image, also the host's relay. Rehearsed on the development
image, through the relay.

Beats: the Marketplace window and its signed catalogue. swayimg's card with
every gate passed. Install: on a development image the download comes from
the host's relay, and on a sealed image it goes over Anyone. A marked
caption says which. "Installed, and held in memory until restart." Then
swayimg opens in its own window, and zooms on a key.

Post: a zoom onto the gates. The download plays at ×8 with its tag. A
lower-third, "swayimg / A Wayland app in its box: no network".

## 8. Talk without being watched

*Chat that leaves only over Nym or Anyone.* 1:00. Needs listings on main.
Scripted, not rehearsed.

- **SimpleX Chat** needs the listing `linux.simplex-chat` on main. The recipe kind exists
  and it needs a static build. Its program
  is over 64 MiB, and main now takes programs up to 128 MiB (c3e713307f,
  1f035d6e9f).
- **Telegram Desktop** needs the listing `linux.telegram-desktop`, which
  has no recipe or letter yet. It also needs a Qt display path in the Linux
  personality, the same one KeePassXC waits on (docs/using/privacy-apps.md).

When each lands, rehearse its block in `tour.steps`, then keep the
captions or fix them. Only what the screen shows may stay.

## 9. Money that stays yours

*A Monero wallet whose node never sees this machine.* 1:30. Needs the
SOCKS endpoint at `127.0.0.1:9050` on main, and a node reachable over
Anyone. Scripted, not rehearsed.

Beats: `market install linux.monero`, with the download at ×8. A new
wallet whose node is reached through the endpoint over Anyone. `status`,
with the wallet syncing. docs/using/privacy-apps.md says what the node
still learns. The film says only that the node sees an exit relay, which
is the box's design.

Post: a zoom onto the wallet syncing.

## 10. Your own AI

*A language model that never leaves the machine.* 1:15. Needs a Qwen model.
Both images carry Qwen3 0.6B on the stick. Rehearsed to the model's load:
under TCG the answer did not come in time, so check it on the first KVM
run.

Beats: `qwen what is a microkernel? ...`. Its policy line,
`WAYLAND MODELS_RO TTY`, with no network. The model checked against its
signed pin ("qwen: verified") and loaded. The answer streams in.

Post: a lower-third, "Qwen3 0.6B / On the stick, held in memory, offline".
Any wait for the model to load can be sped up in the edit.

## 11. Install it, keep it encrypted

*Your answers, your files and your model, on a disk only this machine
opens.* Not scripted.

It needs two things first:

- Homes kept across a reboot. They are not on main yet
  (docs/using/privacy-apps.md: "A home kept across a reboot is not in
  yet").
- A rehearsal of the installer.

It is a recording of two boots: one with `--qemu-arg=--install-target`
that installs, and one with `--qemu-arg=--installed` that boots the
installed disk. It would show setup not running again, and the note still
there. The installed disk's volume is sealed with ChaCha20-Poly1305 under a
key the TPM derives (docs/overview/design-principles-amnesic-by-default.md).
Script it when kept homes land. Until then the film says nothing about
keeping data.

## 12. Gone when you're done

*Power off wipes memory, and nothing was kept.* 0:35. Rehearsed.

| Beat | On screen | Caption |
|---|---|---|
| 1 | The first menu, Power Off, the dialog | Power Off asks first: every window closes and memory is wiped. |
| 2 | The machine stops; black | Power off. This session kept nothing, so nothing of it is left. |

## End card

Label: AGPL-3.0-OR-LATER. Title: NØNOS 0.9.3. Line: nonos.systems, the
homepage About shows. The card holds for 5 s.
