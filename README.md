# Pishirase

## About
* publishing message from Raspberry pi

## Requirements
* Rust 1.41+
* Raspberry pi
* AWS IoT

## Setup

### AWS
Use [AWSIoT](https://aws.amazon.com/iot/) service

* Add thing
* Get certs
* Set action

### Linux Machine

* Compile
```sh
cargo build --release --target arm-unknown-linux-gnueabi
```
* Send binary to raspberry pi

### Raspberry Pi

* Write configure `Settings.toml` and set `/opt/pishirase`
```toml
mqtt_host = "your_mqtt_host"
client_id = "your_client_name"
client_cert_path = "/opt/cert/clientcert.crt"
client_key_path = "/opt/cert/clientkey.pem"
ca_path = "/opt/cert/awsrootca.pem"
topic_name = "mytopic/name"
ontimes = [
    [
        "00:00:00",
        "23:59:59"
    ]
]
timezone = "Asia/Tokyo" # see https://docs.rs/chrono-tz/0.5.1/chrono_tz/enum.Tz.html
```

* Set certs

* Add systemd service
```
[Unit]
Description=call for wakeup

[Service]
ExecStart=/opt/pishirase/pishirase 

[Install]
WantedBy=multi-user.target
```

* Run
```sh
sudo systemctl daemon-reload
sudo systemctl start pishirase.service
sudo systemctl enable pishirase.service
```