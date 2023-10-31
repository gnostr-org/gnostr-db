use serde::{Deserialize, Serialize};
use tokio::{net::{TcpListener, TcpStream, UdpSocket}, sync::RwLock};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use mac_address::MacAddressError;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use rand::{seq::SliceRandom, thread_rng, Rng};

const LOCAL_ADDR: &str = "0.0.0.0";
const LOCAL_TCP_PORT: u16 = 8888;

// #[cfg(debug_assertions)]
// const BROADCAST_ADDR5: &str = "255.255.255.255:8885";
// #[cfg(debug_assertions)]
// const BROADCAST_ADDR4: &str = "255.255.255.254:8884";
// #[cfg(debug_assertions)]
// const BROADCAST_ADDR3: &str = "255.255.255.253:8883";
// #[cfg(debug_assertions)]
// const BROADCAST_ADDR2: &str = "255.255.255.252:8882";
// #[cfg(debug_assertions)]
// const BROADCAST_ADDR1: &str = "255.255.255.251:8881";

const BROADCAST_ADDR: &str = "255.255.255.251:8888";

#[cfg(debug_assertions)]
const TCP_PORT9005: u16 = 9005;
#[cfg(debug_assertions)]
const TCP_PORT9004: u16 = 9004;
#[cfg(debug_assertions)]
const TCP_PORT9003: u16 = 9003;
#[cfg(debug_assertions)]
const TCP_PORT9002: u16 = 9002;
#[cfg(debug_assertions)]
const TCP_PORT9001: u16 = 9001;

const TCP_PORT9000: u16 = 9000;

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Handshake { node_name: String, tcp_addr: SocketAddr },
    Greeting,
    Heartbeat,
    HeartbeatResponse,
    SetValue { key: String, value: String }, // New Message for setting a value
    GetValue { key: String },                // New Message for getting a value
    ValueResponse { value: Option<String> }, // New Message for sending back the value or an acknowledgment
    Sync { key: String, value: String }, // New message for synchronization
}

// Create a new struct for the key-value store
struct KeyValueStore {
    store: RwLock<HashMap<String, String>>,
}

impl KeyValueStore {
    fn new() -> Self {
        KeyValueStore {
            store: RwLock::new(HashMap::new()),
        }
    }

    async fn set(&self, key: String, value: String) {
        let mut store = self.store.write().await;
        store.insert(key, value);
    }

    async fn get(&self, key: &str) -> Option<String> {
        let store = self.store.read().await;
        store.get(key).cloned()
    }
}

struct NodeInfo {
    last_seen: std::time::Instant,
    tcp_addr: SocketAddr,
}

fn get_mac_address() -> Result<String, MacAddressError> {
    let mac = mac_address::get_mac_address()?;
    match mac {
        Some(address) => Ok(address.to_string()),
        None => Err(MacAddressError::InternalError),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let oui_prefixes: Vec<&str> = vec![
        "00:0E:F6", "00:08:28", "84:7A:88", "D8:A2:5E", "00:30:0B", "00:26:0C", "A4:C5:4E",
        "94:86:CD", "E0:35:60", "00:19:BC", "70:01:36", "FC:1F:C0", "00:E0:DE", "00:07:19",
        "00:1B:AF", "00:24:27", "28:4F:CE", "00:22:A0", "74:40:BB", "28:E7:94", "C4:93:00",
        "30:A2:20", "00:17:88", "02:5B:76", "2C:F4:32", "40:16:7E", "40:B0:76", "5C:A6:E6",
        "60:E3:27", "68:9A:87", "62:5C:65", "68:9A:87", "A0:8C:FD", "B8:BB:AF", "F0:F2:74",
        "F0:35:75", "F2:34:11", "CC:40:D0", "20:4E:7F", "AC:37:43", "04:0E:C2", "18:2B:05",
        "00:04:D2", "24:B6:FD", "F0:46:3B", "00:50:FF", "00:0B:2A", "00:1F:ED", "00:22:FB",
        "CC:3F:EA", "00:02:BF",
    ];

    let iters = if std::env::args().any(|arg| arg.parse::<u32>().is_ok()) {
        let arg_num = std::env::args()
            .find(|arg| arg.parse::<u32>().is_ok())
            .expect("Pre-checked for numeric arg.")
            .parse::<u32>()
            .expect("Already verified as parsable arg.");

        if arg_num == 0 {
            1
        } else {
            arg_num
        }
    } else {
        1
    }; // end let iters

    if std::env::args().any(|arg| arg.to_lowercase() == "-h" || arg.to_lowercase() == "--help") {

        println!("Usage: ./macgen [-n (won't append newline)] [num (e.g 5)]");

    } else {

        for _ in 0..iters {

            let mut rng = thread_rng();
            let fake_addr: u64 = thread_rng().gen_range(0x100000..=0xffffff);
            #[cfg(debug_assertions)]
            println!("fake_addr={}", fake_addr);
            let fake_addr_str = format!("{:2X}", fake_addr);
            #[cfg(debug_assertions)]
            println!("fake_addr_str={}", fake_addr_str);

            let substrings = fake_addr_str
                .as_bytes()
                .chunks(2)
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .expect("Expected valid chunks!");

            #[cfg(debug_assertions)]
            println!("substrings={:?}", substrings);

            let formatted_addr = substrings.join(":");

            if std::env::args().any(|arg| arg == "-n") {
                print!(
                    "{}:{} ",
                    oui_prefixes
                        .choose(&mut rng)
                        .expect("Vec should not be empty."),
                    formatted_addr
                );
            } else {
                println!(
                    "{}:{}",
                    oui_prefixes
                        .choose(&mut rng)
                        .expect("Vec should not be empty."),
                    formatted_addr
                );
            };
        }
    }; // end --help else not --help

    // #[cfg(debug_assertions)]
    // let local_addr_9005: SocketAddr = format!("{}:{}", "0.0.0.0", TCP_PORT9005).parse().unwrap();
    // #[cfg(debug_assertions)]
    // let local_addr_9004: SocketAddr = format!("{}:{}", "0.0.0.0", TCP_PORT9004).parse().unwrap();
    // #[cfg(debug_assertions)]
    // let local_addr_9003: SocketAddr = format!("{}:{}", "0.0.0.0", TCP_PORT9003).parse().unwrap();
    // #[cfg(debug_assertions)]
    // let local_addr_9002: SocketAddr = format!("{}:{}", "0.0.0.0", TCP_PORT9002).parse().unwrap();
    // #[cfg(debug_assertions)]
    // let local_addr_9001: SocketAddr = format!("{}:{}", "0.0.0.0", TCP_PORT9001).parse().unwrap();
    // #[cfg(debug_assertions)]
    let local_addr_9000: SocketAddr = format!("{}:{}", "0.0.0.0", TCP_PORT9000).parse().unwrap();

    let socket = UdpSocket::bind(&local_addr_9000).await?;
    socket.set_broadcast(true)?;

    //
    // Initialize the key-value store
    //
    let kv_store = Arc::new(KeyValueStore::new());
    let node = Arc::new(RwLock::new(HashMap::<String, NodeInfo>::new()));

    //
    // Use Arc to share the socket among tasks.
    //
    let socket = Arc::new(socket);
    let socket_for_broadcast = socket.clone();

    tokio::spawn(

      async move {

        match get_mac_address() {

            Ok(node_name) => {

                fn assign_tcp_addr() -> SocketAddr {

                  let tcp_addr: SocketAddr = format!("{}:{}", "0.0.0.0", TCP_PORT9000).parse().unwrap();

                  #[cfg(debug_assertions)]
                  println!("tcp_addr={}", tcp_addr);

                  tcp_addr
                }

                fn alert_user_3(s: &str) {

                  #[cfg(debug_assertions)]
                  println!("alert_user_3()");
                  println!("{}", s);

                }

                //

                (|| {

                  assign_tcp_addr();
                  Ok(())

                })().unwrap_or_else(|_err: String| {

                  alert_user_3("alert_user_3:assign_tcp_addr()");

                });

                let tcp_addr = assign_tcp_addr();

                let msg = Message::Handshake {

                  node_name: node_name.clone(), tcp_addr,

                }; // end let msg

        let serialized_msg = serde_json::to_string(&msg).unwrap();

        loop {

            println!("Sending UDP broadcast...");
            socket_for_broadcast.send_to(serialized_msg.as_bytes(), BROADCAST_ADDR).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        } //end loop

            }, // Ok(node_name)

            Err(e) => {

                eprintln!("Error fetching MAC address: {:?}", e);

            } // end Err(e)

        } // end match get_mac_address

    }); // end tokio::spawn

    let node_clone = node.clone();
    // let node_clone = Arc::new(RwLock::new(HashMap::<String, NodeInfo>::new()));

    tokio::spawn(

      async move {

        let listener = TcpListener::bind(("0.0.0.0", TCP_PORT9000)).await.unwrap();

        println!("TCP listener started.");

        while let Ok((stream, _)) = listener.accept().await {

            println!("Accepted new TCP connection.");

            tokio::spawn(

              handle_tcp_stream(

                stream,
                node_clone.clone(),
                kv_store.clone()

                ) // end handle_tcp_stream

              ); // tokio::spawn

        } // end while let Ok

    } // end async move

    ); // end tokio::spawn

    let mut buf = vec![0u8; 1024];

    loop {

        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Received data on UDP from {}", addr);

        let received_msg: Message = serde_json::from_slice(&buf[..len])?;

        let local_node_name = get_mac_address()?;

        if let Message::Handshake { node_name, tcp_addr } = received_msg {

            // Ignore packets from ourselves
            if node_name == local_node_name {

                continue;
            }
            println!("Received handshake from: {}", node_name);
            {
                let mut nodes_guard = node.write().await;
                nodes_guard.insert(

                  node_name.clone(), NodeInfo {

                    last_seen: std::time::Instant::now(), tcp_addr

                  });
            }

            let greeting = Message::Greeting;
            let serialized_greeting = serde_json::to_string(&greeting).unwrap();
            socket.send_to(serialized_greeting.as_bytes(), &addr).await?;

            // Start heartbeat for this node
            let node_clone = node.clone();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    println!("Sending heartbeat to {}", tcp_addr);
                    let mut stream = TcpStream::connect(tcp_addr).await.unwrap();
                    let heartbeat_msg = Message::Heartbeat;
                    let serialized_msg = serde_json::to_string(&heartbeat_msg).unwrap();
                    stream.write_all(serialized_msg.as_bytes()).await.unwrap();
                }
            });
        }
    }
}

async fn handle_tcp_stream(

  mut stream: TcpStream,
  node: Arc<RwLock<HashMap<String,
  NodeInfo>>>,
  kv_store: Arc<KeyValueStore>

  ) {

    let mut buf = vec![0u8; 1024];
    let len = stream.read(&mut buf).await.unwrap();
    let received_msg: Message = serde_json::from_slice(&buf[..len]).unwrap();

    match received_msg {
        Message::Heartbeat => {
            println!("Received Heartbeat");
            let response = Message::HeartbeatResponse;
            let serialized_response = serde_json::to_string(&response).unwrap();
            stream.write_all(serialized_response.as_bytes()).await.unwrap();
        },
        Message::SetValue { key, value } => {
            println!("Received SetValue");
            kv_store.set(key.clone(), value.clone()).await;

            // Broadcast sync to all nodes
            let node_guard = node.read().await;
            for (_, node_info) in node_guard.iter() {
                let mut stream = match TcpStream::connect(node_info.tcp_addr).await {
                    Ok(stream) => stream,
                    Err(_) => continue,
                };
                let sync_msg = Message::Sync { key: key.clone(), value: value.clone() };
                let serialized_msg = serde_json::to_string(&sync_msg).unwrap();
                let _ = stream.write_all(serialized_msg.as_bytes()).await;
            }

            let response = Message::ValueResponse { value: Some("Value set successfully.".to_string()) };
            let serialized_response = serde_json::to_string(&response).unwrap();
            stream.write_all(serialized_response.as_bytes()).await.unwrap();
        },
        Message::GetValue { key } => {
            println!("Received GetValue");
            let value = kv_store.get(&key).await;
            let response = Message::ValueResponse { value };
            let serialized_response = serde_json::to_string(&response).unwrap();
            stream.write_all(serialized_response.as_bytes()).await.unwrap();
        },
        Message::Sync { key, value } => {
            println!("Received Sync");
            kv_store.set(key, value).await;
        },
        _ => {}
    }
}
