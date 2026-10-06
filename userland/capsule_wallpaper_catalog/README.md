# capsule_wallpaper_catalog

## Role

`capsule_wallpaper_catalog` is the wallpaper asset catalog. It runs as a
CPL=3 capsule and serves the built-in wallpaper images to the desktop
shell and wallpaper client: it answers how many wallpapers exist, their
slugs and sizes, and streams each image back in bounded chunks. The
catalog of pins (slug, length, SHA-256) is built in; the images are read
from the store through vfs, and each is served only when its bytes hash to
its pin. The handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

```text
desktop shell / wallpaper client
        |
        | OP_GET_COUNT / OP_GET_SLUG / OP_GET_SIZE / OP_GET_CHUNK
        v
capsule_wallpaper_catalog -- pinned catalog, images read through vfs
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x59
```

CoreExec, IPC, Memory and FileSystem. CoreExec is for `MkGetPid`, the
pid each vfs read names. The capsule registers its service, serves
callers with `MkIpcRecvFrom` plus `MkIpcReply`, and terminates only through
`MkExit`. FileSystem is for reading `/Wallpapers/collection`, or a kept
wallpaper alone at `/Wallpapers/<slug>.jpg`, through vfs; it reads only the
wallpaper asked for.
The kernel mirror is `src/userspace/capsule_wallpaper_catalog`. It requests no hardware grants.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_GET_COUNT` (0x0001) | none | number of wallpapers |
| `OP_GET_SIZE` (0x0002) | index | byte length of the image |
| `OP_GET_CHUNK` (0x0003) | index, offset | bounded slice of the image |
| `OP_GET_SLUG` (0x0004) | index | wallpaper slug |

Every frame goes to `serve` (`src/server/serve.rs`), which answers each
one exactly once: a frame too short to hold a header gets `E_BAD_LEN`, an
unknown op or a malformed body `E_INVAL`. An out-of-range index or offset
is rejected rather than clamped.

## Authority

The capsule serves catalog data over IPC and reads the wallpapers it
serves through vfs. It writes nothing, and has no PCI, MMIO, IRQ, DMA, PIO,
network, display, or focus-routing authority.

## Privacy and persistence

The pins are static and built in; the capsule holds no per-caller state
beyond the one image it is streaming, and writes nothing to disk.

## Verification

- Build: `make nonos-mk-wallpaper_catalog`; sign: `make nonos-mk-wallpaper_catalog-sign`.
- Host proofs: `userland/wallpaper_catalog_proofs` sends `serve` any frame,
  from any sender, and checks each is answered exactly once.
