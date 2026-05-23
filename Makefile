run:
	cargo run $(FILE)
run-pretty:
	PRETTY=1 cargo run $(FILE)

build-grammar:
    java -jar ./antlr.jar -visitor ./CFood.g4 -o ./src/antlr

astuin:
	astuin "PRETTY=1 STDIN=1 cargo run"
