use crate::math::var_int_and_long::{read_var_int, write_var_int};
use crate::MC_PROTOCOL_VERSION;
use log::{info, warn};
use std::io::Write;
use std::net::TcpStream;

//================================================================================================================
//                         This function handle EVERY packet from Minecraft Client.
// To understand this file I recommend you to check this website => https://wiki.vg (don't forget to set 1.20.2).
//                 https://wiki.vg/Protocol_FAQ#What.27s_the_normal_login_sequence_for_a_client.3F
//================================================================================================================
pub fn packet_listener(data: Vec<u8>, state: &mut u8, stream: &mut TcpStream) {
    match (*state, data[1]) {
        (0, 0x00) => {
            // Handshake

            handle_handshake(&data, state, stream);
        }
        (1, 0x00) => {
            // TODO => Add status (https://wiki.vg/Protocol)
        }
        (2, 0x00) => {
            // Login start and we send login success without encryption

            let (uuid_str, player_name_str) = handle_login_start(&data, state);
            let uuid = Some(uuid_str);
            let player_name = Some(player_name_str);

            send_login_success(
                stream,
                uuid.as_ref().unwrap(),
                player_name.as_ref().unwrap(),
            );
        }
        (2, 0x03) => {
            // Login acknowledged

            handle_login_acknowledged(&data, state);
        }
        // 3 = Play
        // https://wiki.vg/index.php?title=Protocol&oldid=18641#Serverbound_5
        (3, 0x00) => {
            // Confirm Teleportation
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        }
        (3, 0x01) => {
            // Query Block Entity Tag ; Used when F3+I is pressed while looking at a block.
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        }
        (3, 0x02) => {
            // Change Difficulty ; ONLY OP CAN DO THIS ; 0: peaceful, 1: easy, 2: normal, 3: hard ; Appears to only be used on single-player
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        }
        (3, 0x03) => {
            // Acknowledge Message
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        }
        (3, 0x04) => {
            // Chat Command
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        }
        (3, 0x05) => {
            // Chat Message
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        }
        (3, 0x06) => {
            // Player Session
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        }
        (3, 0x07) => {
            // Chunk Batch Received ; Notifies the server that the chunk batch has been received by the client.
            // The server us es the value sent in this packet to adjust the number of chunks to be sent in a batch.
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        }
        (3, 0x08) => {
            // Client Status
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        }
        (3, 0x09) => {
            // Client Information (play) ; Sent when the player connects, or when settings are changed ; Looks like the other Client Information
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        }
        (3, 0x0A) => {
            // Command Suggestions Request ; Sent when the client needs to tab-complete a minecraft:ask_server suggestion type.
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        }
        (3, 0x0B) => {
            // Acknowledge Configuration ; Sent by the client upon receiving a Start Configuration packet from the server.
            // This packet switches the connection state to configuration.
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        }
        // 4 = configuration
        (4, 0x00) => {
            // Client info
            // TODO => Allow the server configuration to send other packets like texture packs (https://wiki.vg/index.php?title=Protocol&oldid=18641#Configuration)
            info!("Client info");
            handle_client_info(&data, state); // Read the client information packet
            send_clientbound_known_packs(stream);
        }
        (4, 0x02) => {
            // Serverbound Plugin Message (https://wiki.vg/Protocol#Serverbound_Plugin_Message_.28configuration.29)
            info!("Serverbound Plugin Message buffer: {:?}", data);
        }
        (4, 0x03) => {
            // Acknowledge Finish Configuration
            info!("Handle acknowledge_finish_configuration");
            handle_acknowledge_finish_configuration(&data, state);
        }
        (4, 0x07) => {
            // Serverbound Known Packs (https://wiki.vg/Protocol#Serverbound_Known_Packs)
            info!("Serverbound Known Packs: {:?}", data);
        }
        _ => {}
    }
}

// ==============================================================
// Handle the "Handshake" package and switch to the target state.
// ==============================================================
fn handle_handshake(data: &Vec<u8>, state: &mut u8, stream: &mut TcpStream) {
    let total_length = data[0] as usize;
    let protocol_version = vec![data[2], data[3]];

    let next_state = read_var_int(&[data[total_length]]).unwrap();

    info!(
        "Packet type: Handshake\nTotal length: {:?}\nID: {:?}\nProtocol version: {:?}\nNext state: {:?}",
        read_var_int(&[data[0]]).unwrap(),
        read_var_int(&[data[1]]).unwrap(),
        read_var_int(&protocol_version).unwrap(),
        next_state
    );

    if read_var_int(&protocol_version).unwrap() != MC_PROTOCOL_VERSION {
        if read_var_int(&protocol_version).unwrap() < 754 {
            warn!("multiplayer.disconnect.outdated_client");
        } else {
            warn!("multiplayer.disconnect.incompatible");
        }
    }

    if next_state == 2 {
        *state = 2;
        info!("State = 2 (Login)");
    }
}

// ===========================================================================
// Handle the "Login Start" packet and return the Player UUID and Player Name.
// ===========================================================================
fn handle_login_start(data: &Vec<u8>, _state: &mut u8) -> (String, String) {
    let player_name_length = data[2] as usize;
    let player_name = &data[3..3 + player_name_length];
    let uuid = &data[3 + player_name_length..3 + player_name_length + 16];

    let uuid_str = uuid
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("");

    let player_name_str = String::from_utf8(player_name.to_vec()).unwrap();

    info!(
        "Packet type: Login start\nPlayer name: {:?}\nUUID: {:?}",
        player_name_str, uuid_str
    );

    (uuid_str, player_name_str)
}

// =================================
// Send the "Login Success" package.
// =================================
fn send_login_success(stream: &mut TcpStream, uuid: &String, pn: &String) {
    let id = 0x02; // Packet ID (login success)

    // Transform UUID => Bytes
    let uuid_bytes = uuid.replace('-', "");
    let uuid_bytes = (0..32)
        .step_by(2)
        .map(|i| u8::from_str_radix(&uuid_bytes[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();

    // Construct the player name in a VarInt
    let player_name_bytes = pn.as_bytes();
    let mut data = Vec::new();
    data.extend_from_slice(&uuid_bytes);
    data.extend_from_slice(&write_var_int(player_name_bytes.len() as i32));
    data.extend_from_slice(player_name_bytes);

    // Properties
    data.extend_from_slice(&write_var_int(0));
    // TODO => Setting properties for future...

    // Calculate the length of the packet
    let length = (write_var_int(data.len() as i32).len() + data.len()) as u8; // length + ID + data

    info!("Login success packet sent");
    send_packet(stream, length, id, Some(data));
}

// =========================================================================
// Handle the "Login Acknowledged" packet and change state to Configuration.
// =========================================================================
fn handle_login_acknowledged(data: &Vec<u8>, state: &mut u8) {
    info!(
        "Packet type: Login acknowledged\nID: {:?}",
        read_var_int(&[data[1]]).unwrap()
    );

    *state = 4; // We set the state to configuration...
    info!("State = 4 (Configuration)")
}

// ========================================
// Handle the "Client information" package.
// ========================================
fn handle_client_info(data: &Vec<u8>, _state: &mut u8) {
    // TODO => The info in this can change some process in the server
    let client_lang_length = data[2] as usize;
    let client_lang = &data[3..2 + client_lang_length];
    let view_distance = data[2 + client_lang_length + 1];
    let chat_mode = data[2 + client_lang_length + 2]; // var_int
    let chat_color = data[2 + client_lang_length + 3]; //bool
    let displayed_skin_part = data[2 + client_lang_length + 4]; // unsigned byte
    let main_hand = data[2 + client_lang_length + 5]; // bool
    let enable_text_filtering = data[2 + client_lang_length + 6]; // bool
    let allow_server_listing = data[2 + client_lang_length + 7]; // bool

    info!("Packet type: Client info\nClient language: {:?}\nView distance: {:?}\nChat mode: {:?} (0 = enabled)\nChat colors: {:?} (1 = true)\nDisplayed skin part: {:?}\nMain hand: {:?} (1 = right)\nEnable text filtering: {:?} (1= true)\nAllow server listing: {:?} (1 = true)",
        read_var_int(&client_lang).unwrap(),
        read_var_int(&[view_distance]).unwrap(),
        read_var_int(&[chat_mode]).unwrap(),
        read_var_int(&[chat_color]).unwrap(),
        read_var_int(&[displayed_skin_part]).unwrap(),
        read_var_int(&[main_hand]).unwrap(),
        read_var_int(&[enable_text_filtering]).unwrap(),
        read_var_int(&[allow_server_listing]).unwrap())
}

// ==============================================================
// Handle "Acknowledge Finish Configuration" packet and change state to Play.
// ==============================================================
fn handle_acknowledge_finish_configuration(data: &Vec<u8>, state: &mut u8) {
    info!(
        "Packet type: Acknowledge Finish Configuration\nID: {}",
        read_var_int(&[data[1]]).unwrap()
    );

    *state = 3;
    info!("State = 3 (Play)");
}

// The issue for now is that the client do not send something after we use this so the error is maybe here...
fn send_clientbound_known_packs(stream: &mut TcpStream) {
    let id = 0x0E;

    let mut data = Vec::new();
    data.extend(write_var_int(1)); // Known Pack count
    let mut required_pack: Vec<u8> = Vec::new();
    required_pack.extend(write_var_int("minecraft".len() as i32));
    required_pack.extend(b"minecraft"); // ressource pack name
    required_pack.extend(write_var_int("core".len() as i32));
    required_pack.extend(b"core"); // ressource pack ID
    required_pack.extend(write_var_int("1.21".len() as i32));
    required_pack.extend(b"1.21"); // ressource pack version
    data.extend(required_pack);

    let length = (2 + data.len()) as u8;

    info!("Clientbound known packs sent!");
    send_packet(stream, length, id, Some(data));
}

fn send_finish_configuration(stream: &mut TcpStream) {
    let id = 0x03;

    info!("Finish configuration packet sent!");
    send_packet(stream, 2, id, None);
}

// ====================================================================
// Take length, ID and data then put them together in one clean packet.
// ====================================================================
fn send_packet(stream: &mut TcpStream, length: u8, id: u8, data: Option<Vec<u8>>) {
    // If data = None ; create an empty vector
    let data = data.unwrap_or_else(Vec::new);

    let mut packet = Vec::with_capacity(2 + data.len());
    packet.push(length);
    packet.push(id);
    packet.extend_from_slice(&data);
    stream.write_all(&packet).unwrap();
}
