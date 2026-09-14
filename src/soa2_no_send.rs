// use std::vec;

// pub struct NoSendTyp2 {
//     id: u32,
//     c: *mut char,
// }

// #[derive(Clone, Copy)]
// pub struct Typ2Ptr {
//     pub id: *const u32,
//     pub c: *const *mut char,
// }

// impl Typ2Ptr {
//     pub unsafe fn add(self, count: usize) -> Self {
//         Self {
//             id: unsafe { self.id.add(count) },
//             c: unsafe { self.c.add(count) },
//         }
//     }
// }

// #[derive(Clone, Copy)]
// pub struct Typ2MutPtr {
//     pub id: *mut u32,
//     pub c: *mut *mut char,
// }

// impl Typ2MutPtr {
//     pub unsafe fn add(self, count: usize) -> Self {
//         Self {
//             id: unsafe { self.id.add(count) },
//             c: unsafe { self.c.add(count) },
//         }
//     }

//     pub unsafe fn copy_from_nonoverlapping(self, src: Typ2Ptr, count: usize) {
//         unsafe { self.id.copy_from_nonoverlapping(src.id, count) };
//         unsafe { self.c.copy_from_nonoverlapping(src.c, count) };
//     }
// }

// pub struct Typ2Vec {
//     id: Vec<u32>,
//     c: Vec<*mut char>,
// }

// impl Typ2Vec {
//     // ctor & dtor

//     pub fn new() -> Self {
//         Self {
//             id: Vec::new(),
//             c: Vec::new(),
//         }
//     }

//     pub fn with_capacity(capacity: usize) -> Self {
//         Self {
//             id: Vec::with_capacity(capacity),
//             c: Vec::with_capacity(capacity),
//         }
//     }

//     pub fn into_inner(self) -> (Vec<u32>, Vec<*mut char>) {
//         (self.id, self.c)
//     }

//     // get

//     pub fn len(&self) -> usize {
//         self.id.len()
//     }

//     pub fn is_empty(&self) -> bool {
//         self.id.is_empty()
//     }

//     pub fn id(&self) -> &[u32] {
//         &self.id
//     }

//     pub fn c(&self) -> &[*mut char] {
//         &self.c
//     }

//     pub fn as_ptr(&self) -> Typ2Ptr {
//         Typ2Ptr {
//             id: self.id.as_ptr(),
//             c: self.c.as_ptr(),
//         }
//     }

//     // mut

//     pub fn push(&mut self, item: NoSendTyp2) {
//         self.id.push(item.id);
//         self.c.push(item.c);
//     }

//     pub fn as_mut_ptr(&mut self) -> Typ2MutPtr {
//         Typ2MutPtr {
//             id: self.id.as_mut_ptr(),
//             c: self.c.as_mut_ptr(),
//         }
//     }

//     pub fn id_mut(&mut self) -> &mut [u32] {
//         &mut self.id
//     }

//     pub fn c_mut(&mut self) -> &mut [*mut char] {
//         &mut self.c
//     }

//     pub fn reserve(&mut self, additional: usize) {
//         self.id.reserve(additional);
//         self.c.reserve(additional);
//     }

//     pub unsafe fn set_len(&mut self, new_len: usize) {
//         unsafe { self.id.set_len(new_len) };
//         unsafe { self.c.set_len(new_len) };
//     }
// }

// impl Default for Typ2Vec {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl From<Typ2Vec> for (Vec<u32>, Vec<*mut char>) {
//     fn from(value: Typ2Vec) -> Self {
//         (value.id, value.c)
//     }
// }

// impl Extend<NoSendTyp2> for Typ2Vec {
//     fn extend<I: IntoIterator<Item = NoSendTyp2>>(&mut self, iter: I) {
//         for x in iter {
//             self.id.push(x.id);
//             self.c.push(x.c);
//         }
//     }
// }

// // iterators

// pub struct Typ2Iter {
//     id: vec::IntoIter<u32>,
//     c: vec::IntoIter<*mut char>,
// }

// impl Iterator for Typ2Iter {
//     type Item = NoSendTyp2;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.id.next().map(|id| {
//             // SAFETY: `id` and `c` has exactly same length
//             let c = unsafe { self.c.next().unwrap_unchecked() };
//             NoSendTyp2 { id, c }
//         })
//     }
// }

// impl IntoIterator for Typ2Vec {
//     type Item = NoSendTyp2;

//     type IntoIter = Typ2Iter;

//     fn into_iter(self) -> Self::IntoIter {
//         Typ2Iter {
//             id: self.id.into_iter(),
//             c: self.c.into_iter(),
//         }
//     }
// }

// pub struct Typ2Ref<'a> {
//     pub id: &'a u32,
//     pub c: &'a *mut char,
// }

// pub struct Typ2IterRef<'a> {
//     id: core::slice::Iter<'a, u32>,
//     c: core::slice::Iter<'a, *mut char>,
// }

// impl<'a> Iterator for Typ2IterRef<'a> {
//     type Item = Typ2Ref<'a>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.id.next().map(|id| {
//             // SAFETY: `id` and `c` has exactly same length
//             let c = unsafe { self.c.next().unwrap_unchecked() };
//             Typ2Ref { id, c }
//         })
//     }
// }

// impl<'a> IntoIterator for &'a Typ2Vec {
//     type Item = Typ2Ref<'a>;

//     type IntoIter = Typ2IterRef<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         Typ2IterRef {
//             id: self.id.iter(),
//             c: self.c.iter(),
//         }
//     }
// }

// pub struct Typ2Mut<'a> {
//     pub id: &'a mut u32,
//     pub c: &'a mut *mut char,
// }

// pub struct Typ2IterMut<'a> {
//     id: core::slice::IterMut<'a, u32>,
//     c: core::slice::IterMut<'a, *mut char>,
// }

// impl<'a> Iterator for Typ2IterMut<'a> {
//     type Item = Typ2Mut<'a>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.id.next().map(|id| {
//             // SAFETY: `id` and `c` has exactly same length
//             let c = unsafe { self.c.next().unwrap_unchecked() };
//             Typ2Mut { id, c }
//         })
//     }
// }

// impl<'a> IntoIterator for &'a mut Typ2Vec {
//     type Item = Typ2Mut<'a>;

//     type IntoIter = Typ2IterMut<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         Typ2IterMut {
//             id: self.id.iter_mut(),
//             c: self.c.iter_mut(),
//         }
//     }
// }
