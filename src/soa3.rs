use orx_parallel::collectables::{ColAndPos, IdxLen, ParExtendCore, ThBegLen};
use orx_priority_queue::{BinaryHeap, PriorityQueue};
use std::vec;

pub struct Typ3 {
    id: u32,
    c: char,
    name: String,
}

#[derive(Clone, Copy)]
pub struct Typ3Ptr {
    pub id: *const u32,
    pub c: *const char,
    pub name: *const String,
}

impl Typ3Ptr {
    pub unsafe fn add(self, count: usize) -> Self {
        Self {
            id: unsafe { self.id.add(count) },
            c: unsafe { self.c.add(count) },
            name: unsafe { self.name.add(count) },
        }
    }
}

#[derive(Clone, Copy)]
pub struct Typ3MutPtr {
    pub id: *mut u32,
    pub c: *mut char,
    pub name: *mut String,
}

impl Typ3MutPtr {
    pub unsafe fn add(self, count: usize) -> Self {
        Self {
            id: unsafe { self.id.add(count) },
            c: unsafe { self.c.add(count) },
            name: unsafe { self.name.add(count) },
        }
    }

    pub unsafe fn copy_from_nonoverlapping(self, src: Typ3Ptr, count: usize) {
        unsafe { self.id.copy_from_nonoverlapping(src.id, count) };
        unsafe { self.c.copy_from_nonoverlapping(src.c, count) };
        unsafe { self.name.copy_from_nonoverlapping(src.name, count) };
    }
}

pub struct Typ3Vec {
    id: Vec<u32>,
    c: Vec<char>,
    name: Vec<String>,
}

impl Typ3Vec {
    // ctor & dtor

    pub fn new() -> Self {
        Self {
            id: Vec::new(),
            c: Vec::new(),
            name: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            id: Vec::with_capacity(capacity),
            c: Vec::with_capacity(capacity),
            name: Vec::with_capacity(capacity),
        }
    }

    pub fn into_inner(self) -> (Vec<u32>, Vec<char>, Vec<String>) {
        (self.id, self.c, self.name)
    }

    // get

    pub fn len(&self) -> usize {
        self.id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.id.is_empty()
    }

    pub fn id(&self) -> &[u32] {
        &self.id
    }

    pub fn c(&self) -> &[char] {
        &self.c
    }

    pub fn name(&self) -> &[String] {
        &self.name
    }

    pub fn as_ptr(&self) -> Typ3Ptr {
        Typ3Ptr {
            id: self.id.as_ptr(),
            c: self.c.as_ptr(),
            name: self.name.as_ptr(),
        }
    }

    // mut

    pub fn push(&mut self, item: Typ3) {
        self.id.push(item.id);
        self.c.push(item.c);
        self.name.push(item.name);
    }

    pub fn as_mut_ptr(&mut self) -> Typ3MutPtr {
        Typ3MutPtr {
            id: self.id.as_mut_ptr(),
            c: self.c.as_mut_ptr(),
            name: self.name.as_mut_ptr(),
        }
    }

    pub fn id_mut(&mut self) -> &mut [u32] {
        &mut self.id
    }

    pub fn c_mut(&mut self) -> &mut [char] {
        &mut self.c
    }

    pub fn name_mut(&mut self) -> &mut [String] {
        &mut self.name
    }

    pub fn reserve(&mut self, additional: usize) {
        self.id.reserve(additional);
        self.c.reserve(additional);
        self.name.reserve(additional);
    }

    pub unsafe fn set_len(&mut self, new_len: usize) {
        unsafe { self.id.set_len(new_len) };
        unsafe { self.c.set_len(new_len) };
        unsafe { self.name.set_len(new_len) };
    }
}

impl Default for Typ3Vec {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Typ3Vec> for (Vec<u32>, Vec<char>, Vec<String>) {
    fn from(value: Typ3Vec) -> Self {
        (value.id, value.c, value.name)
    }
}

impl Extend<Typ3> for Typ3Vec {
    fn extend<I: IntoIterator<Item = Typ3>>(&mut self, iter: I) {
        for x in iter {
            self.id.push(x.id);
            self.c.push(x.c);
            self.name.push(x.name);
        }
    }
}

// iterators

pub struct Typ3Iter {
    id: vec::IntoIter<u32>,
    c: vec::IntoIter<char>,
    name: vec::IntoIter<String>,
}

impl Iterator for Typ3Iter {
    type Item = Typ3;

    fn next(&mut self) -> Option<Self::Item> {
        self.id.next().map(|id| {
            // SAFETY: `id` and `c` has exactly same length
            let c = unsafe { self.c.next().unwrap_unchecked() };
            // SAFETY: `name` and `c` has exactly same length
            let name = unsafe { self.name.next().unwrap_unchecked() };
            Typ3 { id, c, name }
        })
    }
}

impl IntoIterator for Typ3Vec {
    type Item = Typ3;

    type IntoIter = Typ3Iter;

    fn into_iter(self) -> Self::IntoIter {
        Typ3Iter {
            id: self.id.into_iter(),
            c: self.c.into_iter(),
            name: self.name.into_iter(),
        }
    }
}

pub struct Typ3Ref<'a> {
    pub id: &'a u32,
    pub c: &'a char,
    pub name: &'a String,
}

pub struct Typ3IterRef<'a> {
    id: core::slice::Iter<'a, u32>,
    c: core::slice::Iter<'a, char>,
    name: core::slice::Iter<'a, String>,
}

impl<'a> Iterator for Typ3IterRef<'a> {
    type Item = Typ3Ref<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.id.next().map(|id| {
            // SAFETY: `id` and `c` has exactly same length
            let c = unsafe { self.c.next().unwrap_unchecked() };
            // SAFETY: `id` and `name` has exactly same length
            let name = unsafe { self.name.next().unwrap_unchecked() };
            Typ3Ref { id, c, name }
        })
    }
}

impl<'a> IntoIterator for &'a Typ3Vec {
    type Item = Typ3Ref<'a>;

    type IntoIter = Typ3IterRef<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Typ3IterRef {
            id: self.id.iter(),
            c: self.c.iter(),
            name: self.name.iter(),
        }
    }
}

pub struct Typ3Mut<'a> {
    pub id: &'a mut u32,
    pub c: &'a mut char,
    pub name: &'a mut String,
}

pub struct Typ3IterMut<'a> {
    id: core::slice::IterMut<'a, u32>,
    c: core::slice::IterMut<'a, char>,
    name: core::slice::IterMut<'a, String>,
}

impl<'a> Iterator for Typ3IterMut<'a> {
    type Item = Typ3Mut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.id.next().map(|id| {
            // SAFETY: `id` and `c` has exactly same length
            let c = unsafe { self.c.next().unwrap_unchecked() };
            // SAFETY: `id` and `name` has exactly same length
            let name = unsafe { self.name.next().unwrap_unchecked() };
            Typ3Mut { id, c, name }
        })
    }
}

impl<'a> IntoIterator for &'a mut Typ3Vec {
    type Item = Typ3Mut<'a>;

    type IntoIter = Typ3IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Typ3IterMut {
            id: self.id.iter_mut(),
            c: self.c.iter_mut(),
            name: self.name.iter_mut(),
        }
    }
}

// parallel

impl ParExtendCore<Typ3> for Typ3Vec {
    type ThreadValues = Self;

    type OrderedThreadValues = ColAndPos<Self>;

    fn new_thread_values() -> Self::ThreadValues {
        Default::default()
    }

    fn new_ordered_thread_values() -> Self::OrderedThreadValues {
        Default::default()
    }

    // thread collect

    fn add_thread_value(collected: &mut Self::ThreadValues, value: Typ3) {
        collected.push(value);
    }

    fn add_thread_values(
        collected: &mut Self::ThreadValues,
        values: impl IntoIterator<Item = Typ3>,
    ) {
        collected.extend(values);
    }

    fn add_ordered_thread_value(
        collected: &mut Self::OrderedThreadValues,
        idx: usize,
        value: Typ3,
    ) {
        collected.values.push(value);
        collected.positions.push(IdxLen { idx, len: 1 });
    }

    fn add_ordered_thread_values(
        collected: &mut Self::OrderedThreadValues,
        idx: usize,
        values: impl IntoIterator<Item = Typ3>,
    ) {
        let len_begin = collected.values.len();
        collected.values.extend(values);

        let len = collected.values.len() - len_begin;
        if len > 0 {
            collected.positions.push(IdxLen { idx, len });
        }
    }

    // opt: thread collect

    fn add_ordered_thread_optionals(
        collected: &mut Self::OrderedThreadValues,
        idx: usize,
        values: impl IntoIterator<Item = Option<Typ3>>,
    ) -> Option<()> {
        let len_begin = collected.values.len();
        for value in values {
            collected.values.push(value?);
        }

        let len = collected.values.len() - len_begin;
        if len > 0 {
            collected.positions.push(IdxLen { idx, len });
        }

        Some(())
    }
    // res: thread collect

    fn add_ordered_thread_fallibles<E>(
        collected: &mut Self::OrderedThreadValues,
        idx: usize,
        values: impl IntoIterator<Item = Result<Typ3, E>>,
    ) -> Result<(), E> {
        let len_begin = collected.values.len();
        for value in values {
            collected.values.push(value?);
        }

        let len = collected.values.len() - len_begin;
        if len > 0 {
            collected.positions.push(IdxLen { idx, len });
        }

        Ok(())
    }

    // add

    fn add_one(&mut self, value: Typ3) {
        self.push(value);
    }

    // extend - merge

    fn extend_merge_infallibles(&mut self, results: Vec<Self::ThreadValues>) {
        let collected_len: usize = results.iter().map(|x| x.len()).sum();
        self.reserve(collected_len);
        for result in results {
            self.extend(result);
        }
    }

    fn extend_merge_ordered_infallibles(&mut self, mut results: Vec<Self::OrderedThreadValues>) {
        let collected_len: usize = results.iter().map(|x| x.values.len()).sum();
        self.reserve(collected_len);
        let initial_len = self.len();
        let total_len = initial_len + collected_len;

        let mut queue = BinaryHeap::with_capacity(results.len());
        let mut pos_indices = vec![0; results.len()];

        for (t, vec) in results.iter().enumerate() {
            if let Some(pos) = vec.positions.first() {
                let node = ThBegLen::new(t, 0, pos.len);
                queue.push(node, pos.idx);
            }
        }
        let mut curr_t = queue.pop_node();
        let mut ptr_dst = unsafe { self.as_mut_ptr().add(initial_len) };

        while let Some(ThBegLen { th, beg, len }) = curr_t {
            let ptr_src = unsafe { results[th].values.as_ptr().add(beg) };
            unsafe { ptr_dst.copy_from_nonoverlapping(ptr_src, len) };

            pos_indices[th] += 1;
            curr_t = match results[th].positions.get(pos_indices[th]) {
                Some(pos) => {
                    let beg = beg + len;
                    let node = ThBegLen::new(th, beg, pos.len);
                    Some(queue.push_then_pop(node, pos.idx).0)
                }
                None => queue.pop_node(),
            };

            ptr_dst = unsafe { ptr_dst.add(len) };
        }

        for vec in results.iter_mut() {
            // SAFETY: this prevents to drop the elements which are already moved to pinned_vec
            // allocation within vec.capacity() will still be reclaimed; however, as uninitialized memory
            unsafe { vec.values.set_len(0) };
        }

        unsafe { self.set_len(total_len) };
    }
}
