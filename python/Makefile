.POSIX:

default: init run

init:
	python3 -m venv .venv
	. .venv/bin/activate
	pip3 install -r requirements.txt

run:
	. .venv/bin/activate
	python3 main.py

test:
	. .venv/bin/activate
	python3 tests.py

clean:
	rm -rf .venv
