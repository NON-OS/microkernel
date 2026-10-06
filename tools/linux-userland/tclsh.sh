# Tcl 8.6.18, the tclsh shell, with its script library as data beside it.
#   OUT/tclsh, and its library under OUT/tcl8.6
# The Tcl core team publishes no sha256; the hash pinned is of the tarball
# whose SHA-1 (84ba661edc5e615f32944e68039db568328f9a47) and MD5 are the ones
# SourceForge lists for the release under the tcl project.
tar -xzf "$(fetch tcl)" -C "$work"
t="$work/tcl8.6.18"
# The library directory compiled in is /usr/lib/tcl8.6, where the store puts
# the scripts, so tclsh starts with no TCL_LIBRARY set. No extension loading:
# the personality maps no file it has not proved. The bundled packages (Itcl,
# Thread, TDBC, SQLite) are extensions and are not built.
# Tcl reads the build machine's uname as the system it configures for, and
# runs test programs when it can: the system is named Linux here, and the
# build is always cross (a --build that is not the host), so a Mac or an
# x86-64 Linux builder configures the same Tcl. It ships no config.guess;
# its configure reads --build only to know it cannot run what it builds.
# What it would have run to learn, it is told: musl's memcmp, strtod, strstr
# and strtoul all work.
# It also asks uname -s itself, past that setting, in six places (threads,
# gethostbyname, CoreFoundation, the shell's compile switches), which in a
# cross build describes the wrong machine: configure runs with a uname that
# describes the target, x86-64 Linux. On a Mac it gave the shell
# -mdynamic-no-pic, which zig refuses for Linux.
mkdir -p "$work/target-uname"
cat >"$work/target-uname/uname" <<'UNAME'
#!/bin/sh
case "$1" in
-r) echo 6.0.0 ;;
-m | -p) echo x86_64 ;;
-n) echo nonos ;;
-a) echo "Linux nonos 6.0.0 #1 x86_64" ;;
*) echo Linux ;;
esac
UNAME
chmod +x "$work/target-uname/uname"
(cd "$t/unix" &&
	PATH="$work/target-uname:$PATH" CC="$CC" AR="$AR" RANLIB="$RANLIB" CFLAGS="$CFLAGS" LDFLAGS="$LDFLAGS" \
		tcl_cv_sys_version=Linux-6 \
		ac_cv_func_memcmp_working=yes tcl_cv_strtod_buggy=ok tcl_cv_strtod_unbroken=ok tcl_cv_strstr_unbroken=ok \
		tcl_cv_strtoul_unbroken=ok \
		./configure --host=x86_64-linux-musl --build=build-machine \
		--prefix=/usr --disable-shared --disable-load --enable-threads >/dev/null &&
	make -j8 tclsh >/dev/null)
cp "$t/unix/tclsh" "$out/tclsh"
# The scripts tclsh sources as it starts (init.tcl), the ones its
# autoloader finds through tclIndex (unknown commands, packages, modules,
# history, parray, word boundaries), and clock with the msgcat package it
# loads, so clock format and clock scan work. Time zones other than UTC,
# http and tcltest are left to a later install.
mkdir -p "$out/tcl8.6/msgcat"
for f in init.tcl tclIndex auto.tcl package.tcl tm.tcl history.tcl parray.tcl word.tcl clock.tcl \
	msgcat/msgcat.tcl msgcat/pkgIndex.tcl; do
	cp "$t/library/$f" "$out/tcl8.6/$f"
done
