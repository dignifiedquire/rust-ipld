extern crate serde_cbor;

use std::collections::HashMap;
use serde_cbor::{Value, ObjectKey};


pub trait IPLD {
    /// The type of an IPLD object
    type Object;

    /// Type for keys of an IPLD object
    type ObjectKey;

    /// Type for values of an IPLD object
    type Value;

    /// Representation of an IPLD path, e.g. /my/val
    type Path;

    /// Given any value, and a path resolve the path and return the
    /// value at the end.
    fn cat<'a>(&self, &'a Self::Value, Self::Path) -> &'a Self::Value;
}

pub struct CborIpld;

impl IPLD for CborIpld {
    type Object = HashMap<ObjectKey, Value>;
    type ObjectKey = serde_cbor::ObjectKey;
    type Value = serde_cbor::Value;

    type Path = Vec<ObjectKey>;

    fn cat<'a>(&self, obj: &'a Value, path: Vec<ObjectKey>) -> &'a Value {
        path.iter().fold(obj, |acc, x| {
            match *acc {
                Value::Array(ref vec) => {
                    match *x {
                        ObjectKey::Integer(i) => &vec[i as usize],
                        _ => panic!("Can not access array"),
                    }
                }
                Value::Object(ref map) => map.get(x).unwrap(),
                Value::U64(_)   |
                Value::I64(_)   |
                Value::Bytes(_) |
                Value::String(_)|
                Value::F64(_)   |
                Value::Bool(_)  |
                Value::Null     => acc,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use serde_cbor::{Value, ObjectKey};

    #[test]
    fn test_cat_file() {
        let mut file = HashMap::new();
        file.insert(ObjectKey::String("data".to_string()),
                    Value::String("hello world".to_string()));
        file.insert(ObjectKey::String("size".to_string()), Value::U64(11));

        let cbor_ipld = CborIpld;
        let file_val = Value::Object(file);
        let result = cbor_ipld.cat(&file_val, vec![ObjectKey::String("data".to_string())]);

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

        let mut file = HashMap::new();
        file.insert(ObjectKey::String("size".to_string()), Value::U64(1424119));
        file.insert(ObjectKey::String("subfiles".to_string()),
                    Value::Array(vec![Value::Object(chunk_1), Value::Object(chunk_2)]));

        let cbor_ipld = CborIpld;
        let file_val = Value::Object(file);
        let result = cbor_ipld.cat(&file_val,
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
