package main

import (
	"flag"
	"fmt"
	"image/color"
	"io/ioutil"
	"moose"
	"os"
	"strings"
)

func main() {
	out := flag.String("o", "", "Output png file.")
	flag.Parse()

	if *out == "" {
		flag.Usage()
		return
	}

	text := strings.Join(flag.Args(), " ")
	if text == "" {
		data, err := ioutil.ReadAll(os.Stdin)
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
			return
		}
		text = string(data)
	}

	moose.WritePNG(*out, text, color.Black, color.RGBA{0, 255, 0, 255})
}
