# Privacy networks: Nym, Anyone and Direct

Choose which network NONOS's own connections leave through, and know what each choice hides and what it does not.

## The three choices

| Choice | How a connection leaves | Who sees this machine's address | Cost |
|---|---|---|---|
| `Nym mixnet` (the default) | `net.socks5`, then `net.nym`: packets mixed and delayed through a gateway and three mix layers to an exit | The Nym entry gateway, and your local network, which sees that you use Nym | Slow: every packet is delayed on purpose. |
| `Anyone network` | `net.anon`: an onion circuit through three relays, guard, middle and exit | The guard relay, and your local network, which sees that you use Anyone | Much faster than the mixnet. |
| `Direct` | `net.sockets` straight to the host | Every site, every mirror and your local network | Fastest, and nothing hides you. |

The labels are the ones Settings shows (`ROUTE_LABELS` in `userland/policy_proto/src/route.rs:35`). The Nym mixnet is the default: setup's `Network route` step lists `Nym mixnet (default)` first (`ROUTES` in `userland/capsule_setup_wizard/src/render/screens/route.rs:12`), and a value the [policy store](../overview/glossary.md#policy-store) does not hold, does not know or cannot answer is read as Nym, never as Direct (`pick` in `userland/nonos_route_link/src/pick.rs:130-138`).

Setup's own words for each choice: the mixnet "Hides who you talk to, even from someone watching the whole internet"; Anyone means "Sites never see this machine's address" and "Someone watching both ends at once could match the traffic"; Direct means "Every site, the model mirror and your own network see this machine's address" (`WHY` in the same setup file).

## Switch networks

Open Settings, pick `Network`, and change the `Default network` row to `Nym mixnet`, `Anyone network` or `Direct`. The choice holds from the next connection: every program reads it again for each connection it opens (`chosen` in `userland/nonos_route_link/src/chosen.rs:40-43`). The browser can also switch network for one page.

A choice never falls back. If the chosen network is not running, the connection fails and says so, for example `the Nym mixnet, the default, is not running` (`NYM_DOWN` in `userland/nonos_route_link/src/pick.rs:43`). Nothing is retried over another network, and never over Direct.

The Terminal's `nym` command shows the mixnet client: `nym: gateway <address>` or `nym: client up, no gateway yet`, then a `topology:` line reading `ready`, `missing`, `expired`, `clock out of range` or `untrusted authority`. With no mixnet client on this boot it prints `nym: capsule not running`.

```
nym
```

Not tested in this release.
