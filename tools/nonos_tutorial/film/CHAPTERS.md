# NØNOS 0.9.3, the film: chapters

The long cut runs 3 to 5 minutes in nine chapters, and the 60 to 90 s cut for
X comes from the same shots. Each chapter is a timeline file of shots
(`timelines/chapters/NN-name.json`, pulled in by `"include"`), and
`timelines/film.json` names two cuts over them, `full` and `x`. A shot is
cut on the bar at 128 BPM (one bar is 1.875 s).

The look is NØNOS's own: the real screen as a lit plate in a dark volume
(Ink #0A0B0D), a cyan rim light and glow on a near-black floor that holds
a rough reflection, Teal for structure, Geist for statements and JetBrains
Mono for machine text. Evidence is typeset on dark glass cards, never
paper. The Ø is the recurring motif: its slash is the wipe into the end
card.

**Rules for every chapter**

- Real footage only. Every value on screen is in CREDITS.txt with its file
  and line, and `test_film.py` holds the cards to it.
- One idea per shot. Few words: one statement or one caption, not both.
- Each chapter has its own camera grammar and its own machine motif, so no
  move repeats across chapters (the table below is the check).
- A screen from a development image says so in its slug (`DEV IMAGE ·
  <commit>`); a sealed one says `SEALED` or `LIVE · QEMU · 0.9.3 ·
  b434cdf175`.

| # | Chapter | Camera grammar | Machine motif | Length |
|---|---|---|---|---|
| 1 | Boot and attestation | vertical crane up through a stack of lines | boot lines scroll, then resolve into the loader's ten checks | 0:25 |
| 2 | The desktop and its native apps | slow orbit, the plate turning 30° | windows lift off the plate as their own planes | 0:30 |
| 3 | The Terminal, Linux apps in boxes, a refusal | lateral truck along the plate | the isolation box draws round a window as its policy types on; a refusal flashes the box edge | 0:35 |
| 4 | The Marketplace's verified install | top-down track over a card | the eleven pass lines light one by one | 0:25 |
| 5 | Privacy routing | wide pull-back between two plates | routes as thin light paths, hop by hop | 0:40 |
| 6 | GUI apps | parallax dolly past floating windows | each window's box outline and policy tab | 0:25 |
| 7 | Drivers and hardware | handheld-steady push on a real laptop plate | a spec card of the driver's serial lines | 0:20 |
| 8 | Check it yourself | locked-off split, two plates side by side | the film's cards fly to the verifier's lines and match | 0:25 |
| 9 | End card | slow push on the floor | the Ø slash wipe, the wordmark on the floor | 0:08 |

## 1. Boot and attestation

*Before anything runs, the loader proves the kernel.*

| Bars | Shot | On screen | Words |
|---|---|---|---|
| 2 | Lines in the dark | the boot's serial lines scroll up a far slab, out of focus; the crane rises | slug `SEALED b434cdf175` |
| 2 | The ten checks | the loader plate resolves out of the lines: "Measuring the kernel", step 05 CHECKING | statement: "It checks itself first." |
| 2 | The boot chain | close-up on the splash's panel: bootloader, kernel ed25519 verified, capsules zk attested | caption: "Every link of the chain, said out loud." |
| 2 | Evidence | card `EVIDENCE · IOMMU · 61ec29ea2` | none |

- **Footage, recorded:** sealed b434cdf175 frames 000059 (the loader,
  1280x800: the plate takes its 16:10 shape), 000119 and 000179 (the
  splash). Rehearsal r001 to r009 (development image): the boot menu and
  its entries.
  confined by hardware" (TCG, no IOMMU). It stays on screen if that frame
  is used. For a splash that shows DMA confined, film the sealed image in
  QEMU with `-device intel-iommu` or on hardware with VT-d (to film).
- **Evidence card:** `[IOMMU] intel-vt-d present, enforcing=1, unconfined
  grants=0, identity devices=0` and the `[VT-D]` lines,
  `docs/release-evidence/0.9.3/s1-isolation-61ec29ea2/serial.log:129-138`.

## 2. The desktop and its native apps

*Your computer forgets you.*

| Bars | Shot | On screen | Words |
|---|---|---|---|
| 2 | The live session | the plate at rest, the notice: "This is a live session" | statement: "Your computer forgets you." |
| 2 | Lift: Files | the Files window lifts forward off the plate | caption |
| 2 | Lift: Settings | "never sent on a network" | caption |
| 2 | Lift: About, Proofs | the Verified badge, the Proofs page | caption |

- **Footage, recorded:** sealed 000719 (desktop and notice). Rehearsal r033
  to r041 (development image: menu, Launchpad, Files, Settings, About).
- **To film:** Files, Settings, About and Proofs on the sealed image (the
  storyboard notes that a sealed Proofs answer is not filmed yet).
- **Evidence card:** none; this chapter is the picture.

## 3. The Terminal, Linux apps in boxes, a refusal

*Every Linux app gets a box.* (The style test's shots 2 and 3 are this
chapter's grammar.)

| Bars | Shot | On screen | Words |
|---|---|---|---|
| 2 | The box | the isolation box draws round the Terminal; `mask=0x10a NET_ANON FILES_HOME TTY` types on | caption: "Its own home. A terminal. No direct network." |
| 2 | The refusal | the hostile probe's lines; the box edge flashes back on each EPERM | caption: "Reach into another, and it is refused." |
| 2 | Evidence | card `EVIDENCE · ISOLATION · 61ec29ea2` | none |
| 2 | Shared map | card `EVIDENCE · SHARED MAP · 0d0cff02d4`, glide down its lines | caption: "A file mapped shared. Seen at once. Kept." |

- **Footage, recorded:** sealed 003599 (monero-wallet-cli and its policy
  lines), 002159 (the Terminal's start screen). Isolation lane (development
  image 61ec29ea2): calls.png, procs.png, net.png, forks.png. Shared-map
  lane (0d0cff02d4): coherence.png, next-run.png.
- **To film:** the hostile probe on the sealed image, so the refusal shot is
  sealed too.
- **Evidence cards:** isolation, `calls.png` lines 1-4, 23 and the tally
  (README.md:21, 22, 31, 42); shared map, README.md:23-29, 33.

## 4. The Marketplace's verified install

*Checked before it installs.*

| Bars | Shot | On screen | Words |
|---|---|---|---|
| 2 | The install | the Marketplace window, linux.monero | slug `LIVE · QEMU · 0.9.3 · b434cdf175` |
| 2 | Pass lines | card of `market info linux.monero`: index signature, package present, publisher signature, operator validation, architecture, attestation, commerce, release attest, guest policy, price, bond, each `pass`; the camera tracks down as each lights | statement: "Checked before it installs." |
| 2 | Its box | `[LINUX] policy bound: ... NET_ANON FILES_HOME TTY` | caption |

- **Footage, recorded:** sealed 002279 to 002399 (`market install`, `market
  info` with the eleven pass lines, 13:52 UTC), 003599 (the box line).
  Rehearsal r057 to r062 (the Marketplace window, development image).
- **To film:** the Marketplace window on the sealed image, and the install's
  end (seen by hand at 14:20:14 UTC, not in the frames).
- **Evidence card:** the eleven pass lines, transcribed exactly from
  002399.png (add them to evidence.json with that source before use).

## 5. Privacy routing

*Never direct.*

| Bars | Shot | On screen | Words |
|---|---|---|---|
| 2 | The route | a light path draws hop by hop from the plate into the dark as the stream opens (Anyone, then Nym) | statement: "Never direct." |
| 2 | Monero | the wallet syncing through the route | caption |
| 4 | SimpleX | two plates, two machines, one channel between them drawn as a path | caption |
| 2 | Telegram | the app in its box, on the route | caption |

- **Footage:** none usable yet. The wallet on b434cdf175 did not reach a node
  (fixed on the branch since), and the frames from 14:22:50 to 14:28:20 UTC
  were removed because the seed could show.
- **To film once on main:** the wallet syncing over Anyone and over Nym (no
  seed on screen: start from an existing wallet, or cut before the seed);
  SimpleX between two machines (the S2/S3 handoff: two boxes and one
  channel); Telegram.
- **Evidence cards:** the route's own log lines for each hop, from the
  serial log of the filmed run (to collect).

## 6. GUI apps

*Real Linux programs, each in its box.*

| Bars | Shot | On screen | Words |
|---|---|---|---|
| 2 | KeePassXC | the window floats forward; its box and policy tab | caption |
| 2 | Mousepad | as above, on its own path | caption |
| 2 | swayimg | an image opened from the person's files | caption |

- **Footage, recorded:** rehearsal r063, r064 (swayimg, development image).
- **To film:** KeePassXC and Mousepad, and swayimg on the sealed image.
- **Evidence card:** each app's `policy bound` line from the filmed run.

## 7. Drivers and hardware


| Bars | Shot | On screen | Words |
|---|---|---|---|
| 2 | The laptop | camera footage of the laptop, framed as a plate | slug: the machine's name |
| 2 | Wi-Fi | the network joining; a card of the driver's serial lines | caption |

- **To film:** Wi-Fi on a real laptop, and its serial log.

## 8. Check it yourself

*Every value here is real.*

| Bars | Shot | On screen | Words |
|---|---|---|---|
| 2 | The verifier | the verifier's output on a host terminal, as a plate | statement: "Check it yourself." |
| 2 | Match | the film's own cards fly in beside the verifier's lines; the same hashes, side by side | caption |

- **To film:** the release verifier run against 0.9.3 on a host, its output
  in full.
- **Evidence card:** its lines, from that run's log.

## 9. End card

The panel among its evidence cards; the Ø's slash wipes to the wordmark
standing on the floor; `0.9.3` under it. As in the style test's last shot.

## The cut for X (60 to 90 s)

From the same shots, two bars each unless noted: 1 lines in the dark, 1
the ten checks, 2 the live session, 3 the box, 3 the refusal, 4 pass
lines, 5 the route, 3 shared map, 5 SimpleX (4 bars), 8 match, 9 end card
(finale 2 bars). That is 13 shots, 26 bars, about 49 s; add the boot chain,
Monero and the GUI lift for about 60 s, or the kept-home card and KeePassXC
for about 68 s.

## Building it

- A chapter file holds `{"shots": [...]}`; `timelines/film.json` holds the
  tempo, `"include": ["chapters/01-boot.json", ...]` and `"cuts": {"full":
  [...], "x": [...]}`. Render a cut with `--cut x`.
- Footage folders are aliases (`--footage b434=<folder>`), so frames from a
  new sealed image only need a new alias, and its times.txt gives each
  frame's UTC time.
- New evidence goes into evidence.json with its file and line, and into
  CREDITS.txt; the tests refuse a card that draws a line CREDITS lacks.

## What raises it further on the Mac

- **Blender (Cycles or EEVEE):** the same plates and cards as textured
  planes in a real scene. It gives true area-light rim lighting, physically
  based rough reflections, volumetric light shafts, camera depth of field
  with real bokeh, and motion blur. Export the camera tracks from the
  timeline JSON, so the cut and its beat stay the same.
- **Recording at 60 fps of real motion:** windows opening, typing, the
  wallet syncing, instead of stills a minute apart. The pipeline already
  takes a folder of frames as a sequence.
- **Kokoro:** narration.txt voiced on the Mac, placed with a narration
  timeline; the music bed ducks under it.
- **A real music bed** on the beat grid, with cuts on its bars, not an
  assumed 128 BPM.
