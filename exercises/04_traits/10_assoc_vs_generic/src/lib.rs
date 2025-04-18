// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.
trait Power<T> {
    fn power(self, pow: T) -> Self;
}

macro_rules! impl_power {
    ($base: ty, $exp: ty, $converter: expr) => {
        impl Power<$exp> for $base {
            fn power(self, pow: $exp) -> Self {
                self.pow($converter(pow))
            }
        }
    };
}

// impl_power!(u32, u16, pow as u32);
impl_power!(u32, u16, |x| x as u32);
impl_power!(u32, u32, |x| x);
impl_power!(u32, &u32, |x: &u32| *x);

// impl Power<u16> for u32 {
//     fn power(self, pow: u16) -> Self {
//         self.pow(pow as u32)
//     }
// }

// impl Power<u32> for u32 {
//     fn power(self, pow: u32) -> Self {
//         self.pow(pow)
//     }
// }

// impl Power<&u32> for u32 {
//     fn power(self, pow: &u32) -> Self {
//         self.pow(pow.clone())
//     }
// }

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
