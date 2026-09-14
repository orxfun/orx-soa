mod soa2;
mod soa2_no_send;
mod soa3;

pub use orx_soa_derive::NamedSoa;

// #[cfg(test)]
// mod tests {
//     use orx_soa_derive::NamedSoa;

//     #[derive(NamedSoa)]
//     pub struct Typ2 {
//         id: u32,
//         c: char,
//     }

//     #[derive(NamedSoa)]
//     pub struct Typ3 {
//         id: u32,
//         c: char,
//         name: String,
//     }

//     #[derive(NamedSoa)]
//     pub struct NoSendTyp2 {
//         id: u32,
//         c: *mut char,
//     }

//     #[test]
//     fn derive_soa_generates_expected_struct_vectors() {
//         let mut v = Typ2Vec::new();
//         v.push(Typ2 { id: 1, c: 'a' });
//         assert_eq!(v.len(), 1);
//         assert_eq!(v.id()[0], 1);
//         assert_eq!(v.c()[0], 'a');

//         let mut v3 = Typ3Vec::new();
//         v3.push(Typ3 {
//             id: 2,
//             c: 'b',
//             name: String::from("x"),
//         });
//         assert_eq!(v3.len(), 1);
//         assert_eq!(v3.name()[0], "x");

//         let mut vns = NoSendTyp2Vec::new();
//         let c = std::ptr::null_mut::<char>();
//         vns.push(NoSendTyp2 { id: 3, c });
//         assert_eq!(vns.id()[0], 3);
//         assert_eq!(vns.c()[0], c);
//     }
// }

#[derive(NamedSoa)]
pub struct Record {
    id: u32,
    ch: char,
}
