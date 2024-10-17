use practice3chjhy3rpy2uz::list_parser;

pub fn main() {
    let v = list_parser::list("[1,1,2,3,5,8]");
    assert_eq!(&v, &Ok(vec![1, 1, 2, 3, 5, 8]));
    println!("parsed: {:#?}", v.unwrap());
}