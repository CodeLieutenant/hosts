package cmd

import (
	"context"
	"os"
	"runtime"

	"github.com/spf13/cobra"
)

const (
	WindowsHostsPath   = "C:\\Windows\\System32\\drivers\\etc\\hosts"
	LinuxHostsFilePath = "/etc/hosts"
	MacOSHostsFilePath = "/etc/hosts"

	Localhost = "127.0.0.1"
)

var (
	rootCmd = &cobra.Command{
		Use:   "hosts",
		Short: "Host parser",
		Long:  "Hosts file parsers and modifier",
	}
)

func Execute(ctx context.Context, version string) error {
	var filePath string

	defaultHostsPath := ""

	if runtime.GOOS == "windows" {
		defaultHostsPath = WindowsHostsPath
	} else if runtime.GOOS == "linux" {
		defaultHostsPath = LinuxHostsFilePath
	} else if runtime.GOOS == "darwin" {
		defaultHostsPath = MacOSHostsFilePath
	}

	rootCmd.Version = version

	rootCmd.PersistentFlags().StringVarP(&filePath, "file", "f", defaultHostsPath, "Path to 'Hosts' file")

	rootCmd.AddCommand(appendCommand(filePath))
	rootCmd.AddCommand(removeCommand(filePath))
	rootCmd.AddCommand(listCommand(os.Stdout, filePath))

	return rootCmd.ExecuteContext(ctx)
}
