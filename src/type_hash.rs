use serde::*;

use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct MultihashType {
    pub val: multihash::Hash,
}

fn map_string_to_hash_meth(hash_method: String) -> Option<multihash::Hash> {
    match hash_method.as_ref() {
        "SHA1" => Some(multihash::Hash::SHA1),
        "SHA2256" => Some(multihash::Hash::SHA2256),
        "SHA2512" => Some(multihash::Hash::SHA2512),
        
        "SHA3512" => Some(multihash::Hash::SHA3512),
        "SHA3384" => Some(multihash::Hash::SHA3384),
        "SHA3256" => Some(multihash::Hash::SHA3256),
        "SHA3224" => Some(multihash::Hash::SHA3224),
        
        "Keccak224" => Some(multihash::Hash::Keccak224),
        "Keccak256" => Some(multihash::Hash::Keccak256),
        "Keccak384" => Some(multihash::Hash::Keccak384),
        "Keccak512" => Some(multihash::Hash::Keccak512),

        "Blake2b" => Some(multihash::Hash::Blake2b),
        "Blake2s" => Some(multihash::Hash::Blake2s),
        _ => None,
    }
}

fn map_hash_method_to_string(hash_meth: multihash::Hash) -> String {
    match hash_meth {
        multihash::Hash::SHA1 => "SHA1".to_string(),
        multihash::Hash::SHA2256 => "SHA2256".to_string(),
        multihash::Hash::SHA2512 => "SHA2512".to_string(),

        multihash::Hash::SHA3512 => "SHA3512".to_string(),
        multihash::Hash::SHA3384 => "SHA3384".to_string(),
        multihash::Hash::SHA3256 => "SHA3256".to_string(),
        multihash::Hash::SHA3224 => "SHA3224".to_string(),
            
        multihash::Hash::Keccak224 => "Keccak224".to_string(),
        multihash::Hash::Keccak256 => "Keccak256".to_string(),
        multihash::Hash::Keccak384 => "Keccak384".to_string(),
        multihash::Hash::Keccak512 => "Keccak512".to_string(),

        multihash::Hash::Blake2b => "Blake2b".to_string(),
        multihash::Hash::Blake2s => "Blake2s".to_string(),
    }
}

impl Serialize for MultihashType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    {
        serializer.serialize_str(&map_hash_method_to_string(self.val))
    }
}



impl<'de> Deserialize<'de> for MultihashType {
    fn deserialize<D>(deserializer: D) -> Result<MultihashType, D::Error> where D: Deserializer<'de>,
    {
        let estr : String = String::deserialize(deserializer)?;
        match map_string_to_hash_meth(estr) {
            Some(eval) => Ok(MultihashType { val: eval}),
            None => Err(de::Error::custom("Cannot deserialize hash".to_string())),
        }
    }
}

impl Default for MultihashType {
    fn default() -> MultihashType {
        MultihashType { val: multihash::Hash::Keccak256 }
    }
}

impl Debug for MultihashType {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Node({})", map_hash_method_to_string(self.val))
    }

}




