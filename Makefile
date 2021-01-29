GOPATH ?= ${HOME}/go
RACE ?= 0
ENVIRONMENT ?= development
VERSION ?= dev

.PHONY: all
all: clean test build

.PHONY: test
test:
ifeq ($(RACE), 1)
	go test ./... -race -covermode=atomic -coverprofile=coverage.txt -timeout 5m
else
	go test ./... -covermode=atomic -coverprofile=coverage.txt -timeout 1m
endif

.PHONY: build
build:
ifeq ($(ENVIRONMENT),production)
	CGO_ENABLED=0 go build -ldflags="-s -w -X 'main.Version=${VERSION}'" -o ./bin/hosts cmd/hosts/main.go
else ifeq ($(ENVIRONMENT),development)
	go build -o ./bin/hosts cmd/hosts/main.go
else
	echo "Target ${ENVIRONMENT} is not supported"
endif

.PHONY: git-setup
git-setup:
	git config user.name GitHub
	git config user.email noreply@github.com
	git remote set-url origin https://x-access-token:${GITHUB_TOKEN}@github.com/malusev998/dusanmalusev.git

.PHONY: commit
commit:
	git add .
ifneq ($(shell git status --porcelain),)
	git commit --author "github-actions[bot] <github-actions[bot]@users.noreply.github.com>" --message "${MESSAGE}"
	git push
endif

.PHONY: clean
clean:
	rm -rf ./bin
