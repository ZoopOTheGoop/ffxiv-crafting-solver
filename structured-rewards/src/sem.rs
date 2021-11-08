//! This module represents traits for different types of "semantic"
//! equality and ordering, which is distinguished from "structural" equality or ordering.
//!
//! Extending the concept of a [`QVal`] to an arbitrary structure requires exploiting what
//! it is about Q-values and Rewards that allow you to optimize over them - which is to say
//! finding the maximum. This requires defining an ordering over them.
//!
//! These traits allow you to define [`QVal`] structures that implement arbitrary notions of
//! equality and ordering. We could use [`Ord`] and [`Eq`] for this, however some things, notably
//! [`HashMap`], and similar types require [`Eq`] and [`Hash`](std::hash::Hash) to evaluate
//! the same when comparing two types. It may be desireable to store [`QVal`] estimates that
//! are not literally equal, but only "tied" separately.
//!
//! So this crate provides three important types: [`SemanticEq`] and [`SemanticOrd`] which are simply
//! the value-ranking equivalents of [`Eq`] and [`Ord`] and return a [`bool`] and [`Ordering`], respectively.
//! As well as [`Sem`], which is a simple wrapper over any type that at least implements [`SemanticEq`] which
//! implements the semantic equivalents **as** [`Eq`] and [`Ord`] themselves.
//!
//! The intention is that you can semantically compare two values without code becoming hard to read (as well
//! as leveraging things like [`max`](std::iter::Iterator::max)), simply by using [`sem()`](SemanticEq::sem)
//! to wrap it and then calling `.0` to extract the original type when done. This code will look like
//! `q_1.sem() > q_2.sem()` instead of `match q_1.sem_cmp(q_2) { Ordering::Greater => ... }`.
//!
//! Note that it is not wrong, such as in cases where [`QVal`] are simple numbers, for [`SemanticEq`] and [`Eq`]
//! (or [`PartialEq`]) (and their `Ord` equivalents) to be the same, in which case you can simple implement one as
//! calling the other.
//!
//! We do not provide an equivalent to [`PartialEq`] or [`PartialOrd`] because they're insufficient for the algorithms
//! at hand. This does mean you will need to fudge [`f32`] and [`f64`] in particlar (being the most traditional type
//! for RL rewards) as being [`Eq`] and [`Ord`] probably by using a [`debug_assert!`] over [`is_nan()`] and possibly
//! [`is_infinite()`] as well.
//!
//! [`QVal`]: crate::QVal
//! [`HashMap`]: std::collections::HashMap
//! [`is_nan()`]: https://doc.rust-lang.org/std/primitive.f64.html#method.is_nan
//! [`is_infinite()`]: https://doc.rust-lang.org/std/primitive.f64.html#method.is_infinite

use std::cmp::Ordering;

/// A simple wrapper type for any [`Sized`] type that implements
/// [`SemanticEq`]. It implements [`Ord`] and [`Eq`] so that they match
/// their semantic equivalents, so semantic comparisons can be done
/// cleanly in code without forcing users to forego structural notions of these
/// traits for, e.g., storing these types in certain collections.
///
/// It is almost always used with [`SemanticEq::sem`] in
/// algorithms, so that [`max`](std::iter::Iterator::max) and operators such as
/// `>` and `==` can be used, and then the original type extracted via `.0` after.
#[derive(Clone, Copy, Debug)]
pub struct Sem<T>(pub T);

impl<T> PartialEq for Sem<T>
where
    T: SemanticEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.sem_eq(&other.0)
    }
}

impl<T> Eq for Sem<T> where T: SemanticEq {}

impl<T> PartialOrd for Sem<T>
where
    T: SemanticOrd + SemanticEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.sem_cmp(&other.0))
    }
}

impl<T> Ord for Sem<T>
where
    T: SemanticOrd + SemanticEq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.sem_cmp(&other.0)
    }
}

/// Asserts the type has a notion of "semantic" equality, potentially
/// different from the type of equality that would distinguish two
/// structs from being literally, structurally equal in the way
/// [`Eq`] may imply or rely on for use with a thing like [`HashMap`](std::collections::HashMap).
///
/// This, along with [`SemanticOrd`], is a requirement for [`QVal`], and the notion of equivalence
/// should be roughly identical to the notion of the values being "tied" in that we have no preference
/// over which of the two is better. This may or may not be the same as regular equality.
///
/// Since this type has no [`PartialEq`] equivalent, it is recommended that for floats (a common
/// reward type, being the most traditional one), you explicitly for `NaN` or `inf` values if they're
/// wrapped in this, even if it's only checking for them in debug mode via [`debug_assert!`] or similar.
///
/// This type contains a default implementation called [`Sem`] which wraps the value in a struct that
/// converts this and [`SemanticOrd`] to their [`Eq`] and [`Ord`] equivalents for use with builtin operators
/// and functions like [`max`](std::iter::Iterator::max) that rely on these traits. The original type can be retrieved
/// with `.0` (i.e. `my_type.sem().0` returns `my_type`).
///
/// This trait is auto-implemented for `&T` and `&mut T` when impelemented on the owned variant.
///
/// [`QVal`]: crate::QVal
pub trait SemanticEq {
    fn sem_eq(&self, other: &Self) -> bool;

    fn sem(self) -> Sem<Self>
    where
        Self: Sized,
    {
        Sem(self)
    }
}

impl<T> SemanticEq for &T
where
    T: SemanticEq,
{
    fn sem_eq(&self, other: &Self) -> bool {
        (**self).sem_eq(*other)
    }
}

impl<T> SemanticEq for &mut T
where
    T: SemanticEq,
{
    fn sem_eq(&self, other: &Self) -> bool {
        (**self).sem_eq(&**other)
    }
}

/// Asserts the type has a notion of "semantic" ordering, potentially
/// different from the type of ordering that would distinguish two
/// structs from being literally, structurally ordered in the way
/// [`Ord`] may imply. This can be used as a key in a [`BTreeMap`](std::collections::BTreeMap),
/// however
///
/// This, along with [`SemanticEq`], is a requirement for [`QVal`], and the notion of order
/// should be roughly identical to the notion of the values being "better than", "worse than",
/// or "tied". As in `a.sem_eq(b) -> Ordering::Greater` implies it would be better to end up
/// with `a` than `b`, `Equal` implies there's no functional difference, and `Less` implies it's worse.
///
/// Since this type has no [`PartialOrd`] equivalent, it is recommended that for floats (a common
/// reward type, being the most traditional one), you explicitly for `NaN` or `inf` values if they're
/// wrapped in this, even if it's only checking for them in debug mode via [`debug_assert!`] or similar.
///
/// This trait is auto-implemented for `&T` and `&mut T` when impelemented on the owned variant.
///
/// [`QVal`]: crate::QVal
pub trait SemanticOrd: SemanticEq {
    fn sem_cmp(&self, other: &Self) -> Ordering;
}

impl<T> SemanticOrd for &T
where
    T: SemanticOrd,
{
    fn sem_cmp(&self, other: &Self) -> Ordering {
        (**self).sem_cmp(*other)
    }
}

impl<T> SemanticOrd for &mut T
where
    T: SemanticOrd,
{
    fn sem_cmp(&self, other: &Self) -> Ordering {
        (**self).sem_cmp(&**other)
    }
}
