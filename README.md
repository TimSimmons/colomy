# colomy

colomy is nothing

```
$ make run
cargo run
   Compiling colomy v0.1.0 (/Users/tim/code/rust/colomy)
    Finished dev [unoptimized + debuginfo] target(s) in 0.83s
     Running `target/debug/colomy`
{"age":43,"alive":true,"job":{"firm":"Grunnings","type":"engineer"},"name":"John Doe","phones":["+44 1234567","+44 2345678"]}
{"job.type": String("engineer"), "age": Number(43), "name": String("John Doe"), "phones": String("[\"+44 1234567\",\"+44 2345678\"]"), "alive": Bool(true), "job.firm": String("Grunnings")}
key job.type  value engineer
key age  value 43
key name  value John Doe
key phones  value ["+44 1234567","+44 2345678"]
key alive  value true
key job.firm  value Grunnings
```
