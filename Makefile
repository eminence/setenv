

test:
	cargo test --no-run
	bash tests/test_bash
	tcsh tests/test_tcsh
	zsh tests/test_zsh
	ksh tests/test_ksh
