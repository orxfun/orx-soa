use orx_parallel::collectables::{ColAndPos, IdxLen, ParExtendCore, ThBegLen};
use orx_priority_queue::{BinaryHeap, PriorityQueue};
use std::vec;

pub struct Typ2 {
    id: u32,
    c: char,
}

#[derive(Clone, Copy)]
pub struct Typ2Ptr {
    pub id: *const u32,
    pub c: *const char,
}

impl Typ2Ptr {
    pub unsafe fn add(self, count: usize) -> Self {
        Self {
            id: unsafe { self.id.add(count) },
            c: unsafe { self.c.add(count) },
        }
    }
}

#[derive(Clone, Copy)]
pub struct Typ2MutPtr {
    pub id: *mut u32,
    pub c: *mut char,
}

impl Typ2MutPtr {
    pub unsafe fn add(self, count: usize) -> Self {
        Self {
            id: unsafe { self.id.add(count) },
            c: unsafe { self.c.add(count) },
        }
    }

    pub unsafe fn copy_from_nonoverlapping(self, src: Typ2Ptr, count: usize) {
        unsafe { self.id.copy_from_nonoverlapping(src.id, count) };
        unsafe { self.c.copy_from_nonoverlapping(src.c, count) };
    }
}

pub struct Typ2Vec {
    id: Vec<u32>,
    c: Vec<char>,
}

impl Typ2Vec {
    // ctor & dtor

    pub fn new() -> Self {
        Self {
            id: Vec::new(),
            c: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            id: Vec::with_capacity(capacity),
            c: Vec::with_capacity(capacity),
        }
    }

    pub fn into_inner(self) -> (Vec<u32>, Vec<char>) {
        (self.id, self.c)
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

    pub fn as_ptr(&self) -> Typ2Ptr {
        Typ2Ptr {
            id: self.id.as_ptr(),
            c: self.c.as_ptr(),
        }
    }

    // mut

    pub fn push(&mut self, item: Typ2) {
        self.id.push(item.id);
        self.c.push(item.c);
    }

    pub fn as_mut_ptr(&mut self) -> Typ2MutPtr {
        Typ2MutPtr {
            id: self.id.as_mut_ptr(),
            c: self.c.as_mut_ptr(),
        }
    }

    pub fn id_mut(&mut self) -> &mut [u32] {
        &mut self.id
    }

    pub fn c_mut(&mut self) -> &mut [char] {
        &mut self.c
    }

    pub fn reserve(&mut self, additional: usize) {
        self.id.reserve(additional);
        self.c.reserve(additional);
    }

    pub unsafe fn set_len(&mut self, new_len: usize) {
        unsafe { self.id.set_len(new_len) };
        unsafe { self.c.set_len(new_len) };
    }
}

impl Default for Typ2Vec {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Typ2Vec> for (Vec<u32>, Vec<char>) {
    fn from(value: Typ2Vec) -> Self {
        (value.id, value.c)
    }
}

impl Extend<Typ2> for Typ2Vec {
    fn extend<I: IntoIterator<Item = Typ2>>(&mut self, iter: I) {
        for x in iter {
            self.id.push(x.id);
            self.c.push(x.c);
        }
    }
}

// iterators

pub struct Typ2Iter {
    id: vec::IntoIter<u32>,
    c: vec::IntoIter<char>,
}

impl Iterator for Typ2Iter {
    type Item = Typ2;

    fn next(&mut self) -> Option<Self::Item> {
        self.id.next().map(|id| {
            // SAFETY: `id` and `c` has exactly same length
            let c = unsafe { self.c.next().unwrap_unchecked() };
            Typ2 { id, c }
        })
    }
}

impl IntoIterator for Typ2Vec {
    type Item = Typ2;

    type IntoIter = Typ2Iter;

    fn into_iter(self) -> Self::IntoIter {
        Typ2Iter {
            id: self.id.into_iter(),
            c: self.c.into_iter(),
        }
    }
}

pub struct Typ2Ref<'a> {
    pub id: &'a u32,
    pub c: &'a char,
}

pub struct Typ2IterRef<'a> {
    id: core::slice::Iter<'a, u32>,
    c: core::slice::Iter<'a, char>,
}

impl<'a> Iterator for Typ2IterRef<'a> {
    type Item = Typ2Ref<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.id.next().map(|id| {
            // SAFETY: `id` and `c` has exactly same length
            let c = unsafe { self.c.next().unwrap_unchecked() };
            Typ2Ref { id, c }
        })
    }
}

impl<'a> IntoIterator for &'a Typ2Vec {
    type Item = Typ2Ref<'a>;

    type IntoIter = Typ2IterRef<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Typ2IterRef {
            id: self.id.iter(),
            c: self.c.iter(),
        }
    }
}

pub struct Typ2Mut<'a> {
    pub id: &'a mut u32,
    pub c: &'a mut char,
}

pub struct Typ2IterMut<'a> {
    id: core::slice::IterMut<'a, u32>,
    c: core::slice::IterMut<'a, char>,
}

impl<'a> Iterator for Typ2IterMut<'a> {
    type Item = Typ2Mut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.id.next().map(|id| {
            // SAFETY: `id` and `c` has exactly same length
            let c = unsafe { self.c.next().unwrap_unchecked() };
            Typ2Mut { id, c }
        })
    }
}

impl<'a> IntoIterator for &'a mut Typ2Vec {
    type Item = Typ2Mut<'a>;

    type IntoIter = Typ2IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Typ2IterMut {
            id: self.id.iter_mut(),
            c: self.c.iter_mut(),
        }
    }
}

// parallel

impl ParExtendCore<Typ2> for Typ2Vec {
    type ThreadValues = Self;

    type OrderedThreadValues = ColAndPos<Self>;

    fn new_thread_values() -> Self::ThreadValues {
        Default::default()
    }

    fn new_ordered_thread_values() -> Self::OrderedThreadValues {
        Default::default()
    }

    // thread collect

    fn add_thread_value(collected: &mut Self::ThreadValues, value: Typ2) {
        collected.push(value);
    }

    fn add_thread_values(
        collected: &mut Self::ThreadValues,
        values: impl IntoIterator<Item = Typ2>,
    ) {
        collected.extend(values);
    }

    fn add_ordered_thread_value(
        collected: &mut Self::OrderedThreadValues,
        idx: usize,
        value: Typ2,
    ) {
        collected.values.push(value);
        collected.positions.push(IdxLen { idx, len: 1 });
    }

    fn add_ordered_thread_values(
        collected: &mut Self::OrderedThreadValues,
        idx: usize,
        values: impl IntoIterator<Item = Typ2>,
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
        values: impl IntoIterator<Item = Option<Typ2>>,
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
        values: impl IntoIterator<Item = Result<Typ2, E>>,
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

    fn add_one(&mut self, value: Typ2) {
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
