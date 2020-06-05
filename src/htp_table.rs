use crate::bstr;
use std::cmp::Ordering;
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct htp_table_t<T> {
    pub elements: Vec<(bstr::bstr_t, T)>,
}

impl<T> Index<usize> for htp_table_t<T> {
    type Output = (bstr::bstr_t, T);
    fn index(&self, idx: usize) -> &(bstr::bstr_t, T) {
        &self.elements[idx]
    }
}

impl<T> htp_table_t<T> {
    /// Make a new owned htp_table_t with given capacity
    pub fn with_capacity(size: usize) -> Self {
        Self {
            elements: Vec::with_capacity(size),
        }
    }

    /// Add a new tuple (key, item) to the table
    pub fn add(&mut self, key: bstr::bstr_t, item: T) {
        self.elements.push((key, item));
    }

    /// Search the table for the first tuple with a key matching the given slice, ingnoring ascii case in self
    ///
    /// Returns None if no match is found.
    pub fn get_nocase<K: AsRef<[u8]>>(&self, key: K) -> Option<&(bstr::bstr_t, T)> {
        self.elements
            .iter()
            .find(|x| x.0.cmp_nocase(key.as_ref()) == Ordering::Equal)
    }

    /// Search the table for the first tuple with a tuple key matching the given slice, ignoring ascii case and any zeros in self
    ///
    /// Returns None if no match is found.
    pub fn get_nocase_nozero<K: AsRef<[u8]>>(&self, key: K) -> Option<&(bstr::bstr_t, T)> {
        self.elements
            .iter()
            .find(|x| x.0.cmp_nocase_nozero(key.as_ref()) == Ordering::Equal)
    }

    /// Returns the number of elements in the table
    pub fn size(&self) -> usize {
        self.elements.len()
    }
}

/// Allocate a htp_table_t with initial capacity size
///
/// Returns new htp_table_t instance
pub fn htp_table_alloc<T>(size: usize) -> *mut htp_table_t<T> {
    let t = htp_table_t::with_capacity(size);
    let boxed = Box::new(t);
    Box::into_raw(boxed)
}

/// Deallocate the supplied htp_table_t instance. Allows NULL on input.
pub fn htp_table_free<T>(t: *mut htp_table_t<T>) {
    if !t.is_null() {
        unsafe {
            // t will be dropped when this box goes out of scope
            Box::from_raw(t);
        }
    }
}

// Tests

#[test]
fn Add() {
    let mut t = htp_table_t::with_capacity(1);
    let mut k = bstr::bstr_t::from("Key");
    assert_eq!(0, t.size());
    t.add(k, "Value1");
    assert_eq!(1, t.size());
    k = bstr::bstr_t::from("AnotherKey");
    t.add(k, "Value2");
    assert_eq!(2, t.size());
}

#[test]
fn GetNoCase() {
    let mut t = htp_table_t::with_capacity(2);
    let mut k = bstr::bstr_t::from("Key1");
    t.add(k, "Value1");
    k = bstr::bstr_t::from("KeY2");
    t.add(k, "Value2");

    let mut result = t.get_nocase("KEY1");
    assert!(result.is_some());
    let mut res = result.unwrap();
    assert_eq!(Ordering::Equal, res.0.cmp("Key1"));
    assert_eq!("Value1", res.1);

    result = t.get_nocase("keY1");
    assert!(result.is_some());
    res = result.unwrap();
    assert_eq!(Ordering::Equal, res.0.cmp("Key1"));
    assert_eq!("Value1", res.1);

    result = t.get_nocase("key2");
    assert!(result.is_some());
    res = result.unwrap();
    assert_eq!(Ordering::Equal, res.0.cmp("KeY2"));
    assert_eq!("Value2", res.1);

    result = t.get_nocase("NotAKey");
    assert!(result.is_none());
}

#[test]
fn GetNocaseNozero() {
    let mut t = htp_table_t::with_capacity(2);
    let mut k = bstr::bstr_t::from("K\x00\x00\x00\x00ey\x001");
    t.add(k, "Value1");
    k = bstr::bstr_t::from("K\x00e\x00\x00Y2");
    t.add(k, "Value2");

    let mut result = t.get_nocase_nozero("key1");
    assert!(result.is_some());
    let mut res = result.unwrap();
    assert_eq!(Ordering::Equal, res.0.cmp("K\x00\x00\x00\x00ey\x001"));
    assert_eq!("Value1", res.1);

    result = t.get_nocase_nozero("KeY1");
    assert!(result.is_some());
    res = result.unwrap();
    assert_eq!(Ordering::Equal, res.0.cmp("K\x00\x00\x00\x00ey\x001"));
    assert_eq!("Value1", res.1);

    result = t.get_nocase_nozero("KEY2");
    assert!(result.is_some());
    res = result.unwrap();
    assert_eq!(Ordering::Equal, res.0.cmp("K\x00e\x00\x00Y2"));
    assert_eq!("Value2", res.1);

    result = t.get_nocase("key1");
    assert!(result.is_none());
}

#[test]
fn IndexAccess() {
    let mut t = htp_table_t::with_capacity(2);
    let mut k = bstr::bstr_t::from("Key1");
    t.add(k, "Value1");
    k = bstr::bstr_t::from("KeY2");
    t.add(k, "Value2");

    let res = &t[1];
    assert_eq!(Ordering::Equal, res.0.cmp("KeY2"));
    assert_eq!("Value2", res.1);
}
