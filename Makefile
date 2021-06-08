GOPATH ?= ${HOME}/go
RACE ?= 0
ENVIRONMENT ?= development
VERSION ?= dev
EXT ?=

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
	CGO_ENABLED=0 go build -ldflags="-s -w -X 'main.Version=${VERSION}'" -o ./bin/hosts$(EXT) cmd/hosts/main.go
else ifeq ($(ENVIRONMENT),development)
	go build -o ./bin/hosts$(EXT) cmd/hosts/main.go
else
	echo "Target ${ENVIRONMENT} is not supported"
endif

.PHONY: install
install:
	mv bin/hosts ${GOPATH}/bin

.PHONY: git-setup
git-setup:
	git config user.name GitHub
	git config user.email noreply@github.com
	git remote set-url origin https://x-access-token:${GITHUB_TOKEN}@github.com/malusev998/dusanmalusev.git

.PHONY: commit
commit:
	git add .
ifneq ($(shell git status --porcelain),)
	git commit --author "github-actions[bot] <github-actions[bot]@users.noreply.github.com>" --message "${MESSAGE}" --no-verify
	git push
endif

.PHONY: clean
clean:
	rm -rf ./bin

.PHONY: docker-image-build
docker-image-build: clean
	VERSION=$(VERSION) RACE=$(RACE) ENVIRONMENT=$(ENVIRONMENT) docker build --build-arg VERSION --build-arg ENVIRONMENT --build-arg RACE --compress --tag brossquad/fiber-dev:1.0.4 --rm .

docker-test:
ifeq ($(RACE), 1)
	docker run --rm -it -w /app -v $(shell pwd):/app brossquad/fiber-dev:1.0.4 go test ./... -race -covermode=atomic -coverprofile=coverage.txt -timeout 5m
else
	docker run --rm -it -w /app -v $(shell pwd):/app brossquad/fiber-dev:1.0.4 go test ./... -covermode=atomic -coverprofile=coverage.txt -timeout 1m
endif
