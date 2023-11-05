.POSIX:
.PHONY: default dev fmt

default: todo

todo:
	go build

data/sqlite.db:
	touch $@
	sqlite3 $@ '.read data/init.sql'
	sqlite3 $@ '.read data/sample_data.sql'

public/globals.css: styles/globals.css tailwind.config.js
	tailwindcss -i $< -o $@

dev: data/sqlite.db public/globals.css
	go run .

fmt:
	go fmt
