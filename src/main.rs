use log::{debug, warn};
use ruma::{
    api::{client::r0, exports::serde_json},
    events::{
        room::message::{MessageEventContent, TextMessageEventContent},
        AnyMessageEventContent, EventType,
    },
    RoomId,
};
use ruma_client::{Client, HttpClient};
use std::{
    collections::{hash_map::Entry, HashMap},
    convert::TryFrom,
    env,
    fs::File,
    io::{BufRead, BufReader},
};
use hyper::Uri;

const PASSWORD: &str = "asljdfbdnfsd";

struct State {
    server: Uri,
    room_id: RoomId,
    clients: HashMap<String, HttpClient>, // Maps user ids to clients
    counter: u32,
}

impl State {
    pub fn new(server: Uri, room_id: RoomId) -> Self {
        State {
            server,
            room_id,
            clients: HashMap::new(),
            counter: 0,
        }
    }

    pub async fn say(&mut self, displayname: String, line: String) {
        let username = Self::fix_username(displayname.clone());

        let server = self.server.clone();
        let mut entry = self.clients.entry(username.clone());
        let client: &mut HttpClient = match entry {
            Entry::Occupied(ref mut e) => e.get_mut(),
            Entry::Vacant(e) => e.insert(Self::new_client(server, &self.room_id, username, displayname).await),
        };

        self.counter += 1;

        client
            .request(r0::message::send_message_event::Request {
                room_id: &self.room_id,
                event_type: EventType::RoomMessage,
                txn_id: &self.counter.to_string(),
                data: serde_json::value::to_raw_value(&AnyMessageEventContent::RoomMessage(
                    MessageEventContent::Text(TextMessageEventContent {
                        body: line,
                        formatted: None,
                        relates_to: None,
                    }),
                ))
                .unwrap(),
            })
            .await
            .unwrap();
    }

    async fn new_client(server: Uri, room_id: &RoomId, username: String, displayname: String) -> HttpClient {
        let client = Client::new(server, None);
        debug!("Trying to register...");
        if client
            .register_user(Some(dbg!(username.clone())), PASSWORD.to_owned())
            .await
            .is_err()
        {
            warn!("Unable to register. Already registered?");
        }

        debug!("Trying to log in...");
        let user_id = client
            .log_in(username, PASSWORD.to_owned(), None, None)
            .await
            .unwrap()
            .identification
            .unwrap()
            .user_id;

        debug!("Trying to set the display name...");
        client
            .request(r0::profile::set_display_name::Request {
                user_id,
                displayname: Some(displayname),
            })
            .await
            .unwrap();

        debug!("Trying to join the room...");
        client
            .request(r0::membership::join_room_by_id::Request {
                room_id: &room_id,
                third_party_signed: None,
            })
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
async fn main() -> Result<(), Box<String>> {
    // Log info by default
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let mut args = env::args();

    let program_path = args.next().unwrap();

    let server = args
        .next()
        .ok_or_else(|| format!("Usage: {} <server url> <roomid>", program_path))?
        .parse()
        .map_err(|_| "Invalid server url".to_owned())?;

    let room_id = RoomId::try_from(
        args.next()
            .ok_or_else(|| format!("Usage: {} <server url> <roomid>", program_path))?,
    )
    .map_err(|_| "Invalid RoomId".to_owned())?;

    let mut state = State::new(server, room_id);

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

    Ok(())
}
