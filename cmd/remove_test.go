package cmd

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestRemove(t *testing.T) {
	t.Parallel()

	assert := require.New(t)

	t.Run("No host passed", func(t *testing.T) {
		command := removeCommand("./hosts.txt")
		err := command.Execute()

		assert.Error(err)
		assert.Equal(err.Error(), "accepts 1 arg(s), received 0")
	})

	t.Run("Remove host from file", func(t *testing.T) {
		file, close := createFile(t, "./hosts-remove.txt")
		defer close()

		fmt.Fprintln(file, "127.0.0.1\thello.test")
		fmt.Fprintln(file, "127.0.0.1\thello2.test")

		command := removeCommand("./hosts-remove.txt")
		command.SetArgs([]string{"hello.test"})

		err := command.Execute()

		assert.Nil(err)

		resetFile(file)

		linesEqual(t, file, []string{"127.0.0.1\thello2.test"})
	})
}
