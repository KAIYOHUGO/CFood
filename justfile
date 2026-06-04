build-grammar:
    java -jar ./antlr.jar -visitor ./CFood.g4 -o ./src/antlr

run FILE:
    cargo run -- {{FILE}}
    llc -filetype=obj output.ll -o output.o --relocation-model=pic
    gcc -o output.out output.o ./target/release/libswl.so
    ./output.out

astuin:
    astuin "cargo run -- --stdin --emit-cst"
