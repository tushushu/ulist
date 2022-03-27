use crate::base::List;
use crate::boolean::BooleanList;
use crate::floatings::FloatList32;
use crate::floatings::FloatList64;
use crate::index::IndexList;
use crate::integers::IntegerList32;
use crate::integers::IntegerList64;
use crate::non_float::NonFloatList;
use crate::types::AsBooleanList;
use crate::types::AsFloatList32;
use crate::types::AsFloatList64;
use crate::types::AsIntegerList32;
use crate::types::AsIntegerList64;
use pyo3::prelude::*;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;

/// List with string type elements.
#[pyclass]
pub struct StringList {
    _values: RefCell<Vec<String>>,
}

#[pymethods]
impl StringList {
    // Arrange the following methods in alphabetical order.

    #[new]
    pub fn new(vec: Vec<String>) -> Self {
        List::_new(vec)
    }

    pub fn append(&self, elem: String) {
        List::append(self, elem)
    }

    pub fn as_bool(&self) -> BooleanList {
        AsBooleanList::as_bool(self)
    }

    pub fn as_float32(&self) -> FloatList32 {
        AsFloatList32::as_float32(self)
    }

    pub fn as_int32(&self) -> IntegerList32 {
        AsIntegerList32::as_int32(self)
    }

    pub fn as_int64(&self) -> IntegerList64 {
        AsIntegerList64::as_int64(self)
    }

    pub fn contains(&self, elem: &str) -> BooleanList {
        let vec = self.values().iter().map(|x| x.contains(elem)).collect();
        BooleanList::new(vec)
    }

    pub fn copy(&self) -> Self {
        List::copy(self)
    }

    pub fn counter(&self) -> HashMap<String, usize> {
        NonFloatList::counter(self)
    }

    #[staticmethod]
    pub fn cycle(vec: Vec<String>, size: usize) -> Self {
        List::cycle(&vec, size)
    }

    pub fn ends_with(&self, elem: &str) -> BooleanList {
        let vec = self.values().iter().map(|x| x.ends_with(elem)).collect();
        BooleanList::new(vec)
    }

    pub fn equal_scala(&self, elem: String) -> BooleanList {
        List::equal_scala(self, elem)
    }

    pub fn filter(&self, condition: &BooleanList) -> Self {
        List::filter(self, condition)
    }

    pub fn get(&self, index: usize) -> String {
        List::get(self, index)
    }

    pub unsafe fn get_by_indexes(&self, indexes: &IndexList) -> Self {
        List::get_by_indexes(self, indexes)
    }

    pub fn not_equal_scala(&self, elem: String) -> BooleanList {
        List::not_equal_scala(self, elem)
    }

    pub fn pop(&self) {
        List::pop(self);
    }

    #[staticmethod]
    pub fn repeat(elem: String, size: usize) -> Self {
        List::repeat(elem, size)
    }

    pub fn replace(&self, old: String, new: String) -> Self {
        List::replace(self, old, new)
    }

    pub unsafe fn set(&self, index: usize, elem: String) {
        List::set(self, index, elem)
    }

    pub fn size(&self) -> usize {
        List::size(self)
    }

    pub fn sort(&self, ascending: bool) -> Self {
        NonFloatList::sort(self, ascending)
    }

    pub fn starts_with(&self, elem: &str) -> BooleanList {
        let vec = self.values().iter().map(|x| x.starts_with(elem)).collect();
        BooleanList::new(vec)
    }

    pub fn to_list(&self) -> Vec<String> {
        List::to_list(self)
    }

    pub fn union_all(&self, other: &Self) -> Self {
        List::union_all(self, other)
    }

    pub fn unique(&self) -> Self {
        NonFloatList::unique(self)
    }
}

impl List<String> for StringList {
    fn _new(vec: Vec<String>) -> Self {
        Self {
            _values: RefCell::new(vec),
        }
    }

    fn values(&self) -> Ref<Vec<String>> {
        self._values.borrow()
    }

    fn values_mut(&self) -> RefMut<Vec<String>> {
        self._values.borrow_mut()
    }
}

impl NonFloatList<String> for StringList {}

impl AsBooleanList for StringList {
    fn as_bool(&self) -> BooleanList {
        let vec = self.values().iter().map(|x| x.parse().unwrap()).collect();
        BooleanList::new(vec)
    }
}

impl AsFloatList32 for StringList {
    fn as_float32(&self) -> FloatList32 {
        let vec = self.values().iter().map(|x| x.parse().unwrap()).collect();
        FloatList32::new(vec)
    }
}

impl AsFloatList64 for StringList {
    fn as_float64(&self) -> FloatList64 {
        let vec = self.values().iter().map(|x| x.parse().unwrap()).collect();
        FloatList64::new(vec)
    }
}

impl AsIntegerList32 for StringList {
    fn as_int32(&self) -> IntegerList32 {
        let vec = self.values().iter().map(|x| x.parse().unwrap()).collect();
        IntegerList32::new(vec)
    }
}

impl AsIntegerList64 for StringList {
    fn as_int64(&self) -> IntegerList64 {
        let vec = self.values().iter().map(|x| x.parse().unwrap()).collect();
        IntegerList64::new(vec)
    }
}
