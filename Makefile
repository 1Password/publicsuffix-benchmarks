run:
	@echo "\n Testing https://godoc.org/golang.org/x/net/publicsuffix"
	export GOPATH=`pwd` && cd src/go-publicsuffix/ && go get . && go run .

	@echo "\n Testing https://docs.rs/publicsuffix/2.0.10/publicsuffix/ (anycase)"
	cd src/rust-publicsuffix-anycase/ && cargo run -q --release

	@echo "\n Testing https://docs.rs/publicsuffix/2.0.10/publicsuffix/"
	cd src/rust-publicsuffix/ && cargo run -q --release

	@echo "\n Testing https://docs.rs/psl/2.0.8/psl/"
	cd src/rust-psl/ && cargo run -q --release
