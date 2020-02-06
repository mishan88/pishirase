use chrono::{NaiveTime, Utc};
use chrono_tz::Tz;
use config;
use rppal::gpio::Gpio;
use rumqtt::{MqttClient, QoS};
use std::path::PathBuf;
use std::{thread, time::Duration};

use pishirase::*;

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("/opt/pishirase/Settings.toml"))
        .unwrap();

    let mqtt_host = settings.get_str("mqtt_host").unwrap();
    let client_id = settings.get_str("client_id").unwrap();
    let client_cert_path = settings.get_str("client_cert_path").unwrap();
    let client_key_path = settings.get_str("client_key_path").unwrap();
    let ca_path = settings.get_str("ca_path").unwrap();
    let topic_name = settings.get_str("topic_name").unwrap();
    let ontimes = settings.get_array("ontimes").unwrap();
    let timezone: Tz = settings.get_str("timezone").unwrap().parse().unwrap();
    let duration_sec = settings.get_int("duration_sec").unwrap() as u64;

    let mut ontimes_list: Vec<(NaiveTime, NaiveTime)> = Vec::new();

    for ontime in ontimes {
        let start_end_time = ontime.into_array().unwrap();
        let start_time = start_end_time[0].to_owned().into_str().unwrap();
        let end_time = start_end_time[1].to_owned().into_str().unwrap();
        ontimes_list.push((
            NaiveTime::parse_from_str(&start_time, "%H:%M:%S").unwrap(),
            NaiveTime::parse_from_str(&end_time, "%H:%M:%S").unwrap(),
        ))
    }
    if !is_straight_time(&ontimes_list) {
        panic!("Time Config Error!");
    }

    let gpio = Gpio::new().unwrap();
    let pin = gpio.get(23).unwrap().into_input();

    let conf = AWSIoTConfig {
        mqtt_host,
        client_id,
        client_cert_path: Some(PathBuf::from(client_cert_path)),
        client_key_path: Some(PathBuf::from(client_key_path)),
        ca_path: Some(PathBuf::from(ca_path)),
    };
    let mqtt_options = create_mqtt_options(conf);
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options.unwrap()).unwrap();

    mqtt_client
        .subscribe(&topic_name, QoS::AtLeastOnce)
        .unwrap();

    thread::spawn(move || loop {
        if pin.is_high() {
            let nowtime = Utc::now().with_timezone(&timezone).time();
            let payload = settings.get_str("payload").unwrap();

            if is_publish_message(nowtime, &ontimes_list) {
                mqtt_client
                    .publish(&topic_name, QoS::AtLeastOnce, false, payload)
                    .unwrap();
                thread::sleep(Duration::from_secs(duration_sec));
            }
        }
    });

    for notification in notifications {
        println!("{:?}", notification);
    }
}
