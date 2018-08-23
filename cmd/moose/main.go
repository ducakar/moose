package main

import (
	"flag"
	"fmt"
	"image/color"
	"io/ioutil"
	"math/rand"
	"os"
	"strings"
	"time"

	"github.com/ducakar/moose"
	"github.com/ducakar/moose/cows"
	"github.com/ducakar/moose/fortunes"
)

func main() {
	file := flag.String("f", "moose", "Cowfile.")
	eyes := flag.String("e", "oo", "Eyes string, default `oo'")
	tongue := flag.String("T", "  ", "Tongue string, default `  '")
	think := flag.Bool("t", false, "Thinking instead of saying.")

	out := flag.String("o", "", "Output PNG file (print to stdout if is none given).")
	moosify := flag.Bool("m", false, "Generate an ASCII picture of a moose saying the provided text.")
	fortune := flag.String("F", "nice", "Select a random fortune as the input.")
	flag.Parse()

	var text string
	if *fortune != "" {
		rand.Seed(time.Now().UnixNano() * 15485863)
		lib := moose.Library{fortunes.Fortunes}
		text = lib.Get()
	} else {
		text = strings.Join(flag.Args(), " ")
		// Read from stdin if no text is given as an argument.
		if text == "" {
			data, err := ioutil.ReadAll(os.Stdin)
			if err != nil {
				fmt.Fprintln(os.Stderr, err)
				os.Exit(1)
			}
			text = string(data)
		}
	}

	format, ok := cows.Cows[*file]
	if !ok {
		format = cows.Cows["moose"]
	}

	if *moosify {
		text = moose.Moosify(text, format, *eyes, *tongue, *think)
	}

	if *out == "" {
		fmt.Println(text)
	} else {
		err := moose.WritePNG(*out, text, color.Black, color.RGBA{0, 255, 0, 255})
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
	}
}
