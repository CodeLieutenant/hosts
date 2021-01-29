package hosts

import (
	"bufio"
	"bytes"
	"errors"
	"strings"
	"testing"
)

func TestIterate(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name            string
		includeComments bool
		includeBOMBytes bool
		err             error
		commentErr      error
		hosts           map[string]string
		expectedComment string
	}{
		{
			name:            "WithoutComments",
			includeComments: false,
			err:             nil,
			hosts: map[string]string{
				"helloworld.test": "127.0.0.1",
				"hello.test":      "192.168.0.5",
				"hello.app":       "192.168.0.6",
			},
			expectedComment: "",
		},
		{
			name:            "WithComments",
			includeComments: true,
			err:             nil,
			hosts: map[string]string{
				"helloworld.test": "127.0.0.1",
				"hello.test":      "192.168.0.5",
				"hello.app":       "192.168.0.6",
			},
			expectedComment: "# some comment",
		},
		{
			name:            "ErrorInHandle",
			includeComments: false,
			err:             errors.New("Some error"),
			hosts:           nil,
			expectedComment: "",
		},
		{
			name:            "ErrorInHandlingComments",
			includeComments: true,
			err:             nil,
			commentErr:      errors.New("Comment error"),
			hosts: map[string]string{
				"helloworld.test": "127.0.0.1",
				"hello.test":      "192.168.0.5",
				"hello.app":       "192.168.0.6",
			},
			expectedComment: "",
		},
		{
			name:            "BomBytes",
			includeBOMBytes: true,
			hosts: map[string]string{
				"helloworld.test": "127.0.0.1",
				"hello.test":      "192.168.0.5",
				"hello.app":       "192.168.0.6",
			},
		},
	}

	for _, test := range tests {
		buffer := bytes.NewBuffer(nil)

		if test.includeBOMBytes {
			buffer.Write([]byte{239, 187, 191})
		}

		buffer.WriteString(`
		127.0.0.1 helloworld.test
		192.168.0.5                 					hello.test
		192.168.0.6			hello.app
		# some comment
		`)

		handle := func(t *testing.T) func(host, ip string, isComment bool) error {
			return func(host, ip string, isComment bool) error {
				if test.err != nil {
					return test.err
				}

				if isComment {
					if test.commentErr != nil {
						return test.commentErr
					}

					if !test.includeComments {
						t.Error("Comments should not be included!")
					} else if test.expectedComment != host {
						t.Errorf("Comments are not equal: Expected %s, Actual: %s", test.expectedComment, host)
					}

					return nil
				}

				if _, ok := test.hosts[host]; !ok {
					t.Errorf("Host %s is not included in test data", host)
				}

				if test.hosts[host] != ip {
					t.Errorf("IPs are not equal for host %s: Expected IP: %s, Actual: %s", host, test.hosts[host], ip)
				}

				return nil
			}
		}

		t.Run(test.name, func(t *testing.T) {
			err := iterate(bufio.NewScanner(buffer), handle(t), test.includeComments)

			if test.commentErr != nil {
				if !errors.Is(err, test.commentErr) {
					t.Fatalf("Unecpected Comment error %v: Expected %v", err, test.commentErr)
				}
			} else {
				if !errors.Is(err, test.err) {
					t.Fatalf("Unecpected error %v: Expected %v", err, test.err)
				}
			}
		})
	}
}


func TestLargeHostsBuffer(t *testing.T) {
	t.Parallel()
	buffer := bytes.NewBufferString(strings.Repeat("# some comment\n", 5_000_000))
	buffer.WriteString("127.0.0.1\thello.test")
	err := iterate(bufio.NewScanner(buffer), func(host, ip string, isComment bool) error {
		if !isComment && host != "hello.test" && ip != "127.0.0.1" {
			return errors.New("hello.test with ip 127.0.0.1 is not found in file")
		}
		return nil
	}, true)

	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
}
