# capsule_wallpaper

## Role

`capsule_wallpaper` paints the desktop background. It registers one
full-screen surface, submits it to the compositor at `z` 0, below the
desktop shell's overlay (1) and every application window (2), and keeps it
painted with the wallpaper the policy store names. The images come from
`capsule_wallpaper_catalog`. The handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

```text
policy (Field::Wallpaper) --> wallpaper --> wallpaper_catalog (JPEG bytes)
                                  |
                                  | decode, stretch, paint
                                  v
                    own surface at z 0 --> compositor
```

## Microkernel contract

- `MkServiceLookup` finds the compositor, the policy store, the catalog and
  the setters.
- `MkMmap`, `MkSurfaceRegister` and `MkSurfaceShare` make the background
  surface, sized from the compositor's `OP_DISPLAY_INFO`.
- `MkIpcCall` submits the scene and commits damage to the compositor, asks
  `policy` for `Wallpaper` and fetches images from the catalog in chunks.
- `MkIpcRecvFrom` and `MkIpcReply` serve `service:4340:wallpaper`.
- `MkDisplayVsyncWait` is the clock for a fade.
- The kernel mirror is `src/userspace/capsule_wallpaper`, under the feature
  `nonos-capsule-wallpaper`.

## Interface contract

Requests use a 20-byte header with magic `0x4E57_4C50` ("NWLP"):

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_SET_WALLPAPER` | 0x0002 | paint a colour or an image carried in the request (PNG, BMP, JPEG or raw LZ4) |
| `OP_GET_WALLPAPER` | 0x0003 | the current colour, placement policy, size and alpha |
| `OP_SET_POLICY` | 0x0004 | the placement: Fill, Fit, Stretch, Center or Tile |
| `OP_FADE` | 0x0005 | fade to a target alpha over a duration |

`OP_SET_WALLPAPER` is accepted only from the pids behind `desktop_shell`
and `policy`, looked up on every request; any other sender gets `E_ACCES`.
A frame whose header is refused is answered with `E_BAD_MAGIC`,
`E_BAD_VERSION` or `E_BAD_LEN`.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x1818`: IPC, Memory, GraphicsDisplayQuery and
GraphicsSurfaceCreate. GraphicsDisplayQuery is the bit `MkDisplayVsyncWait`
needs. No FileSystem: the images come from the catalog
over IPC. No Debug, network, driver or admin authority.

## Runtime lifecycle

1. `wait_for_setup` retries setup until the compositor answers and the
   surface is registered and submitted.
2. The server loop serves requests. Every 300 ticks the subscriber asks
   the policy store for `Wallpaper`; when the index changed it fetches that
   image from the catalog, decodes it with the toolkit's JPEG decoder into
   a buffer of at most 1920 by 1080 pixels, stretches it over the surface
   and commits damage.

## Privacy and persistence

The capsule reads no user files and stores nothing. The chosen wallpaper
is the policy store's `Wallpaper` field, which is kept across reboots only
as one of the answers first-boot setup keeps on a persistent install.

## Failure model

- Compositor not ready: setup is retried.
- Catalog or policy unreachable: the current background stays.
- An image that does not decode leaves the background as it was and the
  request gets `E_INVAL`.

## Verification

- Build: `make nonos-mk-wallpaper`; sign: `make nonos-mk-wallpaper-sign`.
- `userland/wallpaper_catalog_proofs` covers the catalog's request path;
  nothing covers this capsule's own on the host.
