.PHONY : install
install : 
	install -m 0755 -v ./target/release/mn /usr/local/bin/mn
	cp ./mn.1 /usr/local/share/man/man1/mn.1
	# Zsh completions
	install -d /usr/share/zsh/site-functions
	cp _mn /usr/share/zsh/site-functions/_mn


.PHONY : uninstall
uninstall :
	rm -f /usr/local/bin/mn
	rm -f /usr/local/share/man/man1/mn.1
	rm -f /usr/share/zsh/site-functions/_mn
