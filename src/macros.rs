/// Check that type sizes are equal at compile time
#[macro_export]
macro_rules! assert_eq_size {
    ($x:ty, $($xs:ty),+ $(,)?) => {
        const _: fn() = || {
            $(let _ = core::mem::transmute::<$x, $xs>;)+
        };
    };
}

/// Check expression at compile time
#[macro_export]
macro_rules! const_assert {
    ($x:expr $(,)?) => {
        const _: [(); 0 - !{
            const ASSERT: bool = $x;
            ASSERT
        } as usize] = [];
    };
}
