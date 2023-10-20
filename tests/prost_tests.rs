mod prost;

mod proto;

#[test]
fn test() {
    let e = proto::prost::EntityUuids::default();
    let mut e2 = e.clone();
    // e2.opt_uuid_str = "Foo".into();
    assert_eq!(e, e2);
}