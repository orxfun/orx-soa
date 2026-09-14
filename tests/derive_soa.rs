use orx_parallel::*;
use orx_soa::*;

#[derive(Soa, PartialEq, Debug)]
pub struct Record {
    id: u32,
    ch: char,
}

impl Record {
    fn new(id: u32, ch: char) -> Self {
        Self { id, ch }
    }
}

#[test]
fn named_soa_vec() {
    let mut soa = RecordSoa::new();
    soa.push(Record::new(0, 'x'));
    soa.extend([Record::new(1, 'y'), Record::new(2, 'z')]);

    let el1: RecordRef<'_> = soa.get(1).unwrap();
    assert_eq!(el1.id, &1);
    assert_eq!(el1.ch, &'y');

    let el2: RecordMut<'_> = soa.get_mut(2).unwrap();
    *el2.id = 20;
    *el2.ch = '?';

    assert_eq!(soa.get(4), None);
    assert_eq!(soa.get_mut(5), None);

    assert_eq!(soa.len(), 3);
    assert!(!soa.is_empty());

    soa.id_mut()[0] = 10;
    soa.ch_mut()[2] = '!';

    assert_eq!(soa.id(), &[10, 1, 20]);
    assert_eq!(soa.ch(), &['x', 'y', '!']);

    let (ids, chars) = soa.into_inner();
    assert_eq!(ids, vec![10, 1, 20]);
    assert_eq!(chars, vec!['x', 'y', '!']);
}

#[test]
fn named_soa_into_iter() {
    let mut soa = RecordSoa::new();
    soa.push(Record::new(0, 'x'));
    soa.extend([Record::new(1, 'y'), Record::new(2, 'z')]);

    let aos: Vec<Record> = soa.into_iter().collect();
    assert_eq!(
        aos,
        vec![
            Record::new(0, 'x'),
            Record::new(1, 'y'),
            Record::new(2, 'z')
        ]
    )
}

#[test]
fn named_soa_par_extend() {
    let par = (0..5).par().map(|i| match i.is_multiple_of(2) {
        true => Record::new(i as u32, 'e'),
        false => Record::new(i as u32, 'o'),
    });

    let soa: RecordSoa = par.collect();

    let (ids, chars) = soa.into_inner();

    assert_eq!(ids, vec![0, 1, 2, 3, 4]);
    assert_eq!(chars, vec!['e', 'o', 'e', 'o', 'e']);
}
