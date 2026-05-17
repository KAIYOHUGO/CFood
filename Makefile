run:
	cargo run $(FILE)
run-pretty:
	PRETTY=1 cargo run $(FILE)

build-grammar:
	java -jar ./antlr.jar ./CFood.g4 -o ./src/antlr
