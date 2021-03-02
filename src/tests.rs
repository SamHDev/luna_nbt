use crate::blob::Blob;
use crate::NBTWrite;
use crate::front::NBTRead;
use serde::Serialize;

#[test]
fn blob_example() {
    let mut blob = Blob::create("hello world");
    blob.insert("name", "Bananrama");

    let data = blob.encode().unwrap();
    println!("{:?}", &data);

    let decoded = Blob::decode(&data).unwrap();

    println!("{:?}", decoded)
}

#[derive(Serialize)]
pub struct Example {
    foo: String,
    bar: i8,
    baz: Vec<i32>
}

#[test]
fn ser_example() {
    let data = Example {
        foo: "Hello World!".to_string(),
        bar: 69,
        baz: vec![420, 69, 666]
    };

    println!("{:?}", crate::encode(&data));

    std::fs::write("test.nbt", crate::encode(&data).unwrap().encode().unwrap());

}