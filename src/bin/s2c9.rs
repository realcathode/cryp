use cryp::pkcs7_padding;

fn main() {
    let s= String::from("YELLOW SUBMARINE");
    match pkcs7_padding(s.as_bytes(), 20) {
        Some(v) => println!("{:?}", v),
        None => println!("err padding"),
    };

}