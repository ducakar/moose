package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"

	"github.com/ducakar/moose"
)

var lib = moose.Library{}

func main() {
	if err := appendDir("/usr/share/fortune", false); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	if err := appendDir("/usr/share/fortune/off", true); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	content := fmt.Sprintf("package fortunes\n\nvar Fortunes = %#v\n", lib.List())

	if err := ioutil.WriteFile("fortunes.go", []byte(content), 0644); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func appendDir(dir string, offensive bool) error {
	files, err := ioutil.ReadDir(dir)
	if err != nil {
		return err
	}

	for _, f := range files {
		if f.IsDir() || strings.HasSuffix(f.Name(), ".dat") {
			continue
		}
		path := dir + string(os.PathSeparator) + f.Name()
		if err := lib.Append(path, offensive); err != nil {
			return err
		}
	}
	return nil
}
