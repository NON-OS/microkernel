# The NØNOS 0.9.3 film

Three parts, run in this order:

- `tour.steps` walks a fresh boot of NØNOS 0.9.3 from power-on to power
  off, with a caption for each thing on screen. `tools/nonos-drive
  --record` films it to one 1920x1080 H.264 MP4.
- `STORYBOARD.md` is the film the recording is cut into: its chapters,
  beats, copy and look.
- `post/` cuts the film from the recording with ffmpeg: cards, cross-fades,
  zooms, lower-thirds and sped-up waits. It writes a 1080p60 master, a
  1080p30 web cut and the chapter list.

Every caption was checked against the code and the docs, and against
screenshots of a boot of the development image (a rehearsal of these steps
under TCG; see the last section). A caption says only what the screen shows
at that moment.

## What the host needs

- KVM (`/dev/kvm` read-write). The launcher picks it by itself. Under TCG
  the tour runs, but its pauses are too short for it.
- `nix`, as for any boot (`nix run .#qemu`).
- `Xvfb`, `ffmpeg` with libx264 and drawtext (libfreetype), and libX11. On
  Debian or Ubuntu: `apt install xvfb ffmpeg libx11-6`.
- User networking that reaches the internet, for the Marketplace, chat and
  Monero sections. Nothing else needs a network.
- About 2 GB of disk for the recording and the cut, and 8 GB of memory for
  the guest.

Nothing opens on the host's own desktop. QEMU's window is on an X display
of its own (Xvfb, from `:99` up), even when the host runs Wayland. Xvfb has
no window manager, and GTK makes a window full screen only through one, so
the recorder itself holds QEMU's window over the whole display
(`tools/nonos_drive/fill.py`, through libX11).

## 1. Build the image

The development image, with throwaway keys (minutes to build):

    tools/nonos-dev-image --profile standard

It is `target/dev/tree/target/release/standard-dev/nonos.img`. A sealed
image looks as a release does: its loader and splash say verified where the
development one says development. The seal needs the release keys:

    nix run .#seal        # the image is target/release/<profile>/nonos.img

The two differ in one thing the film shows. On a development image the
Marketplace downloads Alpine's packages from a relay on the host, dialled
directly at `10.0.2.2:8080` (docs/build/dev-mirror.md). On a sealed image
it downloads them over the Anyone network. The caption about it sits in a
`dev-image` block and a `sealed` block; cut the one that does not apply.

## 2. Start the mirror relay (development image only)

For the Marketplace section on a development image, start the relay before
the boot, in its own terminal, from the checkout:

    nix develop --command python3 tools/nonos-mirror-relay --cache target/mirror-cache

It serves only the bytes the catalogue's lock pins, and the guest still
checks Alpine's signature and the market's pin. A sealed image needs no
relay.

## 3. Record

From the checkout, on the development image, with everything that is on
main today:

    mkdir -p target/tour
    sed -e '/^# >>> sealed/,/^# <<< sealed/d' \
        -e '/^# >>> simplex/,/^# <<< simplex/d' \
        -e '/^# >>> telegram/,/^# <<< telegram/d' \
        -e '/^# >>> monero/,/^# <<< monero/d' \
        tools/nonos_tutorial/tour.steps > target/tour/tour.steps
    tools/nonos-drive \
      --image target/dev/tree/target/release/standard-dev/nonos.img \
      --out target/tour \
      --record target/tour/nonos-0.9.3-tour.mp4 \
      --qemu-arg=--fresh \
      --qemu-arg=--lane --qemu-arg=usb-tablet \
      --qemu-arg=--lane --qemu-arg=iommu=intel \
      --steps target/tour/tour.steps

On a sealed image, cut `dev-image` instead of `sealed`, and give
`--image target/release/<profile>/nonos.img`. Once the SOCKS endpoint, the
SimpleX listing or the Telegram listing is on main, drop its `-e` line.

- `--fresh` starts the data disk and the TPM again from the image, so setup
  runs as on a new machine.
- `--lane usb-tablet` gives the guest an absolute pointer. Every click in
  the tour is `abs:X,Y` on the 1920x1080 desktop, and without the tablet the
  pointer would not land there.
- `--lane iommu=intel` puts an Intel IOMMU in the machine. The Terminal's
  `log VT-D` shows its lines. Without it, the splash warns that device DMA
  is not confined.
- `--record` films from before power-on to the last step. It then stops
  ffmpeg with `q`, so the file is whole and plays from its start, then
  QEMU, then Xvfb.

The drive prints each caption with its time in the video, and writes them
to `target/tour/cues.txt`, which the cut reads. `target/tour/` also holds
`serial.log`, `qemu.log`, `ffmpeg.log` and `xvfb.log`. The run fails if a
step does not happen or the video is not whole.

## The sections, and how long each runs

"Pauses" is the time the steps file holds on screen. "On KVM" adds the
machine's own time, estimated: boots, installs and the model. Each section
is one or more chapters of the storyboard.

| Section in `tour.steps` | Block | Pauses | On KVM | State |
|---|---|---:|---:|---|
| 1. Every boot checks itself | | 0:50 | 1:20 | rehearsed |
| 2. No account. Thirteen answers. | | 2:54 | 3:00 | rehearsed |
| 3. Your computer forgets you; a desktop that shows its work | | 1:48 | 2:20 | rehearsed |
| 4. Who can reach what | | 1:02 | 1:10 | rehearsed |
| 5. Apps in a box | | 0:57 | 1:10 | rehearsed |
| 6. A store that checks every byte | `market` | 1:22 | 3 to 7 min | rehearsed on the development image |
| 7a. SimpleX | `simplex` | 0:31 | 2 to 5 min | needs `linux.simplex-chat` on main |
| 7b. Telegram | `telegram` | 0:36 | 3 to 8 min | needs `linux.telegram-desktop` and a Qt display path |
| 8. Money that stays yours | `monero` | 1:11 | 3 to 8 min | needs the SOCKS endpoint on main |
| 9. Your own AI | `qwen` | 1:12 | 1 to 2 min | rehearsed to the model's load; the answer comes only on KVM |
| 10. Gone when you're done | | 0:30 | 0:40 | rehearsed |

The whole recording runs about 15 minutes without the three blocks that
wait on main, and 25 to 35 with them. A section that is not rehearsed was
written from the docs and from the sections that were. Rehearse it once
what it needs lands (see the last section), and keep its captions only if
the screen bears them out.

## Run it with or without a section

Each optional section sits between two marker lines in `tour.steps`. sed
cuts any of them before the drive reads the file, as in step 3:

| Block | Why it is optional |
|---|---|
| `market` | Needs the network, and on a development image the relay above. |
| `simplex` | Needs the listing `linux.simplex-chat` on main. |
| `telegram` | Needs the listing `linux.telegram-desktop` on main, and a Qt display path in the Linux personality. |
| `monero` | Needs the SOCKS endpoint at `127.0.0.1:9050`, which is not on main yet, and a Monero node reachable over Anyone. |
| `qwen` | Needs a Qwen model. Both images carry Qwen3 0.6B on the stick, so it runs offline. |
| `dev-image` | A caption true only of a development image. |
| `sealed` | A caption true only of a sealed image. |

The chat and Monero sections install from the Marketplace too, so cutting
`market` means cutting them with it. Monero's node is named in its
`type:linux monero-wallet-cli` line; change it there. The cut leaves out a
chapter whose sections were not recorded.

Mousepad and KeePassXC have no Marketplace listing on main (the catalogue
is `userland/capsule_market/linux-catalogue.json`). KeePassXC also waits on
a Qt display path (docs/using/privacy-apps.md). swayimg is the one window
app listed, and the tour installs and opens it.

## 4. Cut the film

    python3 tools/nonos_tutorial/post tools/nonos_tutorial/post/film.json --out target/film

`film.json` reads `target/tour/nonos-0.9.3-tour.mp4` and its `cues.txt`.
Give `--source` and `--cues` for a recording kept elsewhere. It writes:

- `target/film/master-1080p60.mp4`: the film at 60 frames a second, every
  chapter marked.
- `target/film/web-1080p30.mp4`: the same at 30, smaller, for the web.
- `target/film/chapters.txt`: the chapter list to paste under the video,
  starting at 0:00.
- with `--uhd`, also `master-2160p60.mp4`, the master scaled to 3840x2160.
  The recording is 1080p, so this adds pixels for players that want them,
  not detail.

The cut cuts at captions, not at times. Each clip and effect in
`film.json` names the caption it starts at, so the edit holds for any
recording of these steps. `post/test_post.py` checks that every caption it
names is in `tour.steps`. The master takes a few minutes on 32 cores with
`--preset slow`; `--preset veryfast` gives a quick look.

What `film.json` can say of a clip: `from` and `to`; `speed` (with `tag`,
such as `×8`, drawn top right); `aspect` (1.6 puts the boot menu's 1280x800
back); `zoom` (a box to ease into, `at` a caption or seconds into the clip,
for so long); and `lower` (a lower-third's title and text, `at`, for so
long). A chapter has a `title`, a `line`, a `card` length (0 for none), and
`optional` when its sections may not be recorded. STORYBOARD.md describes
each chapter's treatment.

The film has no sound. To lay music under the web cut:

    ffmpeg -i target/film/web-1080p30.mp4 -i music.flac -map 0:v -map 1:a -map_chapters 0 \
      -c:v copy -c:a aac -b:a 192k -shortest -movflags +faststart nonos-0.9.3.mp4

## Trim and join by hand

`cues.txt` has one line per caption: its time in the recording, then its
text. Each section's first caption is where to cut. A cut that copies the
stream starts on the keyframe before the time asked, which is close enough
between sections:

    ffmpeg -ss 0:03:10 -to 0:09:45 -i nonos-0.9.3-tour.mp4 -c copy -movflags +faststart setup.mp4

For a cut to the frame, encode it again, as the recording was:

    ffmpeg -ss 0:03:10 -to 0:09:45 -i nonos-0.9.3-tour.mp4 \
      -c:v libx264 -preset veryfast -crf 20 -pix_fmt yuv420p -movflags +faststart setup.mp4

To join pieces, list them in a file and concatenate them. Pieces cut from
the same recording join without encoding:

    printf "file '%s'\n" boot.mp4 setup.mp4 desktop.mp4 > parts.txt
    ffmpeg -f concat -safe 0 -i parts.txt -c copy -movflags +faststart tour-short.mp4

## Changing the tour

- A step is one line. `say:TEXT` sets the caption (`say:` alone clears it),
  `pause:SECS` holds, `until:TEXT:SECS` waits for a serial line, and `mark`
  makes the `until:` steps after it read only new serial lines (a second
  install's `[LINUX] installed`). `key:`, `type:`, `abs:X,Y` and `click`
  drive the machine. `tools/nonos-drive --help` lists them all.
- Keep a caption to one line: about 80 characters fit across the screen.
  `tools/nonos_drive/test_record.py` checks every caption's length. It also
  checks that every step here is one the drive knows, that every typed
  character has a key, and that every block closes.
- A caption sits just above the dock, under a window centred in the work
  area. Each window in the tour is opened alone, so it opens centred:
  capsule_wm places the first window in the middle and steps each later one
  64 pixels down and right. The positions clicked come from screenshots of
  those windows.
- A change to a caption that `film.json` cuts at needs the same change
  there. `test_post.py` fails until it has one.

## Rehearse before recording

To check a changed or newly landed section, run the steps headless, with
`shot:NAME` steps where you want to look, and no `--record`. The shots are
1920x1080 screenshots of the guest, in `--out`. Under TCG, multiply the
pauses by about three, and wait 30 s after opening a window. The rehearsal
behind this tour ran that way, on the development image with the relay:
68 screenshots, one at the end of each caption, every one matching its
caption, from the boot menu to the Power Off dialog. Under TCG, Qwen got
as far as checking and loading the model; its answer needs KVM's speed,
so check that beat's timing on the first KVM run.
