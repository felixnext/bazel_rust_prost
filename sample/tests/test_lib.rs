// Basic Unit Tests for the Rust Library
// Copyright (c) 2024 Radiant Science Inc.

extern crate hello_lib;

use hello_lib::get_proto;

#[test]
fn demo_test() {
    assert_eq!(4 + 4, 8);
}

#[test]
fn test_get_proto() {
    let proto = get_proto();
    assert_eq!(proto.name, "Rust");
    assert_eq!(proto.tags, vec!["rust".to_string(), "proto".to_string()]);
    assert_eq!(proto.subs.len(), 1);
    assert_eq!(proto.subs[0].flag, true);
    assert_eq!(proto.subs[0].value, 3.4);
    assert_eq!(proto.meta.as_ref().unwrap().flag, false);
    assert_eq!(proto.meta.as_ref().unwrap().value, 0.0);
}
