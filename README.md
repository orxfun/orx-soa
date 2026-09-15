# orx-soa

[![orx-soa crate](https://img.shields.io/crates/v/orx-soa.svg)](https://crates.io/crates/orx-soa)
[![orx-soa crate](https://img.shields.io/crates/d/orx-soa.svg)](https://crates.io/crates/orx-soa)
[![orx-soa documentation](https://docs.rs/orx-soa/badge.svg)](https://docs.rs/orx-soa)

Struct-of-arrays collections for tuples and named structs.

## Tuple SOA

Use `Soa2` to store the components of a tuple in separate arrays:

```rust
use orx_soa::soa2::{Soa2, ElemRef2, ElemMut2};

let mut soa = Soa2::<u32, char>::new();
soa.push((1, 'a'));
soa.push((2, 'b'));

assert_eq!(soa.as_slice1(), &[1, 2]);
assert_eq!(soa.as_slice2(), &['a', 'b']);

assert_eq!(soa.get(0), Some(ElemRef2::new(&1, &'a')));
assert_eq!(soa.get_mut(1), Some(ElemMut2::new(&mut 2, &mut 'b')));

let aos: Vec<(u32, char)> = soa.clone().into_iter().collect();
assert_eq!(aos, vec![(1, 'a'), (2, 'b')]);

let (numbers, chars): (Vec<u32>, Vec<char>) = soa.into_inner();
assert_eq!(numbers, vec![1, 2]);
assert_eq!(chars, vec!['a', 'b']);
```

## Named SOA

Use `derive(Soa)` to generate a named SOA for a struct:

```rust
use orx_soa::Soa;

#[derive(Soa, Debug, PartialEq)]
pub struct Record {
    id: u32,
    ch: char,
}

let mut soa = RecordSoa::new();
soa.push(Record { id: 1, ch: 'a' });
soa.push(Record { id: 2, ch: 'b' });

assert_eq!(soa.id(), &[1, 2]);
assert_eq!(soa.ch(), &['a', 'b']);

assert_eq!(soa.get(0), Some(RecordRef { id: &1, ch: &'a' }));
assert_eq!(soa.get_mut(1), Some(RecordMut { id: &mut 2, ch: &mut 'b' }));

let aos: Vec<Record> = soa.clone().into_iter().collect();
assert_eq!(aos, vec![Record { id: 1, ch: 'a' }, Record { id: 2, ch: 'b' }]);

let (ids, chars): (Vec<u32>, Vec<char>) = soa.into_inner();
assert_eq!(ids, vec![1, 2]);
assert_eq!(chars, vec!['a', 'b']);
```

## Contributing

Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-soa/issues/new) or create a PR.

## License

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).
