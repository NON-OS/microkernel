# What each build is allowed to see. A derivation gets only the part of the
# tree it reads, so an edit to the docs rebuilds nothing and an edit to the
# kernel does not rebuild the capsules.
{ lib }:
let
  fs = lib.fileset;
  root = ../..;
  at = p: root + "/${p}";
  maybe = p: fs.maybeMissing (at p);

  # What each cargo build reads, from tools/nix/inputs.py: the crates its
  # path dependencies reach, the files its #[path], include_bytes! and other
  # path literals name, and the .cargo directories on the way up. An edit
  # reaches a build only through one of those, so an edit to the docs
  # rebuilds nothing, an edit to one capsule rebuilds that capsule and the
  # image, and a proof reruns only for its crate and the sources it mounts.
  inputs = lib.importJSON ./inputs.json;

  # Markdown is for people. No build reads it unless its Rust names the file.
  md = fs.fileFilter (f: f.hasExt "md") root;

  crate =
    dir: extra:
    let
      e = inputs.${dir} or (throw "tools/nix/inputs.json has no entry for ${dir}: add it to ROOTS in tools/nix/inputs.py and run that");
      read = fs.difference (fs.unions (map at e.include)) (fs.unions ([ md ] ++ map at e.exclude));
    in
    # e.md: the markdown its Rust names, and the files it names inside a
    # nested crate it does not otherwise read.
    # e.signed: what the seal writes and stages under nonos-data/ before it
    # builds (the market index, the model catalogue, the trust commit), taken
    # when present: they are not committed on a fresh clone, and a capsule
    # built without them embeds an empty catalogue.
    fs.toSource { inherit root; fileset = fs.unions ([ read ] ++ map at (e.md ++ extra) ++ map maybe (e.signed or [ ])); };

  # The checks read the tree the way a reviewer does: everything but the
  # media and the screenshots.
  everything = fs.difference root (fs.unions [
    (at "media")
    (at "screenshots")
    (maybe "wallpapers")
  ]);

  # The flake's source is already the tracked files only.
  set = name: files: fs.toSource { inherit root; fileset = files; };
in
{
  inherit crate;
  # A few named files, for a build that reads nothing else of the tree.
  paths = files: fs.toSource { inherit root; fileset = fs.unions (map at files); };
  everything = fs.toSource { inherit root; fileset = everything; };
  busybox = set "busybox" (fs.unions [
    (at "tools/nonos-busybox-build")
    (at "userland/capsule_linux/guests/busybox.config")
  ]);
  # One Linux program's build: the driver, its own recipe, and the files that
  # recipe names. The pins it fetches come in a sources.txt of their own
  # (tools/nix/userland.nix), so no other program's edit or pin reaches it.
  userlandScripts = name: files: set "linux-${name}" (fs.unions ([
    (at "tools/nonos-linux-userland-build")
    (at "tools/nonos-zig")
    (at "tools/linux-userland/${name}.sh")
  ] ++ map at files));
  inherit root;
}
