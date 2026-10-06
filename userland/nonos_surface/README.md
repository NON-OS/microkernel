# nonos_surface

A thin, `no_std` wrapper over the kernel's surface calls for code that does
not link `nonos_libc`. Each function makes one syscall through `nonos_abi`
and returns its raw result.

| Function | Syscall | What it does |
|---|---|---|
| `create(&SurfaceDescriptor)` | `N_SURFACE_REGISTER` | turn the caller's pages into a surface |
| `share(sid)` | `N_SURFACE_SHARE` | turn a slot the caller owns into a handle |
| `attach(handle, &mut SurfaceDescriptor)` | `N_SURFACE_ATTACH` | map a handle into the caller |
| `destroy(handle)` | `N_SURFACE_RELEASE` | drop the caller's attach and its reference |
| `damage(handle)` | `N_SURFACE_PRESENT` | present the whole surface to the framebuffer |

`SurfaceDescriptor` mirrors the kernel's record: width, height, stride and
format as u32, then byte length, base address and flags as u64.
`SURFACE_FORMAT_ARGB8888` is 1, the one format the kernel accepts.

The kernel checks each call against the caller's capability word:
GraphicsSurfaceCreate for register, share and release, GraphicsSurfaceMap
for attach and GraphicsPresent for present. `damage` is therefore a
present, not a compositor damage commit; desktop apps go through
`app_skeleton` and the compositor instead.

Users: `userland/nonos_runtime`, `userland/sdk/nonos_window` and
`userland/sdk/nonos_app`. The surface model is in
[Compositor](../../docs/handbook/desktop/compositor.md).
