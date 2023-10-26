#[cfg(test)]
mod tests {
    use std::slice;

    #[test]
    fn test_raw_pointer() {
        let mut num = 5;

        let r1 = &num as *const i32; // immutable raw pointer
        let r2 = &mut num as *mut i32; // mutable raw pointer

        unsafe {
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);

            *r2 += 1;

            assert_eq!(*r1, 6);
            assert_eq!(*r2, 6);
        }
    }

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    #[test]
    fn test_split_at_mut() {
        let mut s = [1, 2, 3, 4, 5];

        {
            let (s1, s2) = split_at_mut(&mut s, 3);

            s2[1] *= 2;

            assert_eq!(s1, [1, 2, 3]);
            assert_eq!(s2, [4, 10]);
        }

        assert_eq!(s, [1, 2, 3, 4, 10]);
    }

    #[test]
    fn test_ffi() {
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        let result: i32;

        unsafe {
            result = abs(-3);
        }

        assert_eq!(result, 3);
    }

    #[test]
    fn test_ffi2() {
        extern "C" fn call_from_c() -> *const u8 {
            "hoge-fuga".as_ptr()
        }

        let result = call_from_c();

        unsafe {
            assert_eq!(
                std::str::from_utf8_unchecked(std::slice::from_raw_parts(result, 4)),
                "hoge"
            );
        }
    }
}
