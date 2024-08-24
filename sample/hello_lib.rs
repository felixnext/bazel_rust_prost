// Simple Library to generate a proto message
// Copyright (c) 2024 Radiant Science Inc.

// NOTE: this is very particular about the types
// use prost_types::{Timestamp, Duration as ProstDuration};
use duration_proto::google::protobuf::Duration as ProstDuration;
use timestamp_proto::google::protobuf::Timestamp;

use message::proto::simple::{SampleMessage, SubMessage};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> Timestamp {
    // Setting time to the current system time
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    Timestamp {
        seconds: now.as_secs() as i64,
        nanos: now.subsec_nanos() as i32,
    }
}

pub fn get_proto() -> SampleMessage {
    // Setting time to the current system time
    let time = get_time();

    // Setting time to a specific datetime: 2024-08-01T12:00:00Z
    // let specific_time = SystemTime::UNIX_EPOCH
    //     + Duration::new(1728000000, 0); // Timestamp for 2024-08-01T12:00:00Z
    // let specific_time_duration = specific_time
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");
    // let specific_timestamp = Timestamp {
    //     seconds: specific_time_duration.as_secs() as i64,
    //     nanos: specific_time_duration.subsec_nanos() as i32,
    // };

    // Setting duration to 15 minutes and 10 seconds
    let duration = ProstDuration {
        seconds: 15 * 60 + 10, // 15 minutes and 10 seconds in total seconds
        nanos: 0,
    };

    SampleMessage {
        name: "Rust".to_string(),
        time: Some(time),
        duration: Some(duration),
        tags: vec!["rust".to_string(), "proto".to_string()],
        subs: vec![SubMessage {
            flag: true,
            value: 3.4f32,
        }],
        meta: Some(SubMessage {
            flag: false,
            value: 0.0f32,
        }),
    }
}
