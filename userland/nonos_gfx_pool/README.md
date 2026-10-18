# nonos_gfx_pool

The `nonos_gfx_serve::Platform` a display driver uses when its primaries
are anonymous memory of its own: `alloc` maps zeroed pages, `register`
registers them as an ARGB8888 surface and shares it (the wire hands out the
share handle, never the private surface id), `release` withdraws the
surface and unmaps the pages, `pid` is the driver's own.

Used by `capsule_driver_bga`, `capsule_driver_intel_display` and, when it
lands, `capsule_driver_bootfb`. `share` is a no-op until the broker has
`mk_surface_share_to` (L18 (c)); until then the wire's owner rule is the
only limit on who is answered.

The rules it serves are proved in `gfx_serve_proofs` against a model of
this platform; this crate is the thin layer over the kernel's calls.
