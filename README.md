# colomy

colomy is nothing

```
$ make run
cargo run
   Compiling colomy v0.1.0 (/root/code/rust/colomy)
    Finished dev [unoptimized + debuginfo] target(s) in 1.01s
     Running `target/debug/colomy`
Event {
    fields: {
        "job.type": String(
            "engineer"
        ),
        "age": Number(
            43
        ),
        "phones": String(
            "[\"+44 1234567\",\"+44 2345678\"]"
        ),
        "alive": Bool(
            true
        ),
        "name": String(
            "John Doe"
        ),
        "job.firm": String(
            "Grunnings"
        )
    }
}
key job.type  value engineer
key age  value 43
key phones  value ["+44 1234567","+44 2345678"]
key alive  value true
key name  value John Doe
key job.firm  value Grunnings
```

## Notes

Event
- json
- figures out fields

Database
- takes Event, examines fields
- determines Indexes to be (created and) written to 
- issues writes to each Index
- gets required Readers from Indexes on query
- reads from Readers in parallel to assemble query results

Index
- field name, type
- has file for `field_name.type`
- issues writes to end of file
- open for reading and return Reader

Reader
- return next id:value

"Why We Built Our Own Distributed Column Store" by Sam Stokes
https://www.youtube.com/watch?v=tr2KcekX2kk

"Level Up Your Concurrency Skills With Rust"
https://www.youtube.com/watch?v=oIikwmeGVYY

## Reference

Generic fields, generic index, but how to check types on insert? https://gist.github.com/rust-play/01cad47f8d779073392c23ddddabd48d

With an enum and a trait? https://gist.github.com/rust-play/6363a66d6d4f0791ca1f24eff4a031d1
