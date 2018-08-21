package moose

import (
	"io/ioutil"
	"math/rand"
	"os"
	"strings"
)

// Library holds all loaded fortunes.
type Library struct {
	Fortunes []string
}

// Append fortunes from a given file to the library.
func (l *Library) Append(path string, offensive bool) error {
	file, err := os.Open(path)
	if err != nil {
		return err
	}
	defer file.Close()

	data, err := ioutil.ReadAll(file)
	if err != nil {
		return err
	}

	if offensive {
		data = decode(data)
	}

	// Every fortune ends in "\n%\n", so we gen an empty entry at the end.
	fortunes := strings.Split(string(data), "%\n")
	if len(fortunes) != 0 {
		fortunes = fortunes[:len(fortunes)-1]
	}

	l.Fortunes = append(l.Fortunes, fortunes...)
	return nil
}

// Get a random fortune.
func (l *Library) Get() string {
	nFortunes := len(l.Fortunes)
	if nFortunes == 0 {
		return ""
	}

	index := rand.Intn(len(l.Fortunes))
	return l.Fortunes[index]
}

// List all fortunes from the library.
func (l *Library) List() []string {
	return l.Fortunes
}

func decode(data []byte) []byte {
	for i, ch := range data {
		if 'A' <= ch && ch <= 'Z' {
			data[i] = 'A' + (ch-'A'+13)%26
		} else if 'a' <= ch && ch <= 'z' {
			data[i] = 'a' + (ch-'a'+13)%26
		}
	}
	return data
}
