use chrono::NaiveTime;
use pishirase::{is_publish_message, is_straight_time, read_certs_file};
use std::path::PathBuf;

#[test]
fn test_create_mqtt_options() {}

#[test]
fn test_read_certs_file() {
    let text = read_certs_file(PathBuf::from("./tests/test_certs/test.pub.pem.txt")).unwrap();

    assert_eq!(
        text,
        "-----BEGIN PUBLIC KEY-----
TESTDUMMYPUBLICKEY
-----END PUBLIC KEY-----"
            .as_bytes()
    )
}

#[test]
fn test_is_publish_message_none() {
    assert!(is_publish_message(
        NaiveTime::from_hms(10, 10, 00),
        &Vec::new()
    ));
}

#[test]
fn test_is_publish_message_true() {
    assert!(is_publish_message(
        NaiveTime::from_hms(10, 00, 00),
        &vec![(
            NaiveTime::from_hms(09, 00, 00),
            NaiveTime::from_hms(10, 00, 00)
        )]
    ))
}

#[test]
fn test_is_publish_message_false() {
    assert_eq!(
        is_publish_message(
            NaiveTime::from_hms(10, 00, 01),
            &vec![(
                NaiveTime::from_hms(09, 00, 00),
                NaiveTime::from_hms(10, 00, 00)
            )]
        ),
        false
    )
}

#[test]
fn test_is_publish_message_true_multitimes() {
    assert!(is_publish_message(
        NaiveTime::from_hms(10, 00, 00),
        &vec![
            (
                NaiveTime::from_hms(00, 00, 00),
                NaiveTime::from_hms(01, 00, 00)
            ),
            (
                NaiveTime::from_hms(10, 00, 00),
                NaiveTime::from_hms(11, 00, 00)
            )
        ]
    ))
}

#[test]
fn test_is_publish_message_false_multitimes() {
    assert_eq!(
        is_publish_message(
            NaiveTime::from_hms(10, 00, 00),
            &vec![
                (
                    NaiveTime::from_hms(00, 00, 00),
                    NaiveTime::from_hms(00, 59, 00)
                ),
                (
                    NaiveTime::from_hms(01, 00, 00),
                    NaiveTime::from_hms(01, 59, 00)
                )
            ]
        ),
        false
    )
}

#[test]
fn test_is_straight_time() {
    let ontimes = vec![(
        NaiveTime::from_hms(10, 00, 00),
        NaiveTime::from_hms(11, 00, 00),
    )];
    assert!(is_straight_time(&ontimes));
}

#[test]
fn test_is_straight_time_false() {
    let ontimes = vec![(
        NaiveTime::from_hms(10, 00, 00),
        NaiveTime::from_hms(09, 00, 00),
    )];
    assert!(!is_straight_time(&ontimes));
}

#[test]
fn test_is_straight_time_multiple() {
    let ontimes = vec![
        (
            NaiveTime::from_hms(10, 00, 00),
            NaiveTime::from_hms(11, 00, 00),
        ),
        (
            NaiveTime::from_hms(12, 00, 00),
            NaiveTime::from_hms(13, 00, 00),
        ),
    ];
    assert!(is_straight_time(&ontimes));
}

#[test]
fn test_is_straight_time_multiple_false() {
    let ontimes = vec![
        (
            NaiveTime::from_hms(10, 00, 00),
            NaiveTime::from_hms(11, 00, 00),
        ),
        (
            NaiveTime::from_hms(10, 30, 00),
            NaiveTime::from_hms(11, 30, 00),
        ),
    ];
    assert!(!is_straight_time(&ontimes));
}
