package moose

import (
	"image"
	"image/color"
	"image/draw"
	"image/png"
	"os"
	"strings"

	"golang.org/x/image/font"
	"golang.org/x/image/font/inconsolata"
	"golang.org/x/image/math/fixed"
)

// WritePNG renders and writes a PNG containing a text.
func WritePNG(path, text string, fg, bg color.Color) error {
	file, err := os.OpenFile(path, os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return err
	}
	defer file.Close()

	face := inconsolata.Bold8x16
	dst := Render(text, face, fg, bg)
	return png.Encode(file, dst)
}

// Render renders a text on an image.
func Render(text string, face font.Face, fg, bg color.Color) image.Image {
	lines := strings.Split(text, "\n")
	ascent := face.Metrics().Ascent.Ceil()
	rect := measure(lines, face, ascent)
	dst := image.NewRGBA(rect)
	if !rect.Empty() {
		// Fill background.
		draw.Draw(dst, rect, image.NewUniform(bg), image.ZP, draw.Src)
		// Draw text.
		drawer := font.Drawer{Dst: dst, Src: image.NewUniform(fg), Face: face}
		for i, line := range lines {
			drawer.Dot = fixed.P(0, i*ascent)
			drawer.DrawString(line)
		}
	}
	return dst
}

func measure(lines []string, face font.Face, ascent int) image.Rectangle {
	rect := fixed.Rectangle26_6{}
	for i, line := range lines {
		b, _ := font.BoundString(face, line)
		b = b.Add(fixed.P(0, i*ascent))

		if b.Min.X < rect.Min.X {
			rect.Min.X = b.Min.X
		}
		if b.Min.Y < rect.Min.Y {
			rect.Min.Y = b.Min.Y
		}
		if b.Max.X > rect.Max.X {
			rect.Max.X = b.Max.X
		}
		if b.Max.Y > rect.Max.Y {
			rect.Max.Y = b.Max.Y
		}
	}
	return image.Rect(
		rect.Min.X.Floor(), rect.Min.Y.Floor(),
		rect.Max.X.Ceil(), rect.Max.Y.Ceil(),
	)
}

// Moosify encloses the text in a bubble and adds a moose below as if it is
// saying it.
func Moosify(text string) string {
	text = strings.TrimSuffix(text, "\n")
	text = strings.Replace(text, "\t", "        ", -1)
	lines := strings.Split(text, "\n")
	nLines := len(lines)
	maxWidth := 0
	for _, line := range lines {
		if len(line) > maxWidth {
			maxWidth = len(line)
		}
	}

	framedLines := make([]string, nLines+3)
	framedLines[0] = " _" + strings.Repeat("_", maxWidth) + "_ "
	framedLines[1] = "/ " + strings.Repeat(" ", maxWidth) + " \\"
	for i, line := range lines {
		padding := strings.Repeat(" ", maxWidth-len(line))
		framedLines[i+2] = "| " + line + padding + " |"
	}
	framedLines[nLines+2] = "\\_" + strings.Repeat("_", maxWidth) + "_/"
	return strings.Join(framedLines, "\n") + moose
}

const moose = `
  \
   \   \_\_    _/_/
    \      \__/
           (oo)\_______
           (__)\       )\/\
            U  ||----w |
               ||     ||
`
