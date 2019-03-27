# publicsuffix-benchmarks

Testing performance of public suffix list libraries.

## Results

On 2018 MacBook Pro (2.9 GHz Intel Core i9):

```bash
$ make -s

 Testing https://godoc.org/golang.org/x/net/publicsuffix
  1000 iterations = 0.001s
 10000 iterations = 0.008s
100000 iterations = 0.074s

 Testing https://docs.rs/publicsuffix/1.5.2/publicsuffix/
  1000 iterations =   0.378s
 10000 iterations =   3.534s
100000 iterations =  34.487s

 Testing https://docs.rs/psl/0.4.1/psl/
  1000 iterations =   0.000s
 10000 iterations =   0.002s
100000 iterations =   0.021s
```