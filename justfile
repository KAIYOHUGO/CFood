build-grammar:
    java -jar ./antlr.jar ./CFood.g4 -o ./src/antlr

test FILE:
    PRETTY=1 cargo run -- {{FILE}}
