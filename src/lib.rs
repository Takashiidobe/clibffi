#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::*;
    use std::ffi::c_void;
    use std::ffi::CString;

    macro_rules! cstr_ptr {
        ($a:expr) => {
            CString::new($a).unwrap().as_ptr()
        };
    }

    macro_rules! pair {
        ($a:expr, $b:expr) => {
            Pair {
                first: $a as *const c_void,
                second: $b as *const c_void,
            }
        };
    }

    #[no_mangle]
    pub extern "C" fn add_one(num: i32) -> i32 {
        num + 1
    }

    #[test]
    fn test_vector_for_each() {
        let v = unsafe { vector_new(3, 1, 2, 3) };
        unsafe { vector_for_each(v, Some(add_one)) };
        assert_eq!(unsafe { *vector_begin(v) }, 2);
    }

    fn test_pair_eq(left: Pair, right: Pair) {
        assert_eq!(
            unsafe { pair_eq(&left as *const Pair, &right as *const Pair) },
            1
        );
    }

    #[test]
    fn test_md5() {
        let s = [30, 30, 30, 30, 30].as_ptr() as *const u8;
        let mut buf: [u32; 4] = <[u32; 4]>::default();
        unsafe { md5(s, 2, buf.as_mut_ptr()) };
        assert_eq!(buf, [3415246051, 946725277, 2183009011, 4153239883]);
    }

    #[test]
    fn test_tuple_eq() {
        let left = pair!(cstr_ptr!("hi"), cstr_ptr!("hi"));
        let right = pair!(cstr_ptr!("hi"), cstr_ptr!("hi"));
        test_pair_eq(left, right);
    }

    #[test]
    fn test_djb2_hash_empty() {
        let res = unsafe { djb2_hash(cstr_ptr!("")) };
        assert_eq!(res, 5381);
    }

    #[test]
    fn test_djb2_hash_1() {
        let res = unsafe { djb2_hash(cstr_ptr!("hello")) };
        assert_eq!(res, 261238937);
    }

    #[test]
    fn test_adler_32_empty() {
        let res = unsafe { adler32(cstr_ptr!("")) };
        assert_eq!(res, 1);
    }

    #[test]
    fn test_fnv_32_empty() {
        let res = unsafe { fnv_32(cstr_ptr!("")) };
        assert_eq!(res, 2166136261);
    }

    #[test]
    fn test_fnv_64_empty() {
        let res = unsafe { fnv_64(cstr_ptr!("")) };
        assert_eq!(res, 14695981039346656037);
    }

    #[test]
    fn test_adler_32() {
        let res = unsafe { adler32(cstr_ptr!("hello")) };
        assert_eq!(res, 103547413);
    }

    #[test]
    fn test_fnv_64_1() {
        let res = unsafe { fnv_64(cstr_ptr!("Hello")) };
        assert_eq!(res, 7201466553693376363);
    }

    #[test]
    fn test_fnv_64_2() {
        let res = unsafe { fnv_64(cstr_ptr!("hello")) };
        assert_eq!(res, 11831194018420276491);
    }
}
