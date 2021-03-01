package main

import (
	"fmt"
	"time"

	"golang.org/x/net/publicsuffix"
)

func run(domain, expected string) {
	s, err := publicsuffix.EffectiveTLDPlusOne(domain)
	if err != nil {
		panic(err)
	}

	if s != expected {
		panic(fmt.Errorf("expected %q not %q", expected, s))
	}
}

func main() {
	{
		iterations := 1000
		startedAt := time.Now()
		for i := 0; i < iterations; i++ {
			run("abc.www.example.com", "example.com")
			run("abc.golang.org", "golang.org")
			run("www.食狮.中国", "食狮.中国")
			run("www.xn--85x722f.xn--55qx5d.cn", "xn--85x722f.xn--55qx5d.cn")
			run("a.b.example.uk.com", "example.uk.com")
		}
		elapsed := time.Since(startedAt)
		fmt.Printf("%8d iterations = %3.3fs\n", iterations, elapsed.Seconds())
	}

	{
		iterations := 10000
		startedAt := time.Now()
		for i := 0; i < iterations; i++ {
			run("abc.www.example.com", "example.com")
			run("abc.golang.org", "golang.org")
			run("www.食狮.中国", "食狮.中国")
			run("www.xn--85x722f.xn--55qx5d.cn", "xn--85x722f.xn--55qx5d.cn")
			run("a.b.example.uk.com", "example.uk.com")
		}
		elapsed := time.Since(startedAt)
		fmt.Printf("%8d iterations = %3.3fs\n", iterations, elapsed.Seconds())
	}

	{
		iterations := 100000
		startedAt := time.Now()
		for i := 0; i < iterations; i++ {
			run("abc.www.example.com", "example.com")
			run("abc.golang.org", "golang.org")
			run("www.食狮.中国", "食狮.中国")
			run("www.xn--85x722f.xn--55qx5d.cn", "xn--85x722f.xn--55qx5d.cn")
			run("a.b.example.uk.com", "example.uk.com")
		}
		elapsed := time.Since(startedAt)
		fmt.Printf("%8d iterations = %3.3fs\n", iterations, elapsed.Seconds())
	}

	{
		iterations := 10000000
		startedAt := time.Now()
		for i := 0; i < iterations; i++ {
			run("abc.www.example.com", "example.com")
			run("abc.golang.org", "golang.org")
			run("www.食狮.中国", "食狮.中国")
			run("www.xn--85x722f.xn--55qx5d.cn", "xn--85x722f.xn--55qx5d.cn")
			run("a.b.example.uk.com", "example.uk.com")
		}
		elapsed := time.Since(startedAt)
		fmt.Printf("%8d iterations = %3.3fs\n", iterations, elapsed.Seconds())
	}
}
