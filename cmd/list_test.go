package cmd

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/require"
)

type Lines []string

func (l *Lines) Write(bytes []byte) (n int, err error) {
	*l = append(*l, string(bytes))
	return len(bytes), nil
}

func TestList(t *testing.T) {
	t.Parallel()
	assert := require.New(t)

	file, remove := createFile(t, "./hosts-list.txt")

	defer remove()

	lines := Lines{}
	fmt.Fprintln(file, "127.0.0.1\thello.test")
	fmt.Fprintln(file, "127.0.0.1\thello2.test")

	command := listCommand(&lines, "./hosts-list.txt")

	assert.Nil(command.Execute())

	assert.EqualValues(lines, []string{"Host: hello.test, IP: 127.0.0.1\n", "Host: hello2.test, IP: 127.0.0.1\n"})
}
