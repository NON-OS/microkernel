# Toolkit service

The `toolkit` [capsule](../../overview/glossary.md#capsule): a small service, no longer started at boot, which answers a health check and keeps one animation clock. It draws nothing and carries no theme. Apps draw with the toolkit library linked into each of them, not with this service.

## Library and service

`userland/toolkit/` builds both. The library, `nonos_toolkit`, is everything on the other design-system pages: themes, roles, sizes, widgets, icons and the paint layer, compiled into every app. The service is the same crate run as its own process. Its code is `server/` and `protocol/`, and the static checks keep the library free of kernel imports and IPC calls outside them (`nonos-ci/run-static-checks.sh:2767-2776`).

## Endpoints

| | Value | Where |
|---|---|---|
| Service | `service:4610:toolkit` | `userland/toolkit/Capsule.mk:8-12` |
| Reply | `reply:4611:endpoint.toolkit.reply` | `userland/toolkit/Capsule.mk:8-12` |
| Capabilities | `0x19`: CoreExec, IPC and Memory | `userland/toolkit/Capsule.mk:8-12` |
| Kernel mirror | `spawn_toolkit_capsule`, ports 4610 and 4611 | `src/userspace/capsule_toolkit/spawn.rs:30-56` |

The kernel no longer starts it. The desktop start-up plan ends at the login screen (`src/userspace/init/spawn_plan/desktop_services.rs:17-25`), because nothing sends the service a request. A build with the `nonos-capsule-toolkit` feature still embeds the capsule and its spawner.

## Operations

The server loop takes each request with `mk_ipc_recv_from` on its endpoint and answers with `dispatch` (`userland/toolkit/src/server/runner.rs:36`; `userland/toolkit/src/server/dispatch.rs:20-26`).

| Op | Name | Request | Reply |
|---|---|---|---|
| `0x0000` | `TOOLKIT_OP_HEALTHCHECK` | empty | `STATUS_OK`, no body |
| `0x0002` | `TOOLKIT_OP_ANIMATION_TICK` | an optional little-endian `u64` delta | `STATUS_OK` and the new tick, 8 bytes little-endian |
| any other | | | `E_BAD_OP`, no body |

The ops are `userland/toolkit/src/protocol/ops.rs:23-24`, the status codes `STATUS_OK` 0 and `E_BAD_OP` 1 (`userland/toolkit/src/protocol/errno.rs:17-18`). A missing delta counts as 0, and a delta of 0 still moves the clock by one (`userland/toolkit/src/animation/tick.rs:21-36`; `userland/toolkit/src/animation/store/advance.rs:20-26`). A reply buffer under 8 bytes gets no body.

## Retired operations

| Op | Name | What it did |
|---|---|---|
| `0x0001` | `TOOLKIT_OP_THEME_APPLY` | stored a theme in the service |
| `0x0003` | `TOOLKIT_OP_COMPONENT_RENDER` | drew a button or label for the caller |
| `0x0004` | `TOOLKIT_OP_THEME_GET` | gave the stored theme to an app each frame |

Each app now does that work for itself: the theme follows the policy store in every app ([Themes](themes.md)), and every app draws its own widgets. Commit 8e85fdc67 removed them, with the service's component code and the app skeleton's client for it. A request naming one is refused with `E_BAD_OP`, the same as an op never assigned, and the numbers are never reused (`userland/toolkit/src/protocol/ops.rs:19-22`).

## Who waits for it

No app does. The app skeleton waits at start only for the compositor, the window manager and the input router (`userland/app_skeleton/src/discover/require.rs:22-44`, `require_peers`). It used to wait for the toolkit too, so a toolkit service that came up late, or not at all, kept every app from opening; commit 556d094d9 removed the wait.

## What proves it

- `a_retired_op_is_refused_as_unknown` sends 0x0001, 0x0003, 0x0004, the next number after them and `u16::MAX`, and checks each gets `(E_BAD_OP, 0)` with the reply buffer untouched, then that the health check still answers (`userland/theme_proofs/src/toolkit_refusal_tests.rs:128-136`).
- `an_app_waits_for_the_compositor_wm_and_input_router_only` holds the skeleton to its three peers (`userland/desktop_proofs/tests/skeleton_peers.rs:25-26`).
- The static checks fail if either op stops being routed, if a retired op name appears anywhere under `userland/`, if `component_dispatch` or the skeleton's toolkit client comes back, or if the skeleton looks the toolkit up at start (`nonos-ci/run-static-checks.sh:2737-2827`).

## What it does not do

- Draw, or hold the theme. Both happen inside each app.
- Push anything. It only answers.
- Run, unless something starts it. No app, service or kernel path does.
