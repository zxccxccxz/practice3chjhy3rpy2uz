## My Parser

### Description

Parser for educational purposes

![picture text](photo.jpg)

### Example

```rust
let v = list_parser::list("[1,1,2,3,5,8]");
assert_eq!(&v, &Ok(vec![1, 1, 2, 3, 5, 8]));
println!("parsed: {:#?}", v.unwrap());
```