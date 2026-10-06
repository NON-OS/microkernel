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

## The tiers

There are 17 tiers. Every file is a GGUF file the Qwen team publishes on Hugging Face, pinned by name, length and SHA-256 inside the signed personality (`Pinned` in `userland/capsule_linux/src/linux/file/models/pinned.rs:32-37`). The pins are in `pinned_qwen25.rs`, `pinned_qwen25_big.rs`, `pinned_qwen3.rs` and `pinned_coder.rs` beside it.

Sizes are in GiB (2^30 bytes). "Files" is the pinned lengths summed. "To run" is what the model fetcher's rule says a chat needs: the files, a key and value cache for 2048 positions, a 512-token batch and a 64 MiB margin (`memory` in `userland/capsule_model_fetch/src/need.rs:60-64`). "Installed" is the smallest total memory that fits on an installed NONOS: the run plus 1 GiB kept for the system. These figures come from that rule; they were not measured on a machine.

| Tier | Model and quantisation | Files | To run | Installed |
|---|---|---:|---:|---:|
| `small` | Qwen2.5 0.5B Instruct, Q4_K_M | 0.46 | 0.57 | 1.57 |
| `medium` | Qwen2.5 1.5B Instruct, Q4_K_M | 1.04 | 1.20 | 2.20 |
| `large` | Qwen2.5 3B Instruct, Q4_K_M | 1.96 | 2.16 | 3.16 |
| `xlarge` | Qwen2.5 7B Instruct, Q4_K_M, 2 files | 4.36 | 4.64 | 5.64 |
| `xxl` | Qwen2.5 14B Instruct, Q4_K_M, 3 files | 8.37 | 8.96 | 9.96 |
| `max` | Qwen2.5 32B Instruct, Q4_K_M, 5 files | 18.49 | 19.21 | 20.21 |
| `qwen3-0.6b` | Qwen3 0.6B, Q8_0 | 0.60 | 0.91 | 1.91 |
| `qwen3-1.7b` | Qwen3 1.7B, Q8_0 | 1.71 | 2.05 | 3.05 |
| `qwen3-4b` | Qwen3 4B, Q4_K_M | 2.33 | 2.75 | 3.75 |
| `qwen3-8b` | Qwen3 8B, Q4_K_M | 4.68 | 5.15 | 6.15 |
| `qwen3-14b` | Qwen3 14B, Q4_K_M | 8.38 | 8.91 | 9.91 |
| `qwen3-30b-a3b` | Qwen3 30B-A3B, mixture of experts, 3B active, Q4_K_M | 17.28 | 17.59 | 18.59 |
| `qwen3-32b` | Qwen3 32B, Q4_K_M | 18.40 | 19.12 | 20.12 |
| `coder-1.5b` | Qwen2.5-Coder 1.5B Instruct, Q4_K_M | 1.04 | 1.20 | 2.20 |
| `coder-7b` | Qwen2.5-Coder 7B Instruct, Q4_K_M, 2 files | 4.36 | 4.64 | 5.64 |
| `coder-14b` | Qwen2.5-Coder 14B Instruct, Q4_K_M, 2 files | 8.37 | 8.96 | 9.96 |
| `coder-32b` | Qwen2.5-Coder 32B Instruct, Q4_K_M, 3 files | 18.49 | 19.21 | 20.21 |

On a live boot the model files are held in memory too, so a tier fits only when its files and its run together stay within what the kernel leaves free: total memory less the larger of 1 GiB and a quarter of memory (`fits` in `userland/capsule_model_fetch/src/need.rs:98-103`). `qwen tiers` prints, for this machine, each tier's download, the memory it needs, and whether it is here.

As an example of a pin, `qwen3-0.6b` is the single file `Qwen3-0.6B-Q8_0.gguf`, 639,446,688 bytes, SHA-256 `9465e63a22add5354d9bb4b99e90117043c7124007664907259bd16d043bb031` (`userland/capsule_linux/src/linux/file/models/pinned_qwen3.rs`).

## Install a model

You can install a tier three ways:

- From the Marketplace: the `Models` tab lists every tier. Select one and press Enter. Its card says what it downloads and the memory it needs. See [Marketplace](marketplace.md).
- From the Terminal: `qwen get TIER` downloads one or more tiers, and `qwen tiers` lists them.
- From a release stick: on a live boot from a stick that carries it, `qwen3-0.6b` is imported from the stick with no network when you open it there.

```
qwen tiers
qwen get qwen3-0.6b
qwen get --direct qwen3-8b
```

Not tested in this release.

What a download needs:

- A system built with the signed model catalogue. It is built from the same pins and signed with the marketplace operator key (`mk/22-models.mk`). Without it, `qwen get` ends with exit status 3 (`NO_CATALOGUE` in `userland/capsule_model_fetch/src/exit.rs:35`) and says the system `was built without a signed model catalogue (no marketplace operator key), so it has nothing to fetch from; put a tier on its disk with tools/nonos-qwen-tier.py instead` (`userland/capsule_model_fetch/src/catalogue/embed.rs`).
- A [data volume](../overview/glossary.md#data-volume): NONOS installed on a disk, or a live boot, whose volume is held in memory. A machine with no disk that carries NONOS at all has nowhere to keep a model.
- A network. Downloads go over the Anyone network whatever the default network is, and wait up to three minutes for Anyone to build its first circuit. `qwen get --direct` takes one download directly instead, faster, and the mirror then sees this machine's address (`DIRECT` in `userland/capsule_terminal/src/command/builtin/qwen/fetch_words.rs:29-36`). See [Privacy networks](privacy-network.md).
- A boot that runs a network. Air-Gapped, Safe Mode and Recovery boots run none.

The files come from the NONOS model repository when the build named one (`NONOS_MODEL_MIRROR` in `mk/22-models.mk`), else from the Qwen team's Hugging Face files.

`market uninstall linux.qwen-TIER` takes a tier's model off the machine again.

## Where models are kept

A model is kept on the data volume. The kernel seals every sector of it with ChaCha20-Poly1305 under a random nonce (`seal` in `src/fs/cryptoblock/seal.rs:22-46`).

- The fetcher streams each file to the kernel, which hashes what it seals. The file is linked only when its SHA-256 is the pinned digest, and the kernel reads it back and hashes it again before writing its `<name>.sha256` record (`src/fs/blockfs_volume/import_feed/finish.rs`).
- A download that stops keeps what came: a mark `<name>.partial` is saved every 64 MiB (`MARK_EVERY` in `src/fs/blockfs_volume/import_feed/live.rs:35`), and the next `qwen get` goes on from there.
- On an installed NONOS the volume is on the disk. On a live boot it is held in memory and gone at power off, and it grows only while more than the larger of 1 GiB and a quarter of memory is free (`reserve` in `src/fs/cryptoblock/ram.rs:55-59`).
- The fetcher can write to the volume but holds no FileSystem capability, so it cannot read what the volume holds.

## What reads a model file

`qwenchat` reads a model with llama.cpp's own GGUF loader, inside the Linux personality. Only a file whose SHA-256 is the pin reaches it. The strict GGUF header reader in `userland/nonos_gguf` is used by no capsule in this release.

## When it does not start

`qwen` prints the kernel's reason and its errno name (`REASONS` in `userland/capsule_terminal/src/command/builtin/qwen/refused.rs:25-35`), among them:

- `Linux and Qwen are off for this boot, at setup or by Safe Mode or Recovery (EACCES)`
- `not enough memory to load the model (ENOMEM)`
- `every place for a Qwen chat is taken; end or close a running one first (EBUSY)`
- `this system was built without the Linux personality that runs Qwen (ENOENT)`

Why a download stopped, in the Marketplace's words, is listed on the [Marketplace](marketplace.md) page.

## See also

- [Marketplace](marketplace.md)
- [Linux programs](linux-programs.md)
- [Privacy networks](privacy-network.md)
- [Settings](settings.md)
- [Install to disk](../install/install-to-disk.md)
