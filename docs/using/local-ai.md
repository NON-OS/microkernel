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
