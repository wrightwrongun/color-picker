/* ---------------------------------------------------------------------------
    slintutils.rs

    Helper functions for use with slint/rust interactions.

    MW 2024
--------------------------------------------------------------------------- */

use std::rc::Rc;

use slint::{Model, ModelRc, VecModel};

mod strings;


/// Converts an array of type `T` to a slint array of type `T`.
/// 
/// Use when passing a Rust array into slint.
pub fn wrap_array<T>(array: &[T]) -> ModelRc<T>
where T: Clone + 'static {
    ModelRc::from(array)
}

/// Converts an array of type `T` to a slint array of type `V`.
/// 
/// Use when passing a Rust array into slint.
pub fn cast_array<T,V>(array: &[T]) -> ModelRc<V>
where T: Clone + 'static,
        V: Clone + From<T> + 'static {
        map_to_array(&Vec::from(array), |x| V::from(x.clone()))
}

/// Converts a `Vector` of type `T` to a slint array of type `T`.
/// 
/// Use when passing a Rust `Vector` into slint as an array.
pub fn to_array<T>(vec: Vec<T>) -> ModelRc<T>
where T: Clone + 'static {
    ModelRc::from(Rc::new(VecModel::from(vec)))
}

/// Converts a `Vector` of type `T` to a slint array of type `T` by
/// calling a mapping function for each item in the `Vector`.
/// 
/// Use when passing a Rust `Vector` into slint as an array.
pub fn map_to_array<T,V>(vec: &[T], f: impl Fn(&T) -> V) -> ModelRc<V>
where V: Clone + 'static {
    to_array(vec.iter().map(f).collect())
}

/// Converts a slint array of type `T` to a `Vector` of type `T`.
///
/// Use when a slint array is passed into Rust.
/// 
/// If the array cannot be converted (e.g. because of a type problem) then
/// a `None` will be returned.
pub fn from_array<T>(model: ModelRc<T>) -> Option<Vec<T>>
where T: Clone + 'static {
    Some(model.as_any().downcast_ref::<VecModel<T>>()?.iter().collect())
}

/// Converts a slint array of type `T` to a `Vector` of type `V` by
/// calling a mapping function for each item in the array.
///
/// Use when a slint array is passed into Rust.
/// 
/// If the array cannot be converted (e.g. because of a type problem) then
/// a `None` will be returned.
pub fn map_from_array<T,V>(model: ModelRc<T>, f: impl Fn(&T) -> V) -> Option<Vec<V>>
where T: Clone + 'static {
    Some(model.as_any().downcast_ref::<VecModel<T>>()?.iter().map(|x| f(&x)).collect())
}
