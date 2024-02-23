pub mod my_error;
pub mod my_iterator_ext;

pub use self::{my_error::MyError, my_iterator_ext::MyIteratorExt};

// cannot bc trait is sealed
// impl<I,F> MyIteratorExt for std::iter::Map<I,F> {

// }

/// The doc test
/// ```compile_fail
/// MyError::type_id("a static ref", ??)
/// ```
fn run_me() {}
