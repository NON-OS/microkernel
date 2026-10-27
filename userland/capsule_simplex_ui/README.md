# capsule_simplex_ui

## Role

simplex.ui, the native SimpleX chat window, a no_std capsule on
`nonos_app_skeleton` and the toolkit. It holds no network and no key. It
speaks the official simplex-chat core's chat API, JSON text frames over the
WebSocket the CLI serves when started with `-p PORT`, and the core runs in
its own box with its own database (simplex.core,
[the letter](../../HANDOFF/letters/2026-10-27-s2-to-s3-s1-simplex-split.md)).

## Layout

- `src/proto/`: the API. Commands (`/_get chats 1 pcc=on`,
  `/_get chat @<id> count=50`, `/_send @<id> json [...]`, `/_read chat`,
  `/_connect`, `/_address`, `/_show_address`, `/_accept`), the frame
  `{"corrId","cmd"}`, and the readers for the `resp` envelope: `apiChats`,
  `apiChat`, `newChatItems`, `chatItemStatusUpdated`,
  `receivedContactRequest`, `contactConnected`, links and errors. A small
  JSON reader, since no JSON crate in the tree builds for capsules.
- `src/model/`: chats, the open chat, the compose and search lines, focus,
  connection state and the outbox of commands. Pure.
- `src/link/`: the `Transport` trait; `Transcript`, which replays
  `session.txt`, a recorded session in the v6 shapes; and `CoreChannel`, the
  stub where the NØNOS channel to the core goes.
- `src/ui/`: the window. Chat list on the left (name, last message, time,
  unread count), the chat on the right (bubbles, ours on the right, time and
  delivery status), the compose line at the bottom. Colours from the live
  theme; boxes grow with the toolkit's text scale.

## Keys

Up and Down move through the chats, Tab moves between the list and the
compose line, Ctrl+K searches the chats by name, Enter sends, Esc goes back
to the list, Ctrl+V pastes, Ctrl+C copies a new link, else our address, else
the newest message. Typing in the list starts a message. The compose line
also takes `/invite`, `/address`, `/connect <link>` and `/accept`.

## State and privacy

Everything is in memory and goes with the window. The chats, keys and
database are the core's, in the core's home.

## Build and test

- `cargo check` or `cargo build --release --target ../x86_64-nonos-user.json
  -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem` here.
- `userland/simplex_ui_proofs`: `cargo test` holds the protocol, the model,
  the keys and the transcript loop on the host.

## Wiring

`Capsule.mk` declares `simplex-ui` (`app.simplex`, ports 5014 and 5015) but
`mk/20-build.mk` does not include it yet. Including it puts the capsule in
`tools/nix/capsules.json`, and the image then needs the owner's steps:
publisher keys (`nonos-data/trust/keys/simplex_ui_publisher_{ed25519,mldsa65}.pub`
and their seeds), the signed certificate, manifest and trailer under
`nonos-data/trust/capsules/`, the key manifest entry, and, for a launcher
entry, the kernel mirror under `src/userspace/` with the spawn and the
desktop's app list. None of that is done here.

## Not done yet

- `CoreChannel` is a stub until the box grant to serve the core's port to
  this capsule (`SERVE_ONE simplex.ui`) exists.
- Groups are read and shown; making and managing them, files, the QR code
  of a link and notifications are not here yet.
