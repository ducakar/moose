package main

import (
	"fmt"
	"image/color"
	"math/rand"
	"time"

	"github.com/ducakar/moose"
)

func main() {
	rand.Seed(time.Now().UnixNano())

	l := moose.Library{}
	l.Append("/usr/share/fortune/off/limerick", true)

	m := moose.Moosify(l.Get())
	fmt.Println(m)
	moose.WritePNG("/tmp/out.png", m, color.RGBA{0, 255, 0, 255}, color.Black)
}
