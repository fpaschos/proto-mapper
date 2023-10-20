use crate::types::TypeScanner;
use std::default::Default;
use syn::{parse_quote, Path};

#[test]
fn test_fold_type() {
    let mut scanner = TypeScanner::default();

    let fragment: Path = parse_quote! {
      HashMap<u8,u8>
    };

    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "HashMap<u8,u8>".to_string());

    let fragment: Path = parse_quote! {
      std::string::Option<u8>
    };

    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "Option<u8>".to_string());

    let fragment: Path = parse_quote! {
      std::string::Option<Vec<u8>>
    };

    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "Option<Vec<u8>>".to_string());

    let fragment: Path = parse_quote! {
      std::collections::HashMap<u8,Vec<u8>>
    };

    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "HashMap<u8,Vec<u8>>".to_string());

    let fragment: Path = parse_quote! {
      Triple<u8,u8, Option<Vec<u8>>>
    };

    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "Triple<u8,u8,Option<Vec<u8>>>".to_string());
}

// TODO implement support for references and tuples for these tests to run

// #[test]
// fn test_weird_types() {
//     let fragment: Path = parse_quote! {
//       &u8
//     };
//
//     let mut scanner = TypeScanner::default();
//     let res = scanner.scan(fragment);
//     assert_eq!(res.to_string(), "&u8".to_string());
//
//     let fragment: Path = parse_quote! {
//       Mutex<u8,u8, Option<Vec<(u8,u8)>>>
//     };
//
//     let res = scanner.scan(fragment);
//     assert_eq!(
//         res.to_string(),
//         "Mutex<u8,u8,Option<Vec<(u8,u8)>>>".to_string()
//     );
// }
