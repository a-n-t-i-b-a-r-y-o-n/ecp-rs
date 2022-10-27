use crate::connection::Connection;

use crate::config;
use crate::message::request::Request;
use crate::protocol::command::Set;
use crate::protocol::query::Get;

/// IPv4 for a device on your network
const DEVICE_IP: [u8; 4] = [192, 168, 1, 226];

#[allow(dead_code)]
fn behold() -> Vec<u8> {
    let config = config::load_from_file("conf/secrets");
    assert!(config.len() > 0);
    assert!(config.contains_key("this_one_shows_spirit"));
    let key = config.get("this_one_shows_spirit").unwrap().as_bytes().to_vec();
    key
}

#[tokio::test]
async fn open_ecp_connection() {
    let key = behold();
    let mut connection = Connection::new(
        DEVICE_IP,
        key
    );
    connection.open().await;

    assert!(connection.is_open());
    assert!(connection.is_authenticated());
    assert_eq!(connection.sync_counter, 0);
}

#[tokio::test]
async fn query_device_info_raw() {

    let mut connection = Connection::new(
        DEVICE_IP,
        behold()
    );
    connection.open().await;

    assert!(connection.is_open());
    assert!(connection.is_authenticated());

    let request = Request::new()
        .set_subject("query-device-info")
        .set_request_id(connection.next_sync_number());
    let response = connection.send_request(request).await;
    match response {
        None => {}
        Some(message) => {
            if let Some(data) = message.content_data {
                println!("Response content: {:?}", data)
            }
        }
    }
}

#[tokio::test]
async fn query_device_info() {

    let mut connection = Connection::new(
        DEVICE_IP,
        behold()
    );
    connection.open().await;

    assert!(connection.is_open());
    assert!(connection.is_authenticated());

    let request = Request::new()
        .set_subject(Get::AudioSettings.subject())
        .set_request_id(connection.next_sync_number());
    let response = connection.send_request(request).await;
    assert_ne!(response, None);
    match response {
        None => {}
        Some(message) => {
            if let Some(data) = message.content_data {
                println!("Response content: {:?}", data)
            }
        }
    }
}

#[tokio::test]
async fn query_screensavers() {

    let mut connection = Connection::new(
        DEVICE_IP,
        behold()
    );
    connection.open().await;

    assert!(connection.is_open());
    assert!(connection.is_authenticated());

    let request = Request::new()
        .set_subject(Get::Screensavers.subject())
        .set_request_id(connection.next_sync_number());
    let response = connection.send_request(request).await;
    assert_ne!(response, None);
    match response {
        None => {}
        Some(message) => {
            if let Some(data) = message.content_data {
                println!("Response content: {:?}", data)
            }
        }
    }
}

#[tokio::test]
async fn query_pq_options() {

    let mut connection = Connection::new(
        DEVICE_IP,
        behold()
    );
    connection.open().await;

    assert!(connection.is_open());
    assert!(connection.is_authenticated());

    let pq_options = Request::new()
        .set_subject("query-pq-color-space-settings")
        .set_request_id(connection.next_sync_number());
    let response = connection.send_request(pq_options).await;
    assert_ne!(response, None);
    match response {
        None => {}
        Some(message) => {
            if let Some(data) = message.content_data {
                println!("Response content: \n\n{:?}", data)
            }
        }
    }
}

#[tokio::test]
async fn press_power_button() {
    let mut connection = Connection::new(
        DEVICE_IP,
        behold()
    );
    connection.open().await;

    assert!(connection.is_open());
    assert!(connection.is_authenticated());

    let command = Set::PressKey { key: String::from("Power") };

    let response = connection.send_request(Request::from(command)).await;
    assert_ne!(response, None);
    match response {
        None => {}
        Some(message) => {
            println!("[-] Request success: {}", message.is_success());
        }
    }
}

#[tokio::test]
async fn press_multiple_keys() {
    let mut connection = Connection::new(
        DEVICE_IP,
        behold()
    );
    connection.open().await;

    assert!(connection.is_open());
    assert!(connection.is_authenticated());

    let left = Set::PressKey { key: String::from("Left") };
    let right = Set::PressKey { key: String::from("Right") };
    let select = Set::PressKey { key: String::from("Select") };

    let _ = connection.send_request(left.into()).await;
    let _ = connection.send_request(right.into()).await;
    let _ = connection.send_request(select.into()).await;
}

#[tokio::test]
async fn get_app_icon() {
    let mut connection = Connection::new(
        DEVICE_IP,
        behold()
    );
    connection.open().await;

    assert!(connection.is_open());
    assert!(connection.is_authenticated());

    let request = Get::QueryAppIcon { channel_id: 140704 };
    let response = connection.send_request(request.into()).await;
    assert_ne!(response, None);
    match response {
        None => {}
        Some(response) => {
            println!("Received: {:?}", response.raw_bytes);
        }
    }
}

