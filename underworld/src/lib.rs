extern crate libc;

use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::mem::transmute;


#[no_mangle]
pub extern fn hello_world() {
    println!("hello Rust—Python simple FFI world");
}


#[no_mangle]
pub extern fn sorted_string_counter() -> *mut BTreeMap<String, i64> {
    let our_counter = unsafe {
        transmute(Box::new(BTreeMap::<String, i64>::new()))
    };
    our_counter
}


#[no_mangle]
pub extern fn increment(counter: *mut BTreeMap<String, i64>,
                        key: *const libc::c_char) {
    let our_counter = unsafe { &mut *counter };
    let buffer = unsafe { CStr::from_ptr(key).to_bytes() };
    let string = String::from_utf8(buffer.to_vec()).unwrap();
    *our_counter.entry(string).or_insert(0) += 1;
}


#[no_mangle]
pub extern fn get(counter: *mut BTreeMap<String, i64>,
                  key: *const libc::c_char) -> i64 {
    let our_counter = unsafe { &mut *counter };
    let buffer = unsafe { CStr::from_ptr(key).to_bytes() };
    let string = String::from_utf8(buffer.to_vec()).unwrap();
    *our_counter.get(&string).unwrap_or(&0)
}


#[no_mangle]
pub extern fn represent(counter: *mut BTreeMap<String, i64>)
                        -> *const libc::c_char {
    let our_counter = unsafe { &mut *counter };
    CString::new(format!("{:?}", our_counter).as_bytes()).unwrap().into_raw()
}


#[no_mangle]
pub extern fn display(counter: *mut BTreeMap<String, i64>) {
    let our_counter = unsafe { &mut *counter };
    println!("{:?}", our_counter);
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    #[test]
    fn concerning_whether_btreemap_behaves_like_one_would_expect() {
        let mut my_treemap = BTreeMap::<String, i64>::new();
        my_treemap.insert("foo".to_owned(), 1);
        assert_eq!(my_treemap.get(&"foo".to_owned()), Some(&1i64));
    }

}
