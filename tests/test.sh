rm -f fantasm.bin sjasmplus.bin
for i in *.asm ; do
  printf "%s\t ... " "$i"
  cargo run --quiet --release -- "$i" fantasm.bin &&
  sjasmplus --nologo --msg=none "$i" --raw=sjasmplus.bin &&
  diff fantasm.bin sjasmplus.bin && printf "Passed :)\n"
done
rm -f fantasm.bin sjasmplus.bin