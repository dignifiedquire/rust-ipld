extern crate serde_cbor;

use std::collections::HashMap;
use serde_cbor::{to_vec, from_slice, Value, ObjectKey};

// From https://pyfisch.github.io/cbor/serde_cbor/value/enum.Value.html
// pub enum Value {
//     U64(u64),
//     I64(i64),
//     Bytes(Vec<u8>),
//     String(String),
//     Array(Vec<Value>),
//     Object(HashMap<ObjectKey, Value>),
//     F64(f64),
//     Bool(bool),
//     Null,
// }

#[derive(Clone, Debug, PartialEq)]
pub enum IpldSimpleLink {
    Vec,
    String,
}

// This needs to serialize to
// Value::Array(vec![link, props])
#[derive(Clone, Debug, PartialEq)]
pub struct IpldPropertyLink {
    link: IpldSimpleLink,
    props: HashMap<ObjectKey, Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IpldLink {
    IpldSimpleLink,
    IpldPropertyLink,
}

pub type IpldValue = Value;
pub type IpldObjectKey = ObjectKey;
pub type IpldObject = HashMap<IpldObjectKey, IpldValue>;

pub fn print(obj: IpldObject) -> () {
    let encoded = to_vec(&obj).unwrap();
    let decoded: IpldObject = from_slice(&encoded).unwrap();
    println!("enc {:?}", encoded);
    println!("dec {:?}", decoded);
}

pub type IpldPath = Vec<IpldObjectKey>;

pub fn cat<'a>(obj: &'a Value, path: IpldPath) -> &'a IpldValue {
    path.iter()
        .fold(obj, |acc, x| {
            match *acc {
                Value::U64(_) => acc,
                Value::I64(_) => acc,
                Value::Bytes(_) => acc,
                Value::String(_) => acc,
                Value::Array(ref vec) => {
                    match *x {
                        ObjectKey::Integer(i) => &vec[i as usize],
                        _ => panic!("Can not access array"),
                    }
                }
                Value::Object(ref map) => map.get(x).unwrap(),
                Value::F64(_) => acc,
                Value::Bool(_) => acc,
                Value::Null => acc,
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use serde_cbor::{Value, ObjectKey};

    #[test]
    fn test_cat_file() {
        let mut file = IpldObject::new();
        file.insert(ObjectKey::String("data".to_string()),
                    Value::String("hello world".to_string()));
        file.insert(ObjectKey::String("size".to_string()), Value::U64(11));

        let file_val = Value::Object(file);
        let result = cat(&file_val, vec![ObjectKey::String("data".to_string())]);

        let val = match result {
            &Value::String(ref val) => val,
            _ => panic!("Wrong value"),
        };

        assert_eq!(val, "hello world");
    }

    #[test]
    fn test_cat_chunked_file() {
        let mut chunk_1 = HashMap::new();
        chunk_1.insert(ObjectKey::String("@link".to_string()),
                       Value::String("QmAAA".to_string()));
        chunk_1.insert(ObjectKey::String("size".to_string()), Value::U64(100324));

        let mut chunk_2 = HashMap::new();
        chunk_2.insert(ObjectKey::String("@link".to_string()),
                       Value::String("QmBBB".to_string()));
        chunk_2.insert(ObjectKey::String("size".to_string()), Value::U64(120345));

        let mut file = IpldObject::new();
        file.insert(ObjectKey::String("size".to_string()), Value::U64(1424119));
        file.insert(ObjectKey::String("subfiles".to_string()),
                    Value::Array(vec![Value::Object(chunk_1), Value::Object(chunk_2)]));

        let file_val = Value::Object(file);
        let result = cat(&file_val,
                         vec![ObjectKey::String("subfiles".to_string()),
                              ObjectKey::Integer(1),
                              ObjectKey::String("@link".to_string())]);

        let val = match result {
            &Value::String(ref val) => val,
            _ => panic!("Wrong value"),
        };

        assert_eq!(val, "QmBBB");
    }
}
