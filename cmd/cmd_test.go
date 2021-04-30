package cmd

import (
	"bufio"
	"os"
	"testing"

	"github.com/stretchr/testify/require"
)

func createFile(t *testing.T, name string) (*os.File, func()) {
	assert := require.New(t)
	file, err := os.Create(name)
	assert.Nil(err)

	return file, func() {
		assert.Nil(file.Close())
		os.Remove(name)
	}
}

func resetFile(file *os.File) {
	file.Seek(0, os.SEEK_SET)
}

func linesEqual(t *testing.T, file *os.File, lines []string) {
	assert := require.New(t)
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	for _, line := range lines {
		scanner.Scan()
		assert.Equal(line, scanner.Text())
	}

	resetFile(file)
}
