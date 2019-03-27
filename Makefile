run:
	@echo "\n Testing https://godoc.org/golang.org/x/net/publicsuffix"
	go run go-publicsuffix/main.go

	@echo "\n Testing https://docs.rs/publicsuffix/1.5.2/publicsuffix/"
	cd rust-publicsuffix/ && cargo run -q --release

	@echo "\n Testing https://docs.rs/psl/0.4.1/psl/"
	cd rust-psl/ && cargo run -q --release
