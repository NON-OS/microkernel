# capsule_video_player

`video_player` is the "Video" window: a library of the video files in the vfs and a player for
Motion-JPEG AVI. It streams frames from the vfs through a bounded read window, decodes each JPEG
with the crates.io `zune-jpeg` 0.4, and letterboxes it to the window. It is for any desktop user
watching the Motion-JPEG AVI films they put under `/Movies`; the image ships none. The handbook page is
[Audio and media](../../docs/handbook/audio-and-media.md).

## Role

A `no_std` application on `nonos_app_skeleton`. `src/main.rs` sizes the heap to 64 MiB and calls
`nonos_app_skeleton::run` with `VideoApp::new` (`src/app/state.rs`). AVI parsing is the shared
`userland/nonos_avi` crate. The kernel embeds it under `nonos-capsule-video-player` through
`src/userspace/capsule_video_player/`, with instances on `app.video_player.1` and
`app.video_player.2`.

## Capabilities

`CAPSULE_REQUIRED_CAPS := 0x3859`, commented in `Capsule.mk` as
`CoreExec|IPC|Memory|FileSystem|GraphicsDisplayQuery|GraphicsSurfaceCreate|GraphicsSurfaceMap`.
`CAPSULE_OPTIONAL_CAPS := 0x100` (Debug, granted only by a `capsule-serial-debug` build) and
`CAPSULE_CAPS_CEILING := 0x3959`.

- `0x0001` CoreExec and `0x0010` Memory: run, and hold the read window and frame buffers.
- `0x0008` IPC: the vfs and the window services.
- `0x0040` FileSystem: `list_paths`, `stat`, `read_file` and `VfsStream` reads
  (`src/catalog/scan.rs`, `src/catalog/probe.rs`, `src/player/source.rs`).
- `0x0800`, `0x1000`, `0x2000` graphics: its window surface.

Endpoints: service `service:4926:app.video_player`, reply
`reply:4927:endpoint.app.video_player.reply`, plus instance pairs `4928/4929` and `4930/4931`.

## Interface

It serves no IPC requests. As a client it asks `desktop_shell` about once a second for a file
handed to it (`OP_TAKE_OPEN_ARG`, `src/app/open_arg.rs`) and plays it. The window is 960 by 600
(`src/app/view.rs`) with two pages in its sidebar, Library and Folders, plus Now Playing and Media
Details (`src/ui/screen.rs`).

- Library: `src/catalog/scan.rs` lists `/`, `/Movies`, `/Series`, `/Downloads` and `/Clips` (up to
  256 entries) and lists only what the player decodes, Motion-JPEG `.avi`
  (`src/catalog/entry.rs`). `src/catalog/probe.rs` reads each file's AVI header for its size,
  frame size and length, and decodes its first frame, which the cards and the details page draw
  (`src/ui/widget/poster.rs`). When no folder could be listed, the library screens say the file
  store did not answer or would not list them, instead of an empty library, and the next key or
  click scans again (`src/catalog/says.rs`). The sidebar counts the videos; at the scan's cap it
  says "First 256 videos".
- Folders: the same list filtered to one of those roots, with an Open button.
- Player: `open` reads the first 2 MiB of the file; a longer film keeps its `idx1` index after
  its frames, so the index is read from where the `movi` list ends and parsed with the head
  (`src/player/source.rs`, `nonos_avi::AviFile::parse_parts`). `src/player/source.rs` keeps a
  1 MiB window of the file, `src/player/clock.rs` decides show, wait or skip per frame,
  `src/player/decode.rs` decodes, `src/player/scale.rs` letterboxes. The stream has no sound
  path, and the bar says "Picture only, no sound" where a volume control would be.
- Where a video was left is kept for the window (`src/app/resume.rs`): pausing, leaving the
  player, reaching the end or opening another video notes the position, the cards draw how much
  was watched, and the video opens there again unless it was left within two seconds of its end.
- Media Details shows the video the player has open, a handed-over file included, not the
  library's selected row.
- Keys (`src/event/key.rs`): on the list pages Up and Down move, Enter opens, a printable key
  types into the search and Backspace erases, Esc closes; the wheel moves the selection. In the
  player, space plays and pauses, Left and Right seek 10 seconds, `l` shows the library, `0`
  restarts and Esc closes. Clicks: the back arrow and the info button in the player's header,
  play, the 10-second skips, the scrub bar, and the picture itself to play or pause.

## State and privacy

All in memory: the scanned items with their thumbnails and where each was left, the search and
folder filter, the view mode, the current file's index and read window, and the playback clock.
Nothing is written to the vfs, so all of it resets with each new window. It has no Network bit.

## Build and test

- `make nonos-mk-video-player` and `make nonos-mk-video-player-sign` (the slug is `video-player`).
- `make nonos-mk-video-player-test` rebuilds it with `nonos-video-player-smoketest`;
  `src/selftest.rs` parses and decodes all 12 frames of `assets/clip.avi`, scales one into 320 by
  240, and prints `[VIDEO] PASS` or `FAIL`.
- `userland/nonos_avi/tests/` runs host tests of the demuxer against the same `assets/clip.avi`,
  `split.rs` among them for a head parsed with its index read apart.
- `userland/apps_proofs` (`video_browse_tests`) covers the library filter, the search keys, the
  folders and where a video was left.

## Limits

- No sound: the AVI's audio stream is not read, and nothing here talks to `audio.server`.
- Only Motion-JPEG AVI. Other files in the folders are not listed; a non-AVI handed over from
  Files is named with that reason.
- Where a video was left lasts as long as the window; nothing is written to the vfs.
