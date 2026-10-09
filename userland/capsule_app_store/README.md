# capsule_app_store

`app_store` is the marketplace window, titled "NØNOS Marketplace". It shows the catalogue the
market capsule serves on `market.index`, filtered into NONOS, Linux and Community tabs, and for the
selected listing shows its publisher, release, and each of the six install gates with its own
verdict. It is for a desktop user browsing what can be installed. It holds no install or payment
logic of its own: it displays the market's answers and asks the kernel to queue an install.

## Role

A `no_std` application on `nonos_app_skeleton` (`src/main.rs` calls `nonos_app_skeleton::run`
with `store::Store::new`). The kernel embeds it under `nonos-capsule-app-store` through the mirror
at `src/userspace/capsule_app_store/`. That feature is in `microkernel-desktop-base`, so the
standard desktop images carry the window and the offline desktop does not. The handbook page is
[docs/handbook/apps/market-and-store.md](../../docs/handbook/apps/market-and-store.md).

## Capabilities

`CAPSULE_REQUIRED_CAPS := 0x40001819`, commented in `Capsule.mk` as
`CoreExec|IPC|Memory|GraphicsDisplayQuery|GraphicsSurfaceCreate|AppInstall`:

- `0x00000001` CoreExec and `0x00000010` Memory: run, and hold the catalogue (replies up to 96 KiB,
  `src/store/market/wire.rs`).
- `0x00000008` IPC: `market.index` and the window services.
- `0x00000800` GraphicsDisplayQuery and `0x00001000` GraphicsSurfaceCreate: its window surface.
- `0x40000000` AppInstall: the `mk_app_install`, `mk_app_install_status` and `mk_app_launch` calls
  in `src/store/install.rs` and `src/store/poll.rs`. The `Capsule.mk` comment states this asks for
  an install and does not perform one; it holds neither ForeignExec nor any store authority.

Endpoints: service `service:4940:app.store`, reply `reply:4941:endpoint.app.store.reply`.

## Interface

It serves no IPC requests. As a client of `market.index` (magic `0x4E4D4B54`, version 1, 1500 ms
timeout, `src/store/market/wire.rs`) it uses:

| Op | File | Purpose |
|---|---|---|
| 2 | `src/store/market/list.rs` | list every listing: id, measurement, name, ready flag |
| 3 | `src/store/market/detail.rs` | publisher and description |
| 4 | `src/store/market/release.rs` | default release version and operator note |
| 5 | `src/store/market/ready.rs` | verdict plus six gate bits |

No market call is made on a key or a click, nor before the first frame: the catalogue is asked
on the first tick, and `src/store/fill.rs` asks ops 3 to 5 one listing a tick, the selected one
first, then each description in turn so the search can match it. Each answer is kept on its
listing (`Known` in `src/store/listing.rs`) until `r` reloads; a market that stops answering is
not asked again until then. Every reply is checked by `nonos_market_proto` (magic, version, op and
request id), so an answer to an earlier call is never read as this one's. After an install
is asked for, the window polls `mk_app_install_status` and says what happened in words
(`src/store/progress_text.rs`), wrapped to at most two lines of the detail pane. The words and
whether a retry could help come from `nonos_market_proto::reason`, which the Terminal's `market`
reads too. Codes 10 to 20 are the reasons a Qwen tier's model could not be fetched: no data
volume (no NONOS disk at all, no retry), a locked volume, no network, no signed model catalogue listing it,
a SHA-256 mismatch, a download cut short, no room, another download running, file names too long
for the volume, a machine with less memory than the tier needs, or a volume that could not be
reached (EIO or EAGAIN, a retry); 16 no room and 25 too little memory say that a live session holds
its volume in memory, and only the first offers a retry. An installed listing says where it is
held (`installed_line`): a package until restart, a model's tier in memory on a live session.
A reason no retry can change shows Live boot or Details on the
button, never Retry. A Qwen tier's card says what installing it downloads and the memory it
needs (`src/store/model_card.rs`): the pins' summed length, which `build.rs` reads from the
personality's pin tables, and the model fetcher's own memory rule, `capsule_model_fetch/src/need.rs`,
mounted by `#[path]`. Keys
(`src/store/event_keys.rs`, `src/store/event_actions.rs`): arrows, Page Up and Down, Home and End
move; Left and Right change tab; `/` opens search; Enter installs, waits or opens, and after an
uninstall that stopped asks to uninstall again; `o` opens; `u` (or Remove on an installed card)
uninstalls, and asks the system even for a listing this boot never installed, since a tier's model
may be on the disk from an earlier boot (`src/store/next_step.rs`); `r` reloads. Painting lives in
`src/store/ui/`.

## State and privacy

In memory only: the listings with each one's install progress, the cursor, scroll, tab, search
text and the last answers from the market (`src/store/state.rs`). It reads no files, writes nothing,
and has no FileSystem or Network bit; it does not see package bytes or URLs (the release URL is
skipped in `src/store/market/release.rs`).

## Build and test

- `make nonos-mk-app_store` and `make nonos-mk-app_store-sign`.

`userland/model_fetch_proofs` mounts `src/store/progress.rs` and `progress_text.rs` and checks
that every model reason reads as its own sentence. `userland/market_proofs` tests the market capsule's
readiness gate that this window displays, and mounts the search, the order the market is asked
in, the progress words and buttons, and what Enter and `u` do (`src/store/next_step.rs`).

## Not done yet

- Only `linux.` listings can be installed or opened (`src/store/install.rs:50`, `:65`); NONOS and
  Community listings answer "nothing to fetch for this listing".
- `build.rs` exports `ABOUT_GIT_SHA`, but nothing in `src/` reads it.
