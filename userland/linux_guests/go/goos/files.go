package main

import (
	"bytes"
	"io"
	"io/fs"
	"os"
	"time"
)

func files() {
	p := "files"
	src := root + "/src"
	data := bytes.Repeat([]byte("0123456789"), 1000)
	os.WriteFile(src, data, 0o644)
	in, _ := os.Open(src)
	out, _ := os.Create(root + "/dst")
	n, err := io.Copy(out, in)
	in.Close()
	out.Close()
	got, _ := os.ReadFile(root + "/dst")
	if !check(p, err == nil && n == 10000 && bytes.Equal(got, data), "io.Copy", n, err) {
		return
	}
	if !check(p, os.Chmod(src, 0o600) == nil, "Chmod") {
		return
	}
	st, _ := os.Stat(src)
	if !check(p, st.Mode().Perm() == 0o600 && st.Size() == 10000, "Stat", st.Mode(), st.Size()) {
		return
	}
	when := time.Date(2020, 5, 17, 10, 30, 0, 0, time.UTC)
	os.Chtimes(src, when, when)
	st, _ = os.Stat(src)
	if !check(p, st.ModTime().Equal(when), "Chtimes", st.ModTime()) {
		return
	}
	os.Truncate(src, 25)
	st, _ = os.Stat(src)
	if !check(p, st.Size() == 25, "Truncate", st.Size()) {
		return
	}
	if !check(p, os.Rename(src, root+"/moved") == nil, "Rename") {
		return
	}
	os.Symlink("moved", root+"/link")
	to, err := os.Readlink(root + "/link")
	lst, _ := os.Lstat(root + "/link")
	fst, _ := os.Stat(root + "/link")
	if !check(p, err == nil && to == "moved" && lst.Mode()&fs.ModeSymlink != 0 && fst.Size() == 25, "Symlink", to, err) {
		return
	}
	done(p, "io.Copy, Chmod, Chtimes, Truncate, Rename, Symlink, Readlink, Lstat")
}
