package hosts_test

import (
	"io"
	"io/ioutil"
	"os"
	"testing"

	"github.com/BrosSquad/hosts/v2"
)

func setup(t *testing.T) (hosts.Parser, *os.File) {
	hostsFile, err := ioutil.TempFile("", "hosts_remove_test")
	if err != nil {
		t.Fatalf("Cannot create hosts file: %v", err)
	}

	_, _ = hostsFile.WriteString("#some comment\n127.0.0.1\thello.test\n127.0.0.1\tmywebsite.app\n")
	_, _ = hostsFile.Seek(0, io.SeekStart)

	return hosts.Parser{
		File: hostsFile,
	}, hostsFile
}

func TestRemove(t *testing.T) {
	t.Parallel()

	parser, hostsFile := setup(t)
	defer hostsFile.Close()

	tmp, err := ioutil.TempFile("", "hosts_remove_tmp_test")
	if err != nil {
		t.Fatalf("Cannot create temporary file: %v", err)
	}

	defer tmp.Close()

	err = parser.Remove(tmp, "hello.test")

	if err != nil {
		t.Fatalf("Host cannot be removed from the file: %v", err)
	}

	_, _ = hostsFile.Seek(0, io.SeekStart)

	data, err := ioutil.ReadAll(hostsFile)
	if err != nil {
		t.Fatalf("Host cannot be read: %v", err)
	}

	content := string(data)
	if content != "#some comment\n127.0.0.1\tmywebsite.app\n" {
		t.Fatalf("Content of the file is not correct. Content: %s", content)
	}
}

func TestList(t *testing.T) {
	t.Parallel()
	parser, hostsFile := setup(t)
	defer hostsFile.Close()

	hosts := map[string]string{
		"hello.test":    "127.0.0.1",
		"mywebsite.app": "127.0.0.1",
	}

	err := parser.List(func(host, ip string, isComment bool) error {
		eIp, ok := hosts[host]

		if !ok {
			t.Fatalf("Host %s is not expected", host)
		}

		if eIp != ip {
			t.Fatalf("Ips are not equal: %s != %s", eIp, ip)
		}

		if isComment && host != "# some comment" {
			t.Fatalf("Unexpected comment %s", host)
		}

		return nil
	})

	if err != nil {
		t.Fatalf("Unexpected error %v", err)
	}
}

func TestAdd(t *testing.T) {
	t.Parallel()
	parser, hostsFile := setup(t)
	defer hostsFile.Close()

	err := parser.Add("testhost.hosts", "192.168.11.90")

	if err != nil {
		t.Fatalf("Unexpected error %v", err)
	}

	_, _ = hostsFile.Seek(0, io.SeekStart)

	data, err := ioutil.ReadAll(hostsFile)
	if err != nil {
		t.Fatalf("Host cannot be read: %v", err)
	}

	content := string(data)
	if content != "#some comment\n127.0.0.1\thello.test\n127.0.0.1\tmywebsite.app\n192.168.11.90\ttesthost.hosts\n" {
		t.Fatalf("Content of the file is not correct. Content: %s", content)
	}
}
