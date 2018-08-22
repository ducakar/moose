package main

import (
	"fmt"
	"io/ioutil"
	"os"

	"github.com/ducakar/moose"
)

var (
	niceLib      = moose.Library{}
	offensiveLib = moose.Library{}
)

func main() {
	if err := niceLib.AppendDir("/usr/share/fortune", false); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	if err := offensiveLib.AppendDir("/usr/share/fortune/off", true); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	fortunes := append(niceLib.Fortunes, offensiveLib.Fortunes...)
	nNice := len(niceLib.Fortunes)
	content := "// Generated code; DO NOT EDIT\n\npackage fortunes\n\n"

	content += fmt.Sprintf("var Fortunes = %#v\n", fortunes)
	content += fmt.Sprintf("var NiceFortunes = Fortunes[:%d]\n", nNice)
	content += fmt.Sprintf("var OffensiveFortunes = Fortunes[%d:]\n", nNice)

	if err := ioutil.WriteFile("fortunes.go", []byte(content), 0644); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
