rm -f test.bin sjasm.bin
cargo run --release -- all_opcodes.asm test.bin
sjasmplus all_opcodes.asm --raw=sjasm.bin
diff test.bin sjasm.bin
