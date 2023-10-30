use std::net::IpAddr;
use std::thread::spawn;
use std::time::Duration;
use std::{env, io, process};

use crate::common::generate_websocket_key;
use crate::config::env::EnvConfig;
use crate::recruitment_platform::ReqPlatform;

use futures::SinkExt;
use futures::StreamExt;
use mqtt::{ConnectOptions, ConnectOptionsBuilder, CreateOptions, SslOptionsBuilder};
use paho_mqtt as mqtt;
use reqwest::header::{HeaderMap, COOKIE};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::http::Request;
use tokio_tungstenite::tungstenite::Message;
use tracing::debug;

// 聊天建立链接
// 有BUG
pub async fn connect_chat(req_platform: &ReqPlatform, config: &EnvConfig, headers: HeaderMap) {
    const QOS: i32 = 1;
    const NO_LOCAL: bool = true;
    match req_platform {
        ReqPlatform::Zp => {
            let url = format!("wss://ws.zhipin.com/chatws");
            let chat_user = format!(" ").to_string();
            let chat_group = "group".to_string();
            let chat_topic = format!("chat");
            let client_id = format!("ws-{}", e_utils::random!(nanoid 16));

            debug!("尝试建立WebSocket链接{}", url);
            let h = headers
                .into_iter()
                .filter_map(|x| Some((x.0?.as_str().to_string(), x.1.to_str().ok()?.to_string())))
                .collect::<Vec<(String, String)>>();
            // Create a client to the specified host, no persistence
            let create_opts = mqtt::CreateOptionsBuilder::new()
                .mqtt_version(3)
                .server_uri(&url)
                .client_id(client_id)
                // .persistence(None)
                .finalize();
            let cli = mqtt::AsyncClient::new(create_opts).unwrap();
            //如果我们的连接丢失，LWT将广播到组
            //但在广播之前，请等待30秒重新连接。
            let lwt_props = mqtt::properties! {
                mqtt::PropertyCode::WillDelayInterval => 10
            };
            let lwt = mqtt::MessageBuilder::new()
                .topic(&chat_topic)
                .payload(format!("<<< {} left the group >>>", chat_user))
                .qos(QOS)
                .properties(lwt_props)
                .finalize();
            // Session will exist for a day (86,400 sec) between connections.
            let props = mqtt::properties! {
                mqtt::PropertyCode::SessionExpiryInterval => 86400
            };
            let mut trust_store = env::current_dir().unwrap();
            trust_store.push("_.zhipin.crt");
            let mut key_store = env::current_dir().unwrap();
            key_store.push("_.zhipin.pem");
            if !trust_store.exists() {
                println!("The trust store file does not exist: {:?}", trust_store);
                println!(
                    "  Get a copy from \"paho.mqtt.c/test/ssl/{:?}\"",
                    trust_store
                );
                process::exit(1);
            }

            if !key_store.exists() {
                println!("The key store file does not exist: {:?}", key_store);
                println!("  Get a copy from \"paho.mqtt.c/test/ssl/{:?}\"", key_store);
                process::exit(1);
            }
            let ssl = SslOptionsBuilder::new()
                .trust_store(trust_store)
                .unwrap()
                .key_store(key_store)
                .unwrap()
                .finalize();
            // Connect with MQTT v5 and a persistent server session (no clean start).
            // For a persistent v5 session, we must set the Session Expiry Interval
            // on the server. Here we set that requests will persist for a day
            // (86,400sec) if the service disconnects or restarts.
            let conn_opts = mqtt::ConnectOptionsBuilder::new_ws()
                // .ssl_options(ssl)
                // .clean_session(true)
                // .properties(props)
                // .will_message(lwt)
                .http_headers(&h)
                .user_name(&chat_user)
                .password("DSRShXEA8g2KB4aW5wqqiI-npg0Z0c-Yc7G2mGrRPl78ykR9E2u3k-ozbwxzA28YowZbXhGEl2Kn55cOznd_cOw~~")
                // .connect_timeout(Duration::from_millis(25*1000))
                .finalize();

            // Set a closure to be called when the client loses the connection.
            // It will simply end the session.
            cli.set_connection_lost_callback(|_cli| {
                println!("*** Connection lost ***");
                process::exit(2);
            });

            // Attach a closure to the client to receive callbacks on incoming
            // messages. Just print them to the console.
            cli.set_message_callback(|_cli, msg| {
                if let Some(msg) = msg {
                    println!("msg -> {}", msg.payload_str());
                }
            });
            // Connect and wait for it to complete or fail

            if let Err(err) = cli.connect(conn_opts).await {
                eprintln!("Unable to connect: {}", err);
                process::exit(1);
            }
            debug!("成功建立{}(WebSocket)链接", url);
            drop(url);
            // Since we publish and subscribe to a single topic,
            // a topic instance is helpful.
            let topic = mqtt::Topic::new(&cli, chat_topic, QOS);
            // Subscribe to the group messages.
            println!("Joining the group '{}'...", chat_group);
            let r = topic.subscribe_with_options(NO_LOCAL, None).await.unwrap();
            // Let everyone know that a new user joined the group
            let r = topic
                .publish(format!("<<< {} joined the group >>>", chat_user))
                .await
                .unwrap();
            // Read messages from the console and publish them.
            // Quit when the use enters an empty line, or a read error occurs.

            loop {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let msg = input.trim();
                        if msg.is_empty() {
                            break;
                        }

                        // Publish payload as "<user>: <message>"
                        let chat_msg = format!("{}: {}", chat_user, msg);
                        if let Err(err) = topic.publish(chat_msg).await {
                            eprintln!("Error: {}", err);
                            break;
                        }
                    }
                    Err(err) => println!("Error: {}", err),
                }
            }

            // If we're still connected, let everyone know that we left the group,
            // and then disconnect cleanly.

            if cli.is_connected() {
                println!("Leaving the group...");
                // Disconnect and tell the server to publish the LWT (after the expiry)
                let opts = mqtt::DisconnectOptionsBuilder::new()
                    .publish_will_message()
                    .finalize();
                cli.disconnect(opts).await.unwrap();
            }
        }
        ReqPlatform::Lp => todo!(),
    }
}
