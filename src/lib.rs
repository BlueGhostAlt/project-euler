pub mod traits {
    pub trait Integer {
        fn one() -> Self;
    }

    macro_rules! int_impl {
        ($t:ty) => {
            impl Integer for $t {
                #[inline]
                fn one() -> Self {
                    1
                }
            }
        };
    }

    int_impl!(u8);
    int_impl!(u16);
    int_impl!(u32);
    int_impl!(u64);
    int_impl!(u128);
    int_impl!(usize);
    int_impl!(i8);
    int_impl!(i16);
    int_impl!(i32);
    int_impl!(i64);
    int_impl!(i128);
    int_impl!(isize);
}

pub mod fib {
    use std::{mem, ops};

    use super::traits;

    pub struct Fibonacci<T> {
        current: T,
        next: T,
    }

    impl<T: traits::Integer> Fibonacci<T> {
        pub fn new() -> Self {
            Self {
                current: T::one(),
                next: T::one(),
            }
        }
    }

    impl<T: ops::Add<T, Output = T> + Clone> Iterator for Fibonacci<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            let new_next = self.current.clone() + self.next.clone();
            let new_current = mem::replace(&mut self.next, new_next);

            Some(mem::replace(&mut self.current, new_current))
        }
    }
}
