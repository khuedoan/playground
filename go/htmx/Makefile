.POSIX:
.PHONY: default dev fmt

default: todo

todo:
	go build

data/sqlite.db:
	touch $@
	sqlite3 $@ '.read data/init.sql'
	sqlite3 $@ '.read data/sample_data.sql'

dev: data/sqlite.db
	go run .

fmt:
	go fmt
