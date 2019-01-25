.PHONY : install
install : 
	cargo build --release
	cargo run --bin generate-docs
	sudo install -m 0755 -v ./target/release/mn /usr/local/bin/mn
	# man page
	# sudo cp ./docs/mn.1 /usr/local/share/man/man1/mn.1
	# Zsh completions
	sudo install -d /usr/share/zsh/site-functions
	sudo cp ./completions/_mn /usr/share/zsh/site-functions/_mn


.PHONY : uninstall
uninstall :
	sudo rm -f /usr/local/bin/mn
	sudo rm -f /usr/local/share/man/man1/mn.1
	sudo rm -f /usr/share/zsh/site-functions/_mn
