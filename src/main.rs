use log::debug;
use r0::room::{create_room::RoomPreset, Visibility};
use ruma::{
    api::client::r0,
    events::{room::message::MessageEventContent, AnyMessageEventContent},
    RoomId,
};
use ruma_client::Client;
use std::{
    collections::{hash_map::Entry, HashMap},
    env,
    fs::File,
    io::{BufRead, BufReader},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

const PASSWORD: &str = "asljdfbdnfsd";

type MatrixClient = ruma_client::Client<ruma_client::http_client::HyperNativeTls>;

struct State {
    server: String,
    room_id: RoomId,
    clients: HashMap<String, MatrixClient>, // Maps username to client
    id: String,
    counter: u32,
}

impl State {
    pub fn new(server: String, room_id: RoomId, id: String) -> Self {
        State {
            server,
            room_id,
            clients: HashMap::new(),
            id,
            counter: 0,
        }
    }

    pub async fn say(&mut self, displayname: String, line: String) {
        let username = Self::fix_username(displayname.clone()) + "_" + &self.id;

        let mut entry = self.clients.entry(username.clone());
        let client = match entry {
            Entry::Occupied(ref mut e) => e.get_mut(),
            Entry::Vacant(e) => e.insert(
                Self::new_client(self.server.clone(), &self.room_id, username, displayname).await,
            ),
        };

        self.counter += 1;

        client
            .send_request(r0::message::send_message_event::Request::new(
                &self.room_id,
                &self.counter.to_string(),
                &AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(line)),
            ))
            .await
            .unwrap();
    }

    async fn new_client(
        server: String,
        room_id: &RoomId,
        username: String,
        displayname: String,
    ) -> MatrixClient {
        let client = Client::new(server, None);
        debug!("Trying to register...");
        client
            .register_user(Some(dbg!(&username)), PASSWORD)
            .await
            .unwrap();

        debug!("Trying to log in...");
        let user_id = client
            .log_in(&username, PASSWORD, None, None)
            .await
            .unwrap()
            .user_id;

        debug!("Trying to set the display name...");
        client
            .send_request(r0::profile::set_display_name::Request::new(
                &user_id,
                Some(&displayname),
            ))
            .await
            .unwrap();

        debug!("Trying to join the room...");
        client
            .send_request(r0::membership::join_room_by_id::Request::new(&room_id))
            .await
            .unwrap();

        client
    }

    fn fix_username(username: String) -> String {
        username
            .to_ascii_lowercase()
            .replace(|c: char| !c.is_ascii_alphanumeric(), "_")
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    // Log info by default
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let mut args = env::args();

    let program_path = args.next().unwrap();

    if args.len() < 1 {
        eprintln!("Usage: time {} <server1> [<server..>]", program_path);
        return Ok(());
    }

    let servers: Vec<String> = args.filter_map(|s| s.parse().ok()).collect();

    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time is valid")
        .as_millis()
        .to_string();

    // Use one client to create the room
    let room_id = {
        let client = MatrixClient::new(servers[0].clone(), None);

        let first_username = format!("user_{}", id);

        client
            .register_user(Some(&first_username), PASSWORD)
            .await
            .unwrap();

        client
            .log_in(&first_username, PASSWORD, None, None)
            .await
            .unwrap();

        let mut create = r0::room::create_room::Request::new();
        let name = format!("Romeo and Juliet {}", id);
        create.name = Some(&name);
        create.preset = Some(RoomPreset::PublicChat);
        create.visibility = Visibility::Public;

        let room_id = client.send_request(create).await.unwrap().room_id;

        room_id
    };

    // Join with all other servers
    for server in servers.iter().skip(1) {
        let client = MatrixClient::new(server.clone(), None);

        let first_username = format!("user_{}", id);

        println!("{}", first_username);
        client
            .register_user(Some(&first_username), PASSWORD)
            .await
            .unwrap();

        client
            .log_in(&first_username, PASSWORD, None, None)
            .await
            .unwrap();

        client
            .send_request(r0::membership::join_room_by_id::Request::new(&room_id))
            .await
            .unwrap();
    }

    let start_time = Instant::now();
    let mut futs = Vec::new();
    for server in servers {
        let state = State::new(server, room_id.clone(), id.clone());
        futs.push(play(state));
    }

    futures::future::join_all(futs).await;
    println!("Result: {:?}", start_time.elapsed());

    Ok(())
}

async fn play(mut state: State) {
    let file = File::open("romeo_and_juliet.txt").unwrap();
    let reader = BufReader::new(file);

    let mut displayname = "STAGE DIRECTION".to_owned();

    let mut line_iter = reader.lines();

    while let Some(line) = line_iter.next() {
        let mut line = line.unwrap();

        if line.trim().is_empty() {
            displayname = "STAGE DIRECTION".to_owned();
            continue;
        }

        if line.starts_with(' ') {
            line = line.trim().to_owned();
            state.say(displayname.clone(), line).await;
        } else if line.starts_with("ACT") {
            line = line.trim().to_owned();
            state.say("ACTS".to_owned(), line).await;
        } else {
            line = line.trim().to_owned();
            displayname = line;
            // Skip a line
            line_iter.next();
        }
    }
}
