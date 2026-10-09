# capsule_audio_player

`audio_player` is the music player window titled "Resonare" (1440 by 900). It lists the WAV and MP3
files under `/home/nonos/music` in the vfs (nothing ships with the system: the library is what the
person downloads from Search or copies in), decodes the selected track, draws its waveform, and streams the PCM
to the `audio.server` capsule (`userland/capsule_audio`). It is for any desktop user who wants to
play local audio files. The handbook page is
[Audio and media](../../docs/handbook/audio-and-media.md).

## Role

A `no_std` application on `nonos_app_skeleton` (`src/main.rs` calls `nonos_app_skeleton::run`
with `PlayerApp::new` from `src/app.rs`). MP3 decoding is the vendored C library in
`third_party/minimp3`, compiled by `build.rs` with clang against the `userland/nonos_qjs/shim`
headers and called through `src/decode/minimp3_sys.rs`; WAV is parsed in `src/decode/wav.rs`. The
kernel embeds it under `nonos-capsule-audio-player` through `src/userspace/capsule_audio_player/`,
with on-demand instances on `app.audio_player.1` and `app.audio_player.2`.

## Capabilities

`CAPSULE_REQUIRED_CAPS := 0x187d`, commented in `Capsule.mk` as
`CoreExec|Network|IPC|Memory|Crypto|FileSystem|GraphicsDisplayQuery|GraphicsSurfaceCreate`
(Network and Crypto for downloading an MP3 over the chosen route through TLS), and
`CAPSULE_OPTIONAL_CAPS := 0x100`, Debug, which only a `capsule-serial-debug` build grants.

- `0x0001` CoreExec and `0x0010` Memory: run on a 128 MiB heap (`src/main.rs`), and hold whole
  files (up to 32 MiB, `src/track_limit.rs`); the waveform keeps stretch peaks, never the whole
  decoded PCM (`src/peaks.rs`).
- `0x0008` IPC: `audio.server`, the vfs and the window services.
- `0x0040` FileSystem: `list_paths` on `/home/nonos/music` (`src/library/catalog.rs`), the
  download written to `/tmp` and moved there (`src/fetch/`), and a track read a
  chunk per tick through a vfs stream (`src/track.rs`, `src/loading.rs`).
- `0x0100` Debug, optional: `[PLAYER]` and `[AP]` markers (`src/mark.rs`, `src/library/catalog.rs`).
- `0x0800` GraphicsDisplayQuery and `0x1000` GraphicsSurfaceCreate: its window surface.

Endpoints: service `service:4870:app.audio_player`, reply
`reply:4871:endpoint.app.audio_player.reply`, plus instance pairs `4874/4875` and `4876/4877`.

## Interface

It serves no IPC requests. As a client, `src/audio_client/call.rs` opens a stream on
`audio.server`, feeds PCM, and sends pause, resume and close, using the shared
`userland/audio_proto` wire format. A window holds one stream and gives it back when it closes
(the transport closes it when dropped, `src/transport/machine.rs`) and about half a second after
play ends with nothing after it; play then opens a fresh one. When the lookup fails, `src/app.rs` uses a `NullSink`: tracks
still load and show their waveform, play stops at the first feed, and the transport bar says there
is no audio output (`src/trouble.rs`). A track that will not load names its reason there too.
About once a second it asks `desktop_shell` for a file handed to it (`OP_TAKE_OPEN_ARG`,
`src/app/open_arg.rs`): an MP3 or WAV in the music folder selects that library entry, one from elsewhere
plays as now playing without joining the library, and any other file is named with the reason it
cannot play (`src/library/handed.rs`).
`src/transport/` is the play, pause and seek state machine with its pump; `src/resample.rs`
converts to 48 kHz. A track is read and decoded a tick at a time (`src/loading.rs`), so the window
keeps painting while it loads. The window is built from `src/ui/`: a shell (`ui/shell/`), screens
for Home, Library, Search, Now Playing, Downloads and Settings (`ui/screen/`), and widgets.
Keys (`src/ui/event.rs`, `src/ui/shortcut.rs`): space or Enter play and pause, Left and Right seek
10 seconds of the track at its own sample rate, Up and Down step the volume 5%, M mutes, N and P
go to the next and previous track, S shuffles, R repeats and / opens Search. On the Search page
(`src/ui/search_key.rs`) typed keys, space included, edit the query, Ctrl+V pastes, Backspace
erases, Esc clears and Enter plays the first match, or downloads an https address; the arrows
still seek and step the volume.

The volume slider and mute are the system's master volume (`src/audio_client/master.rs`,
`audio.server` `OP_SET_VOLUME`), the one the keyboard's volume keys step; the window asks for it
every two seconds so a key press shows on the slider.

Downloads (`src/fetch/`, `src/app/fetched.rs`): an https link to an MP3 joins the Downloads page,
which runs one at a time on a worker thread over the route the person chose (Anyone by default),
through TLS with the chain checked. A row shows how far it has got, its speed and the time left;
it can be cancelled, and one cancelled or stopped by the network resumes from the bytes that came
(`/tmp/music-<pid>-<id>.part`, a Range request). Redirects are followed, a dropped connection is
resumed up to six times, and the file is read as MP3 or WAV before it is moved into
`/home/nonos/music`; anything else is deleted with a sentence that says what it was. The first
to finish while nothing plays starts playing.

Times and seeks count the track's own frames at its own rate (`Transport::frames_to_ms`). An MP3
has no length in its header: the load's pass through it records each frame's offset
(`src/decode/mp3_index.rs`), which gives its length once the pass ends and lets a seek start a
frame or two before the target to refill the bit reservoir, then skip into it. A track's Time
column shows `--` until it has loaded once. Tags are read a few files a tick
(`src/library/tag_pass.rs`, `src/library/tags/`: ID3v2.2 to 2.4 and ID3v1) for the title, artist
and album; a file with none keeps its file name as the line under its title, and no artist is made
up. The Library's Artists and Albums tabs order tracks by those tags (`src/library/order.rs`). The playing row's four-bar meter is lit by the loudest
sample just sent to the output, at about -24, -18, -12 and -6 dB (`src/ui/widget/eq.rs`). Lists
scroll with the wheel.

## State and privacy

All in memory: the track list, the play queue with shuffle and repeat (`src/library/queue.rs`), the
current decoder and waveform, the Downloads list and the view state (`src/ui/state.rs`). It writes
only the files it downloads, to `/tmp` and then `/home/nonos/music`; the Network bit is for those
alone. Nothing about what was played is kept after the window closes.

## Build and test

- `make nonos-mk-audio_player` and `make nonos-mk-audio_player-sign`.
- `make nonos-mk-audio-player-smoketest-test` builds it with `nonos-audio-player-smoketest`, the vfs
  with `seed-audio-store`, and a smoke test kernel; `src/selftest.rs` plays
  `/audio/boot_tone.wav` through `audio.server` and decodes `/audio/boot_tone.mp3`, printing
  `[PLAYER]` frame counts.
- `tests/host/` holds host checks for decode, resample, transport, waveform and UI geometry.

## Limits

- `tests/host/` is a subdirectory with no `[[test]]` entries, so cargo does not discover it; no
  script in the repo runs it. `userland/apps_proofs` covers the transport, the WAV decoder's
  rewind, the waveform peaks, the track size limit, the stepped load, the library's listing and
  Artists and Albums orders, the tag reader (`audio_tags_tests`), the Downloads list and its lines
  (`audio_downloads_tests`), the volume and keys (`audio_keys_tests`), the handed-over file, the lines the transport bar shows when it cannot
  play, the stream given back when a window closes or play ends, the format label and which track
  a click on the rail's queue means, and (`audio_time_tests`) times at a track's own rate, the
  MP3 frame index's length and seek plan, the row meter, the Search keys and the file-name byline.
  The MP3 decoder itself (C, built by `build.rs`) and the rest of the UI are not covered.
