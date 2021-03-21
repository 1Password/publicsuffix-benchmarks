use publicsuffix::{List, Psl};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

fn run(list: &List, domain: &str, expected: &str) {
    let root = list.domain(domain.as_bytes()).unwrap();
    assert_eq!(root, expected);
}

fn main() {
    let list = {
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = Path::new(&root).join("..").join("public_suffix_list.dat");
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents.parse().unwrap()
    };
    {
        let iterations = 1000;
        let now = Instant::now();

        for _i in 0..iterations {
            run(&list, "abc.www.example.com", "example.com");
            run(&list, "abc.golang.org", "golang.org");
            run(&list, "www.食狮.中国", "食狮.中国");
            run(
                &list,
                "www.xn--85x722f.xn--55qx5d.cn",
                "xn--85x722f.xn--55qx5d.cn",
            );
            run(&list, "a.b.example.uk.com", "example.uk.com");
        }

        let elapsed = now.elapsed();

        println!(
            "{:8} iterations = {:3}.{:03}s",
            iterations,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    }

    {
        let iterations = 10_000;
        let now = Instant::now();

        for _i in 0..iterations {
            run(&list, "abc.www.example.com", "example.com");
            run(&list, "abc.golang.org", "golang.org");
            run(&list, "www.食狮.中国", "食狮.中国");
            run(
                &list,
                "www.xn--85x722f.xn--55qx5d.cn",
                "xn--85x722f.xn--55qx5d.cn",
            );
            run(&list, "a.b.example.uk.com", "example.uk.com");
        }

        let elapsed = now.elapsed();

        println!(
            "{:8} iterations = {:3}.{:03}s",
            iterations,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    }

    {
        let iterations = 100_000;
        let now = Instant::now();

        for _i in 0..iterations {
            run(&list, "abc.www.example.com", "example.com");
            run(&list, "abc.golang.org", "golang.org");
            run(&list, "www.食狮.中国", "食狮.中国");
            run(
                &list,
                "www.xn--85x722f.xn--55qx5d.cn",
                "xn--85x722f.xn--55qx5d.cn",
            );
            run(&list, "a.b.example.uk.com", "example.uk.com");
        }

        let elapsed = now.elapsed();

        println!(
            "{:8} iterations = {:3}.{:03}s",
            iterations,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    }

    {
        let iterations = 10_000_000;
        let now = Instant::now();

        for _i in 0..iterations {
            run(&list, "abc.www.example.com", "example.com");
            run(&list, "abc.golang.org", "golang.org");
            run(&list, "www.食狮.中国", "食狮.中国");
            run(
                &list,
                "www.xn--85x722f.xn--55qx5d.cn",
                "xn--85x722f.xn--55qx5d.cn",
            );
            run(&list, "a.b.example.uk.com", "example.uk.com");
        }

        let elapsed = now.elapsed();

        println!(
            "{:8} iterations = {:3}.{:03}s",
            iterations,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    }
}
