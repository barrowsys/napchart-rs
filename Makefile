###############################################################
# THIS FILE IS LICENSED UNDER MIT                             #
# THE FOLLOWING MESSAGE IS NOT A LICENSE                      #
#                                                             #
# <barrow@tilde.team> wrote this file.                        #
# by reading this message, you are reading "TRANS RIGHTS".    #
# this file and the content within it is the queer agenda.    #
# if we meet some day, and you think this stuff is worth it,  #
# you can buy me a beer, tea, or something stronger.          #
# -Ezra Barrow                                                #
###############################################################

FLAGS_ALL = --all-targets --all-features

WATCH_CMD = cargo watch -s

check:
	cargo check $(FLAGS_ALL)

build:
	cargo check $(FLAGS_ALL)

clippy:
	cargo clippy $(FLAGS_ALL) -- -D warnings

fmt:
	cargo fmt

test:
	cargo test --all-features

doc:
	cargo doc --all-features

clean:
	cargo clean

watch:
	$(WATCH_CMD) 'make check build'

watcht:
	$(WATCH_CMD) 'make check build test'

watchc:
	$(WATCH_CMD) 'make check build clippy'

watchcf:
	$(WATCH_CMD) 'make check build clippy fmt'

watchdt:
	$(WATCH_CMD) 'make check build doc test'
