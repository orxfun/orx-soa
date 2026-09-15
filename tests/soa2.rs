use orx_parallel::extendable::ParExtendCore;
use orx_soa::soa2::*;

#[test]
fn soa2_push_get_and_into_inner_work() {
    let mut soa = Soa2::<u32, char>::new();
    soa.push((1, 'a'));
    soa.push((2, 'b'));

    assert_eq!(soa.len(), 2);
    assert!(!soa.is_empty());

    let first = soa.get(0).unwrap();
    assert_eq!(*first.v1, 1);
    assert_eq!(*first.v2, 'a');

    let first_mut = soa.get_mut(0).unwrap();
    *first_mut.v1 = 10;
    *first_mut.v2 = 'z';

    let ptr: Ptr2<u32, char> = soa.as_ptr();
    let mut_ptr: PtrMut2<u32, char> = soa.as_mut_ptr();
    let _ = (ptr, mut_ptr);

    let (v1, v2) = soa.into_inner();
    assert_eq!(v1, vec![10, 2]);
    assert_eq!(v2, vec!['z', 'b']);
}

#[test]
fn soa2_iterator_and_mutation_views_work() {
    let mut soa = Soa2::<u32, char>::new();
    soa.push((1, 'a'));
    soa.push((2, 'b'));

    let values: Vec<_> = soa.into_iter().collect();
    assert_eq!(values, vec![(1, 'a'), (2, 'b')]);

    let mut soa = Soa2::<u32, char>::new();
    soa.push((1, 'a'));
    soa.push((2, 'b'));

    let first = soa.get_mut(0).unwrap();
    *first.v1 += 100;
    *first.v2 = 'x';

    let second = soa.get_mut(1).unwrap();
    *second.v1 += 100;
    *second.v2 = 'x';

    assert_eq!(
        soa.into_iter().collect::<Vec<_>>(),
        vec![(101, 'x'), (102, 'x')]
    );
}

#[test]
fn soa2_parallel_extend_core_matches_generic_pattern() {
    let mut left = Soa2::<u32, char>::new();
    left.push((1, 'a'));
    left.push((2, 'b'));

    let mut right = Soa2::<u32, char>::new();
    right.push((3, 'c'));

    let mut merged = Soa2::<u32, char>::new();
    merged.extend_merge_infallibles(vec![left, right]);
    assert_eq!(
        merged.into_iter().collect::<Vec<_>>(),
        vec![(1, 'a'), (2, 'b'), (3, 'c')]
    );

    let mut thread_values = Soa2::<u32, char>::new_thread_values();
    Soa2::<u32, char>::add_thread_value(&mut thread_values, (4, 'd'));
    Soa2::<u32, char>::add_thread_value(&mut thread_values, (5, 'e'));
    assert_eq!(
        thread_values.into_iter().collect::<Vec<_>>(),
        vec![(4, 'd'), (5, 'e')]
    );
}

#[test]
fn soa2_sort() {
    let mut soa = Soa2::new();
    soa.extend([(1, 'a'), (0, 'c'), (3, 'b'), (2, 'd')]);

    soa.sort_by1();
    let mut expected_by1 = Soa2::new();
    expected_by1.extend([(0, 'c'), (1, 'a'), (2, 'd'), (3, 'b')]);
    assert_eq!(soa, expected_by1);

    soa.sort_by2();
    let mut expected_by2 = Soa2::new();
    expected_by2.extend([(1, 'a'), (3, 'b'), (0, 'c'), (2, 'd')]);
    assert_eq!(soa, expected_by2);
}
