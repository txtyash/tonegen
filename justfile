watch:
	watchexec -r just run

line:
	@echo -e "--------------------------------------\n"

run: line
	cargo run

test: line
	watchexec -r cargo t
