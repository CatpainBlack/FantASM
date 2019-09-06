cargo build --release
rm -f fantasm.bin sjasmplus.bin
for i in test_macros.asm ; do
  printf "%-22s ... " "$i"
  cargo run --quiet --release -- "$i" --z80n --nologo fantasm.bin --cspect &&
  sjasmplus --nofakes --zxnext --nologo --msg=none "$i" --raw=sjasmplus.bin &&
  diff fantasm.bin sjasmplus.bin && printf "Passed!\n"
done
#rm -f fantasm.bin sjasmplus.bin
