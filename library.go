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

// Get a random fortune.
func (l *Library) Get() string {
	nFortunes := len(l.Fortunes)
	if nFortunes == 0 {
		return ""
	}

	index := rand.Intn(len(l.Fortunes))
	return l.Fortunes[index]
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

// AppendDir appends fortunes from files in a given directory that do not end
// with `*.dat`. Does not recourse.
func (l *Library) AppendDir(dirPath string, offensive bool) error {
	files, err := ioutil.ReadDir(dirPath)
	if err != nil {
		return err
	}

	for _, f := range files {
		if f.IsDir() || strings.HasSuffix(f.Name(), ".dat") {
			continue
		}
		path := dirPath + string(os.PathSeparator) + f.Name()
		if err := l.Append(path, offensive); err != nil {
			return err
		}
	}
	return nil
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
