package cmd

import (
	"context"
	"runtime"

	"github.com/spf13/cobra"
)

const (
	WindowsHostsPath   = "C:\\Windows\\System32\\drivers\\etc\\hosts"
	LinuxHostsFilePath = "/etc/hosts"

	Localhost = "127.0.0.1"
)

var (
	rootCmd = &cobra.Command{
		Use:   "hosts",
		Short: "Host parser",
		Long:  "Hosts file parsers and modifier",
	}

	filePath string
)

func Execute(ctx context.Context, version string) error {
	defaultHostsPath := ""

	if runtime.GOOS == "windows" {
		defaultHostsPath = WindowsHostsPath
	} else if runtime.GOOS == "linux" {
		defaultHostsPath = LinuxHostsFilePath
	}

	rootCmd.Version = version

	rootCmd.PersistentFlags().StringVarP(&filePath, "file", "f", defaultHostsPath, "Path to 'Hosts' file")

	rootCmd.AddCommand(appendCommand())
	rootCmd.AddCommand(removeCommand())
	rootCmd.AddCommand(listCommand())

	return rootCmd.ExecuteContext(ctx)
}
