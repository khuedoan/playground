.POSIX:

default: test build

run:
	go run main.go

fmt:
	go fmt

vet: fmt
	go vet

test: vet
	go test

build:
	go build -o bin/main main.go

clean:
	rm -rf bin
