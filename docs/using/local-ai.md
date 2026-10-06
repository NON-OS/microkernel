# Local AI with Qwen

Chat with a Qwen language model that runs on your own machine, offline, from the Terminal or in its own window.

## Start a chat

```
qwen
qwen qwen3-4b explain what a capability is
qwen window small
```

Not tested in this release.

- `qwen` alone runs the default tier. A tier word first picks another; the rest of the line is your first question, and it is not kept in the Terminal's history.
- The answer streams onto the screen as it is written. Enter sends a line. In the chat, `/reset` starts over, and `/exit` or `/bye` ends it. Ctrl-D on an empty line ends the chat; Ctrl-C stops it at once.
- `qwen window [tier]` opens the chat in its own desktop window, and the Terminal prompt comes back at once.
- To ask a question that starts with `window`, `get` or `tiers`, name a tier first.

These rules are the Terminal's own help for `qwen` (`HELP` in `userland/capsule_terminal/src/command/builtin/qwen/help.rs:20-53`); `help qwen` prints them.

`Qwen` in the dock or the Launchpad opens the same window on the default tier, and a toast says when the tier is a default rather than your choice (`open` in `userland/capsule_desktop_shell/src/apps_off/qwen.rs`).

The default tier, when you name none, is chosen in this order (`resolve` in `userland/capsule_model_fetch/src/default_tier.rs:78-92`):

1. the tier chosen at setup, or in Settings under `General`, in the `Qwen model` row;
2. else `qwen3-0.6b`, the tier release sticks carry, when it fits this machine;
3. else the largest tier that fits this machine's memory;
4. else the smallest tier, which then says it does not fit.

When the tier is a default rather than your choice, the Terminal says so in a line as the chat starts.

## How it runs, and why it is offline

`qwen` asks the kernel to run the tier through the [Linux personality](../overview/glossary.md#linux-personality) as a child of the Terminal. The program is `qwenchat`, built from llama.cpp at a pinned commit for three instruction sets: x86-64-v3, x86-64-v2 and plain x86-64, which QEMU's software CPU needs (`userland/linux_userland/Userland.mk`). The personality chooses among them.

A chat makes no network connection:

- A Terminal chat runs in one of two slots, `app.linux.term.1` and `app.linux.term.2`, which ask for no capability beyond the personality's own, and the personality's own set has no Network [capability](../overview/glossary.md#capability) (`TERMINAL` in `src/userspace/capsule_linux/terminal/roles.rs:25-42`). Two Terminal chats can run at once; a third is refused with EBUSY.
- A window chat runs in the role `app.linux.run`, which also asks for nothing more (`src/userspace/capsule_linux/roles.rs`). One window runs at a time; a second is refused with EBUSY. The window stays open when the Terminal that asked for it closes.
- Once a program family opens a model, the personality also refuses it every internet socket, and refuses to open a model while an internet socket is open (`refuse_inet` in `userland/capsule_linux/src/linux/net/offline.rs:37-39`).
- From the moment a model is opened, the family's console output goes only to the Terminal or window that started it, never to the serial log (`held` in `userland/capsule_linux/src/linux/file/models/held.rs:31-33`).

Only the download of a model needs a network, once per tier.

The local Qwen model, running offline: Works on an x86_64 laptop (Intel Gemini Lake, 8 GB), maintainer hardware report, 6 October 2026; the image commit was not recorded.
