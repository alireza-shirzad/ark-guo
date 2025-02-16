use ark_guo::integer::ZZ;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

fn main() {
    println!("Hello, world!");

    let a = ZZ::from(1124131321);
    let b = ZZ::from(2);
    let c = a + b;
    println!("{}", c);

    let mut serialized = Vec::new();
    c.serialize_uncompressed(&mut serialized).unwrap();

    println!("{:?}", serialized);

    let deserialized = ZZ::deserialize_uncompressed(&mut serialized.as_slice()).unwrap();
    println!("{}", deserialized);

}