package cmd

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestAdd(t *testing.T) {
	t.Parallel()
	assert := require.New(t)

	t.Run("No arguments passed", func(t *testing.T) {
		command := appendCommand("./hosts")

		err := command.Execute()

		assert.Error(err)
		assert.Equal(err.Error(), "Hosts program needs at least an Host")
	})

	t.Run("Invalid IP", func(t *testing.T) {
		command := appendCommand("./hosts")

		command.SetArgs([]string{"somehost.test", "invalid ip"})
		err := command.Execute()

		assert.Error(err)
		assert.Equal(err.Error(), "IP 'invalid ip' is not valid address")
	})

	t.Run("More than 2 args", func(t *testing.T) {
		command := appendCommand("./hosts")

		command.SetArgs([]string{"somehost.test", "192.168.0.1", "one more"})
		err := command.Execute()

		assert.Error(err)
		assert.Equal(err.Error(), "Hosts program accepts only 2 arguments. Host and IP.")
	})

	t.Run("Appends new host to the end of the file", func(t *testing.T) {
		file, removeFile := createFile(t)
		defer removeFile()
		command := appendCommand("./hosts.txt")
		command.SetArgs([]string{"somehost.test", "192.168.0.1"})
		err := command.Execute()

		assert.Nil(err)
		resetFile(file)

		linesEqual(t, file, []string{"192.168.0.1\tsomehost.test"})
	})
}
