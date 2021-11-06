mod complex_traits {
    /// A structured Reward, this will generally be implemented
    pub trait Reward<'a, T, U, V, W>:
        Ord
        + Composable<U, Output = V>
        + Bellman<V, Output = Self>
        + Composable<&'a Self, Output = W>
        + Bellman<W, Output = Self>
        + 'a
    {
    }

    impl<'a, T, U, V, W> Reward<'a, T, U, V, W> for T where
        T: Ord
            + Composable<U, Output = V>
            + Bellman<V, Output = T>
            + Composable<&'a Self, Output = W>
            + Bellman<W, Output = Self>
            + 'a
    {
    }

    pub trait Bellman<Rhs = Self> {
        type Output;

        fn update(&self, other: Rhs) -> Self::Output;
    }

    pub trait Composable<Rhs = Self> {
        type Output;

        fn compose(&self, other: Rhs) -> Self::Output;
    }
}
