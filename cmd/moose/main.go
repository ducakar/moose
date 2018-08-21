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
	out := flag.String("o", "", "Output PNG file (print to stdout if is none given).")
	moosify := flag.Bool("m", false, "Generate an ASCII picture of a moose saying the provided text.")
	flag.Parse()

	text := strings.Join(flag.Args(), " ")
	// Read from stdin if no text is given as an argument.
	if text == "" {
		data, err := ioutil.ReadAll(os.Stdin)
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
			return
		}
		text = string(data)
	}

	if *moosify {
		text = moose.Moosify(text)
	}

	if *out == "" {
		fmt.Println(text)
	} else {
		err := moose.WritePNG(*out, text, color.Black, color.RGBA{0, 255, 0, 255})
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
		}
	}
}
