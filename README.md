# publicsuffix-benchmarks

Testing performance of public suffix list libraries.

## Results

On Intel(R) Core(TM) i5-8265U CPU @ 1.60GHz:

```bash
$ make -s

 Testing https://godoc.org/golang.org/x/net/publicsuffix
    1000 iterations = 0.001s
   10000 iterations = 0.015s
  100000 iterations = 0.100s
10000000 iterations = 9.794s

 Testing https://docs.rs/publicsuffix/2.0.10/publicsuffix/ (anycase)
    1000 iterations =   0.000s
   10000 iterations =   0.008s
  100000 iterations =   0.083s
10000000 iterations =   8.717s

 Testing https://docs.rs/publicsuffix/2.0.10/publicsuffix/
    1000 iterations =   0.000s
   10000 iterations =   0.002s
  100000 iterations =   0.025s
10000000 iterations =   2.839s

 Testing https://docs.rs/psl/2.0.8/psl/
    1000 iterations =   0.000s
   10000 iterations =   0.001s
  100000 iterations =   0.015s
10000000 iterations =   1.566s
```
