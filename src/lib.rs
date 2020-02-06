use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;

use chrono::NaiveTime;
use itertools::Itertools;
use rumqtt::MqttOptions;

#[derive(Debug)]
pub struct AWSIoTConfig {
    pub mqtt_host: String,
    pub client_id: String,
    pub client_cert_path: Option<PathBuf>,
    pub client_key_path: Option<PathBuf>,
    pub ca_path: Option<PathBuf>,
}

/// AWS IoTを使うためのmqtt optionを作る
/// 
pub fn create_mqtt_options(config: AWSIoTConfig) -> Result<MqttOptions, std::io::Error> {
    let client_cert = read_certs_file(config.client_cert_path.unwrap())?;
    let client_key = read_certs_file(config.client_key_path.unwrap())?;
    let ca = read_certs_file(config.ca_path.unwrap())?;

    let mqtt_options = MqttOptions::new(config.client_id, config.mqtt_host, 8883)
        .set_ca(ca)
        .set_client_auth(client_cert, client_key)
        .set_keep_alive(120);
    Ok(mqtt_options)
}

/// certsファイルを読む
/// 
pub fn read_certs_file(file_path: PathBuf) -> Result<Vec<u8>, std::io::Error> {
    let mut cert: Vec<u8> = Vec::new();
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_end(&mut cert)?;
    Ok(cert)
}

/// messageをpublishするか
///
/// ontimesの中であればpublishする
///
/// # Examples
/// ```
/// use chrono::{NaiveTime};
/// use pishirase::is_publish_message;
///
/// let start_time = NaiveTime::from_hms(10, 00, 00);
/// let end_time = NaiveTime::from_hms(10, 10, 00);
/// let ontimes = vec![(start_time, end_time)];
/// let now = NaiveTime::from_hms(10, 05, 00);
/// let publish_true: bool = is_publish_message(now, ontimes);
/// assert!(publish_true);
/// ```
pub fn is_publish_message(now: NaiveTime, ontimes: Vec<(NaiveTime, NaiveTime)>) -> bool {
    if ontimes.is_empty() {
        return true;
    }
    if ontimes.iter().any(|x| x.0 <= now && now <= x.1) {
        return true;
    }
    false
}

/// timeが直列になっているか（交差していないか）
///
pub fn is_straight_time(ontimes: Vec<(NaiveTime, NaiveTime)>) -> bool {
    // check start_time < end_time
    if !ontimes.iter().all(|x| x.0 < x.1) {
        return false;
    }
    // check start_time < other_start_time < end_time
    let perms = ontimes.into_iter().permutations(2);
    for perm in perms {
        if (perm[0].0 < perm[1].0) && (perm[1].0 < perm[0].1)
            || (perm[0].0 < perm[1].1) && (perm[1].1 < perm[0].1)
        {
            return false;
        }
    }
    true
}
