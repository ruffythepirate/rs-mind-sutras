build:
	cargo build
run:
	cargo run

test:
	cargo test

test-watch:
	 while inotifywait -e close_write -r . &> /dev/null ; do cargo test; done
