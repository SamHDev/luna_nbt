use crate::blob::Blob;
use crate::NBTWrite;
use crate::front::NBTRead;

#[test]
fn blob_example() {
    let mut blob = Blob::create("hello world");
    blob.insert("name", "Bananrama");

    let data = blob.bytes().unwrap();
    println!("{:?}", &data);

    let decoded = Blob::from_bytes(&data).unwrap();

    println!("{:?}", decoded)
}
