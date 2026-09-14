use orx_parallel::*;
use orx_soa::*;

#[derive(NamedSoa, PartialEq, Debug)]
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
    let mut vec = RecordVec::new();
    vec.push(Record::new(0, 'x'));
    vec.extend([Record::new(1, 'y'), Record::new(2, 'z')]);

    assert_eq!(vec.len(), 3);
    assert!(!vec.is_empty());

    vec.id_mut()[0] = 10;
    vec.ch_mut()[2] = '!';

    assert_eq!(vec.id(), &[10, 1, 2]);
    assert_eq!(vec.ch(), &['x', 'y', '!']);

    let (ids, chars) = vec.into_inner();
    assert_eq!(ids, vec![10, 1, 2]);
    assert_eq!(chars, vec!['x', 'y', '!']);
}

#[test]
fn named_soa_into_iter() {
    let mut vec = RecordVec::new();
    vec.push(Record::new(0, 'x'));
    vec.extend([Record::new(1, 'y'), Record::new(2, 'z')]);

    let aos: Vec<Record> = vec.into_iter().collect();
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

    let collected: RecordVec = par.collect();

    let (ids, chars) = collected.into_inner();

    assert_eq!(ids, vec![0, 1, 2, 3, 4]);
    assert_eq!(chars, vec!['e', 'o', 'e', 'o', 'e']);
}
