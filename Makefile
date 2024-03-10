build:
    mkdir ./src/transfer 
    protoc --rust_out=src/transfer proto/transfer.proto 