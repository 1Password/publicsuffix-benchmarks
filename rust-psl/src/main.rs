extern crate psl;

use psl::{Psl, List};
use std::time::Instant;

fn run(list: &List, domain: &str, expected: &str) {
    let result = list.domain(domain).unwrap();
    if result.to_str() != expected {
        panic!("expected '{}' not '{:?}'", expected, result)
    }
}

fn main() {
    let list = List::new();

    {
        let iterations = 1000;
        let now = Instant::now();

        for _i in 0..iterations {
			run(&list, "abc.www.example.com", "example.com");
			run(&list, "abc.golang.org", "golang.org");
			run(&list, "www.食狮.中国", "食狮.中国");
			run(&list, "www.xn--85x722f.xn--55qx5d.cn", "xn--85x722f.xn--55qx5d.cn");
			run(&list, "a.b.example.uk.com", "example.uk.com");
        }

        println!("{:6} iterations = {:3}.{:03}s", iterations, now.elapsed().as_secs(), now.elapsed().subsec_millis());
    }

    {
        let iterations = 10_000;
        let now = Instant::now();

        for _i in 0..iterations {
			run(&list, "abc.www.example.com", "example.com");
			run(&list, "abc.golang.org", "golang.org");
			run(&list, "www.食狮.中国", "食狮.中国");
			run(&list, "www.xn--85x722f.xn--55qx5d.cn", "xn--85x722f.xn--55qx5d.cn");
			run(&list, "a.b.example.uk.com", "example.uk.com");
        }

        println!("{:6} iterations = {:3}.{:03}s", iterations, now.elapsed().as_secs(), now.elapsed().subsec_millis());
    }

    {
        let iterations = 100_000;
        let now = Instant::now();

        for _i in 0..iterations {
			run(&list, "abc.www.example.com", "example.com");
			run(&list, "abc.golang.org", "golang.org");
			run(&list, "www.食狮.中国", "食狮.中国");
			run(&list, "www.xn--85x722f.xn--55qx5d.cn", "xn--85x722f.xn--55qx5d.cn");
			run(&list, "a.b.example.uk.com", "example.uk.com");
        }

        println!("{:6} iterations = {:3}.{:03}s", iterations, now.elapsed().as_secs(), now.elapsed().subsec_millis());
    }

}
