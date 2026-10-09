package main

import (
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

func tree() {
	p := "tree"
	if !check(p, os.MkdirAll(root+"/a/b/c", 0o755) == nil, "MkdirAll") {
		return
	}
	for _, f := range []string{"a/one", "a/b/two", "a/b/c/three", "top"} {
		os.WriteFile(root+"/"+f, []byte(f), 0o644)
	}
	entries, err := os.ReadDir(root + "/a")
	var names []string
	for _, e := range entries {
		names = append(names, fmt.Sprintf("%s:%v", e.Name(), e.IsDir()))
	}
	if !check(p, err == nil && strings.Join(names, ",") == "b:true,one:false", "ReadDir", names, err) {
		return
	}
	var walked []string
	err = filepath.WalkDir(root, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		walked = append(walked, strings.TrimPrefix(path, root)+map[bool]string{true: "/", false: ""}[d.IsDir()])
		return nil
	})
	want := "/,/a/,/a/b/,/a/b/c/,/a/b/c/three,/a/b/two,/a/one,/top"
	if !check(p, err == nil && strings.Join(walked, ",") == want, "WalkDir", walked, err) {
		return
	}
	matches, _ := filepath.Glob(root + "/a/b/*")
	if !check(p, len(matches) == 2, "Glob", matches) {
		return
	}
	done(p, "MkdirAll, ReadDir, WalkDir and Glob see the tree as made")
}
