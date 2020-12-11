pub mod did;
use did::DID;

fn main() {
    let example = DID::new("example", "123456789asdfghj");
    println!("{}", example.parse());
}