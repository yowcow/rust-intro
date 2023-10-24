pub struct CustomSmartPointer<'a> {
    ref_to_count: &'a mut i32,
}

impl<'a> CustomSmartPointer<'a> {
    pub fn new(ref_to_count: &'a mut i32) -> Self {
        Self { ref_to_count }
    }

    pub fn current_count(&self) -> i32 {
        *self.ref_to_count
    }
}

impl<'a> Drop for CustomSmartPointer<'a> {
    fn drop(&mut self) {
        *self.ref_to_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implicit_drop() {
        let mut drop_count = 0;

        {
            let p = CustomSmartPointer::new(&mut drop_count);

            assert_eq!(p.current_count(), 0);
        }

        assert_eq!(drop_count, 1);
    }

    #[test]
    fn test_explicit_drop() {
        let mut drop_count = 0;

        {
            let p = CustomSmartPointer::new(&mut drop_count);

            assert_eq!(p.current_count(), 0);

            drop(p);

            assert_eq!(drop_count, 1);
        }

        assert_eq!(drop_count, 1);
    }
}
