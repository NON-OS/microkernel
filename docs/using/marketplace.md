# Marketplace

What the Marketplace and the Terminal's `market` command can install on NONOS, how a listing is checked before anything is installed, and what needs a network.

## What you can install

| Tab | What it lists | In 0.9.2 |
|---|---|---|
| `Models` | The Qwen tiers | 17 tiers, from `userland/capsule_market/linux-guests.json`. Installing one puts its model files on the [data volume](../overview/glossary.md#data-volume); the chat program is part of the image. See [Local AI](local-ai.md). |
| `Linux` | Alpine packages a build lists | None. The shipped list, `userland/capsule_market/linux-packages.txt`, is empty, so the tab says `this build lists no Linux packages`. |
| `NONOS` | NONOS apps | None. NONOS apps ship in the image, so the catalogue lists none and the tab says `this catalogue lists no NONOS apps`. |
| `Community` | Apps from other publishers | None yet: `this catalogue lists no community apps yet`. |
| `All` | Everything above | |

The tabs are `TABS` in `userland/capsule_app_store/src/store/tab.rs:33`, and the empty-tab lines are `empty` in the same file, lines 52-61. Only listings whose id starts with `linux.` can be installed; selecting anything else says `part of this system: start it from the dock` (`ask` in `userland/capsule_app_store/src/store/install.rs:81-97`). A Qwen tier's id is `linux.qwen-` and the tier word, for example `linux.qwen-small` or `linux.qwen-qwen3-0.6b`.

The catalogue exists only in a build signed with the marketplace operator key. A build without that key embeds an empty catalogue, which the market reads as no catalogue at all (`mk/21-market.mk`), and every tab is empty.

Nothing costs money. A payment [capsule](../overview/glossary.md#capsule), `userland/capsule_payment`, is in the tree and has build rules, but no kernel profile embeds it, so no 0.9.2 image carries it.

## Use the Marketplace window

Open `Marketplace` from the dock (setup's app list calls it `App store`). The window asks the market for its catalogue, then for each listing's details one at a time, the selected one first.

| Key | What it does |
|---|---|
| Up, Down, Page Up, Page Down, Home, End | Move through the list. |
| Left and Right | Change tab. |
| `/` | Search, descriptions included. |
| Enter | The next sensible step: install, wait, or open. |
| `o` | Open an installed listing. |
| `u` | Uninstall. |
| `d` | Install a Qwen tier with its model downloaded directly. |
| `r` | Reload the catalogue. |

The keys are `on_key` in `userland/capsule_app_store/src/store/event_keys.rs:32-57` and `act` in `userland/capsule_app_store/src/store/event_actions.rs:29-66`. The selected listing's card shows its publisher, version, description and each install gate with its own verdict.
