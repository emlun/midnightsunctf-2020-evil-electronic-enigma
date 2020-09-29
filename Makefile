default: dist/evil-electronic-enigma

dist/evil-electronic-enigma: target/release/evil-electronic-enigma
	mkdir -p dist
	cp $< $@
	strip $@

target/release/evil-electronic-enigma:
	cargo build --release

test:
	cargo test
	cargo run -- < flag.txt | grep -q 'OK!'
	cat flag.txt flag.txt | cargo run | grep -q 'ERR'
