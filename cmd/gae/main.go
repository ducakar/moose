package main

import (
	"image/color"
	"image/png"
	"moose"
	"net/http"

	"golang.org/x/image/font/inconsolata"
	"google.golang.org/appengine"
)

var font = inconsolata.Bold8x16
var fgColour = color.RGBA{0, 255, 0, 255}
var bgColour = color.Black

func init() {
	http.HandleFunc("/api/text", renderText)
	http.HandleFunc("/api/moose", renderMoose)
}

func main() {
	appengine.Main()
}

func renderText(w http.ResponseWriter, r *http.Request) {
	defer r.Body.Close()

	text := r.URL.Query().Get("text")
	render(w, text)
}

func renderMoose(w http.ResponseWriter, r *http.Request) {
	defer r.Body.Close()

	text := r.URL.Query().Get("text")
	text = moose.Moosify(text)
	render(w, text)
}

func render(w http.ResponseWriter, text string) {
	img, err := moose.Render(text, font, fgColour, bgColour)
	if err != nil {
		writeError(w, err)
		return
	}

	w.Header().Add("Content-Type", "image/png")
	if err := png.Encode(w, img); err != nil {
		writeError(w, err)
	}
}

func writeError(w http.ResponseWriter, err error) {
	w.WriteHeader(500)
	w.Header().Add("Contnet-Type", "text/plain")
	w.Write([]byte(err.Error()))
}
