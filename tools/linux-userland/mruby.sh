# mruby 3.4.0, the lightweight Ruby: the mruby interpreter with the default
# gem box (strings, arrays, hashes, IO, Time, Math, Struct, pack, sprintf and
# the rest the release bundles), built with its own minirake.
#   OUT/mruby
# mruby releases by git tag and publishes no tarball digest; the pin is the
# commit the 3.4.0 tag names, a309524d0bc90eef077a24634db2495a6f68e318
# ("Update version and release date"), and the hash of its tree.
. "$root/tools/linux-userland/lib/git.sh"
src=$(fetch_git mruby)
m="$work/mruby"
cp -R "$src" "$m"
rm -rf "$m/.git" "$m/.rev"
# minirake runs each tool as one program, so zig gets a wrapper per role.
mkdir -p "$work/zig"
printf '#!/bin/sh\nexec "%s" cc -target x86_64-linux-musl "$@"\n' "$zig" >"$work/zig/cc"
printf '#!/bin/sh\nexec "%s" ar "$@"\n' "$zig" >"$work/zig/ar"
chmod +x "$work/zig/cc" "$work/zig/ar"
# The host build makes mrbc, which compiles the gems' Ruby to bytecode, with
# the build machine's cc (its gcc toolchain would ask for a gcc a Mac does
# not have); the cross build is the program, with the build machine's paths
# mapped out.
cat >"$work/nonos.rb" <<RUBY
MRuby::Build.new do |conf|
  conf.toolchain
  [conf.cc, conf.linker].each { |t| t.command = 'cc' }
  conf.gem core: 'mruby-bin-mrbc'
end
MRuby::CrossBuild.new('nonos') do |conf|
  conf.toolchain :clang
  [conf.cc, conf.linker].each { |t| t.command = '$work/zig/cc' }
  conf.archiver.command = '$work/zig/ar'
  conf.cc.flags = %w($CFLAGS -ffile-prefix-map=$m=/mruby)
  conf.linker.flags = %w($LDFLAGS)
  conf.gembox 'default'
end
RUBY
(cd "$m" && MRUBY_CONFIG="$work/nonos.rb" ruby ./minirake -j8 all >/dev/null)
cp "$m/build/nonos/bin/mruby" "$out/mruby"
