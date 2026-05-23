build-grammar:
    java -jar ./antlr.jar -visitor ./CFood.g4 -o ./src/antlr

test FILE:
    PRETTY=1 cargo run -- {{FILE}}

astuin:
    astuin "PRETTY=1 STDIN=1 cargo run"

# build jar
# , jar cvmf antlr/META-INF/MANIFEST.MF antlr.jar -C antlr/ .
