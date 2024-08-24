// Simple Library to generate a proto message
// Copyright (c) 2024 Radiant Science Inc.

// NOTE: this is very particular about the types
// use prost_types::{Timestamp, Duration as ProstDuration};
use duration_proto::google::protobuf::Duration as ProstDuration;
use timestamp_proto::google::protobuf::Timestamp;

// use complex::proto::message::Message;
// use internal::samples::proto::message::ComplexMessage;
use cmessage::sample::proto::complex::ComplexMessage;
// use message::proto::complex::ComplexMessage;
use std::time::{SystemTime, UNIX_EPOCH};
use types::proto::complex::{MessageStatus, MessageType};

use hello_lib::{get_proto, get_time};

pub fn get_proto_complex() -> ComplexMessage {
    let msg = get_proto();
    let time = get_time();

    ComplexMessage {
        sample: Some(msg),
        // type: MessageType::TYPE_A,
        status: MessageStatus::STATUS_ACTIVE,
        created: Some(time),
    }
}

fn main() -> anyhow::Result<()> {
    // create proto msg
    let proto = get_proto_complex();
    // print the message
    println!("{:?}", proto);
    println!("Hello, complex world!");
    Ok(())
}
