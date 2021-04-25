use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
pub struct RedisLess {
    data_mapper: HashMap<Vec<u8>, DataType>,
    string_store: HashMap<Vec<u8>, Vec<u8>>,
}

enum DataType {
    String,
    List,
    Set,
    Hash,
}

impl RedisLess {
    pub fn new() -> Self {
        RedisLess {
            data_mapper: HashMap::new(),
            string_store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &[u8], value: &[u8]) {
        self.data_mapper.insert(key.to_vec(), DataType::String);
        self.string_store.insert(key.to_vec(), value.to_vec());
    }

    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.string_store.get(key).map(|v| &v[..])
    }

    pub fn del(&mut self, key: &[u8]) -> u32 {
        match self.data_mapper.get(key) {
            Some(data_type) => match data_type {
                DataType::String => match self.string_store.remove(key) {
                    Some(_) => 1,
                    None => 0,
                },
                DataType::List => 0,
                DataType::Set => 0,
                DataType::Hash => 0,
            },
            None => 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn redisless_new() -> *mut RedisLess {
    Box::into_raw(Box::new(RedisLess::new()))
}

#[no_mangle]
pub extern "C" fn redisless_set(
    redisless: &mut RedisLess,
    key: *const c_char,
    value: *const c_char,
) {
    if key.is_null() || value.is_null() {
        return;
    }

    let key_str = unsafe { CStr::from_ptr(key) };
    let value_str = unsafe { CStr::from_ptr(value) };

    if let Ok(key) = key_str.to_str() {
        if let Ok(value) = value_str.to_str() {
            redisless.set(key.as_bytes(), value.as_bytes());
        }
    }
}

// #[no_mangle]
// pub extern "C" fn redisless_get(redisless: &mut RedisLess, key: *const c_char) -> *const c_char {
//     if key.is_null() {
//         return ptr::null();
//     }
//
//     let key_str = unsafe { CStr::from_ptr(key) };
//
//     if let Ok(key) = key_str.to_str() {
//         return match redisless.get(key.as_bytes()) {
//             Some(x) => {
//                 let ptr = CString::new("").unwrap();
//                 let ptr = ptr.as_ptr();
//                 unsafe { *ptr };
//             }
//             None => ptr::null(),
//         };
//     }
//
//     ptr::null()
// }

#[cfg(test)]
mod tests {
    use crate::RedisLess;

    #[test]
    fn test_set_get_and_del() {
        let mut redisless = RedisLess::new();
        redisless.set(b"key", b"xxx");
        assert_eq!(redisless.get(b"key"), Some(&b"xxx"[..]));
        assert_eq!(redisless.del(b"key"), 1);
        assert_eq!(redisless.del(b"key"), 0);
        assert_eq!(redisless.get(b"does not exist"), None);
    }
}