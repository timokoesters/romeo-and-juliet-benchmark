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
    convert::TryFrom, fs::File, io::{BufRead, BufReader},
};

const PASSWORD: &str = "asljdfbdnfsd";
const ROOM_ID: &str = "!VnpxPgWzIdObOCKfvL:synapse-server-name";

#[derive(Default)]
struct State {
    clients: HashMap<String, HttpClient>, // Maps user ids to clients
    counter: u32, 
}

impl State {
    pub async fn say(&mut self, displayname: String, line: String) {
        let username = Self::fix_username(displayname.clone());

        let mut entry = self.clients.entry(username.clone());
        let client: &mut HttpClient = match entry {
            Entry::Occupied(ref mut e) => e.get_mut(),
            Entry::Vacant(e) => e.insert(Self::new_client(username, displayname).await),
        };

        self.counter += 1;

        client
            .request(r0::message::send_message_event::Request {
                room_id: &RoomId::try_from(ROOM_ID).unwrap(),
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

    async fn new_client(username: String, displayname: String) -> HttpClient {
        let client = Client::new("http://localhost:8008".parse().unwrap(), None);
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
            .unwrap().identification.unwrap().user_id;

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
                room_id: &RoomId::try_from(ROOM_ID).unwrap(),
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
async fn main() {
    // Log info by default
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let mut state = State::default();

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
