use alloc::{vec, vec::Vec};
use orx_parallel::extendable::{ColAndPos, IdxLen, ParExtendCore, ThBegLen};
use orx_priority_queue::{BinaryHeap, PriorityQueue};

/// Struct-of-arrays storage for pairs of values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Soa2<T1, T2> {
    v1: Vec<T1>,
    v2: Vec<T2>,
}

/// A pair of immutable pointers to the component arrays.
pub struct Ptr2<T1, T2> {
    p1: *const T1,
    p2: *const T2,
}

impl<T1, T2> Clone for Ptr2<T1, T2> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T1, T2> Copy for Ptr2<T1, T2> {}

impl<T1, T2> Ptr2<T1, T2> {
    /// Advances both pointers by `count` elements.
    ///
    /// # Safety
    ///
    /// The resulting pointers must remain within, or one past, their respective
    /// allocated arrays.
    pub unsafe fn add(self, count: usize) -> Self {
        Self {
            p1: unsafe { self.p1.add(count) },
            p2: unsafe { self.p2.add(count) },
        }
    }
}

/// A pair of mutable pointers to the component arrays.
pub struct PtrMut2<T1, T2> {
    p1: *mut T1,
    p2: *mut T2,
}

impl<T1, T2> Clone for PtrMut2<T1, T2> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T1, T2> Copy for PtrMut2<T1, T2> {}

impl<T1, T2> PtrMut2<T1, T2> {
    /// Advances both pointers by `count` elements.
    ///
    /// # Safety
    ///
    /// The resulting pointers must remain within, or one past, their respective
    /// allocated arrays.
    pub unsafe fn add(self, count: usize) -> Self {
        Self {
            p1: unsafe { self.p1.add(count) },
            p2: unsafe { self.p2.add(count) },
        }
    }

    /// Copies `count` pairs from `src` into these destination pointers.
    ///
    /// # Safety
    ///
    /// Both regions must be valid for reads and writes of `count` elements and
    /// must not overlap.
    pub unsafe fn copy_from_nonoverlapping(self, src: Ptr2<T1, T2>, count: usize) {
        unsafe { self.p1.copy_from_nonoverlapping(src.p1, count) };
        unsafe { self.p2.copy_from_nonoverlapping(src.p2, count) };
    }
}

impl<T1, T2> Default for Soa2<T1, T2> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T1, T2> Soa2<T1, T2> {
    /// Creates an empty collection.
    pub fn new() -> Self {
        Self {
            v1: Default::default(),
            v2: Default::default(),
        }
    }

    /// Creates an empty collection with space for at least `capacity` pairs.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            v1: Vec::with_capacity(capacity),
            v2: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of stored pairs.
    pub fn len(&self) -> usize {
        self.v1.len()
    }

    /// Returns whether the collection contains no pairs.
    pub fn is_empty(&self) -> bool {
        self.v1.is_empty()
    }

    /// Decomposes the collection into its component vectors.
    pub fn into_inner(self) -> (Vec<T1>, Vec<T2>) {
        (self.v1, self.v2)
    }

    /// Returns the first component slice.
    pub fn as_slice1(&self) -> &[T1] {
        &self.v1
    }

    /// Returns the second component slice.
    pub fn as_slice2(&self) -> &[T2] {
        &self.v2
    }

    /// Returns a mutable slice of the first component.
    pub fn as_mut_slice1(&mut self) -> &mut [T1] {
        &mut self.v1
    }

    /// Returns a mutable slice of the second component.
    pub fn as_mut_slice2(&mut self) -> &mut [T2] {
        &mut self.v2
    }

    /// Returns immutable pointers to the component arrays.
    pub fn as_ptr(&self) -> Ptr2<T1, T2> {
        Ptr2 {
            p1: self.v1.as_ptr(),
            p2: self.v2.as_ptr(),
        }
    }

    /// Returns the pair at `index` as borrowed component references.
    pub fn get(&self, index: usize) -> Option<ElemRef2<'_, T1, T2>> {
        self.v1.get(index).map(|v1| {
            let v2 = &self.v2[index];
            ElemRef2 { v1, v2 }
        })
    }

    /// Returns the pair at `index` as mutable component references.
    pub fn get_mut(&mut self, index: usize) -> Option<ElemMut2<'_, T1, T2>> {
        self.v1.get_mut(index).map(|v1| {
            let v2 = &mut self.v2[index];
            ElemMut2 { v1, v2 }
        })
    }

    /// Appends a pair to the collection.
    pub fn push(&mut self, (v1, v2): (T1, T2)) {
        self.v1.push(v1);
        self.v2.push(v2);
    }

    /// Returns mutable pointers to the component arrays.
    pub fn as_mut_ptr(&mut self) -> PtrMut2<T1, T2> {
        PtrMut2 {
            p1: self.v1.as_mut_ptr(),
            p2: self.v2.as_mut_ptr(),
        }
    }

    /// Reserves capacity for at least `additional` more pairs.
    pub fn reserve(&mut self, additional: usize) {
        self.v1.reserve(additional);
        self.v2.reserve(additional);
    }

    /// Sets the number of stored pairs without initializing or dropping values.
    ///
    /// # Safety
    ///
    /// The new length must not exceed the capacity, and all newly exposed
    /// elements must be initialized in both component arrays.
    pub unsafe fn set_len(&mut self, new_len: usize) {
        unsafe { self.v1.set_len(new_len) };
        unsafe { self.v2.set_len(new_len) };
    }

    pub fn sort_by1(&mut self)
    where
        T1: Ord,
    {
        self.sort_by(|a, b| a.v1.cmp(b.v1));
    }

    pub fn sort_by2(&mut self)
    where
        T2: Ord,
    {
        self.sort_by(|a, b| a.v2.cmp(b.v2));
    }

    fn sort_by<F>(&mut self, mut compare: F)
    where
        F: FnMut(ElemRef2<'_, T1, T2>, ElemRef2<'_, T1, T2>) -> core::cmp::Ordering,
    {
        let len = self.len();
        if len <= 1 {
            return;
        }

        let mut indices: Vec<usize> = (0..len).collect();
        indices.sort_by(|&left, &right| {
            compare(
                ElemRef2 {
                    v1: &self.v1[left],
                    v2: &self.v2[left],
                },
                ElemRef2 {
                    v1: &self.v1[right],
                    v2: &self.v2[right],
                },
            )
        });

        let mut positions = vec![0; len];
        for (new_position, old_position) in indices.into_iter().enumerate() {
            positions[old_position] = new_position;
        }

        for index in 0..len {
            while positions[index] != index {
                let other = positions[index];
                self.v1.swap(index, other);
                self.v2.swap(index, other);
                positions.swap(index, other);
            }
        }
    }
}

impl<T1, T2> From<Soa2<T1, T2>> for (Vec<T1>, Vec<T2>) {
    fn from(value: Soa2<T1, T2>) -> Self {
        (value.v1, value.v2)
    }
}

impl<T1, T2> Extend<(T1, T2)> for Soa2<T1, T2> {
    fn extend<I: IntoIterator<Item = (T1, T2)>>(&mut self, iter: I) {
        for x in iter {
            self.push(x);
        }
    }
}

/// Owning iterator over the pairs in a `Soa2` collection.
pub struct Soa2Iter<T1, T2> {
    i1: vec::IntoIter<T1>,
    i2: vec::IntoIter<T2>,
}

impl<T1, T2> Iterator for Soa2Iter<T1, T2> {
    type Item = (T1, T2);

    fn next(&mut self) -> Option<Self::Item> {
        self.i1.next().map(|v1| {
            let v2 = unsafe { self.i2.next().unwrap_unchecked() };
            (v1, v2)
        })
    }
}

impl<T1, T2> IntoIterator for Soa2<T1, T2> {
    type Item = (T1, T2);

    type IntoIter = Soa2Iter<T1, T2>;

    fn into_iter(self) -> Self::IntoIter {
        Soa2Iter {
            i1: self.v1.into_iter(),
            i2: self.v2.into_iter(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, derive_new::new)]
/// Borrowed component references returned by SOA accessors.
pub struct ElemRef2<'a, T1, T2> {
    /// Reference to the first component.
    pub v1: &'a T1,
    /// Reference to the second component.
    pub v2: &'a T2,
}

/// Iterator over borrowed component references.
pub struct Soa2IterRef<'a, T1, T2> {
    i1: core::slice::Iter<'a, T1>,
    i2: core::slice::Iter<'a, T2>,
}

impl<'a, T1, T2> Iterator for Soa2IterRef<'a, T1, T2> {
    type Item = ElemRef2<'a, T1, T2>;

    fn next(&mut self) -> Option<Self::Item> {
        self.i1.next().map(|v1| {
            let v2 = unsafe { self.i2.next().unwrap_unchecked() };
            ElemRef2 { v1, v2 }
        })
    }
}

#[derive(PartialEq, Eq, Debug, derive_new::new)]
/// Mutable component references returned by SOA accessors.
pub struct ElemMut2<'a, T1, T2> {
    /// Mutable reference to the first component.
    pub v1: &'a mut T1,
    /// Mutable reference to the second component.
    pub v2: &'a mut T2,
}

/// Mutable iterator over the component arrays.
pub struct Soa2IterMut<'a, T1, T2> {
    i1: core::slice::IterMut<'a, T1>,
    i2: core::slice::IterMut<'a, T2>,
}

impl<'a, T1, T2> Iterator for Soa2IterMut<'a, T1, T2> {
    type Item = ElemMut2<'a, T1, T2>;

    fn next(&mut self) -> Option<Self::Item> {
        self.i1.next().map(|v1| {
            let v2 = unsafe { self.i2.next().unwrap_unchecked() };
            ElemMut2 { v1, v2 }
        })
    }
}

// parallel

impl<T1: Send, T2: Send> ParExtendCore<(T1, T2)> for Soa2<T1, T2> {
    type ThreadValues = Self;

    type OrderedThreadValues = ColAndPos<Self>;

    fn new_thread_values() -> Self::ThreadValues {
        Default::default()
    }

    fn new_ordered_thread_values() -> Self::OrderedThreadValues {
        Default::default()
    }

    // thread collect

    fn add_thread_value(collected: &mut Self::ThreadValues, value: (T1, T2)) {
        collected.push(value);
    }

    fn add_thread_values(
        collected: &mut Self::ThreadValues,
        values: impl IntoIterator<Item = (T1, T2)>,
    ) {
        collected.extend(values);
    }

    fn add_ordered_thread_value(
        collected: &mut Self::OrderedThreadValues,
        idx: usize,
        value: (T1, T2),
    ) {
        collected.values.push(value);
        collected.positions.push(IdxLen { idx, len: 1 });
    }

    fn add_ordered_thread_values(
        collected: &mut Self::OrderedThreadValues,
        idx: usize,
        values: impl IntoIterator<Item = (T1, T2)>,
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
        values: impl IntoIterator<Item = Option<(T1, T2)>>,
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
        values: impl IntoIterator<Item = Result<(T1, T2), E>>,
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

    fn add_one(&mut self, value: (T1, T2)) {
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
