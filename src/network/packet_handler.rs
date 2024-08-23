use std::io::Write;
use std::net::TcpStream;
use log::{info, warn};
use crate::math::var_int_and_long::{read_var_int, write_var_int};
use crate::MC_PROTOCOL_VERSION;

pub fn packet_listener(data: Vec<u8>, state: &mut u8, stream: &mut TcpStream) {
    let mut uuid: Option<String> = None;
    let mut player_name: Option<String> = None;

    match (*state, data[1]) {
        (0, 0) => {
            // Handshake

            handle_handshake(&data, state);
        },
        (1, 0) => {
            // TODO => Add status (https://wiki.vg/Protocol)
        }
        (2, 0) => {
            // Login start and we send login success without encryption

            let (uuid_str, player_name_str) = handle_login_start(&data, state);
            uuid = Some(uuid_str);
            player_name = Some(player_name_str);

            let packet_login_success = send_login_success(&data, state, uuid.as_ref().unwrap(), player_name.as_ref().unwrap());
            stream.write_all(&packet_login_success).unwrap();
        },
        (2, 3) => {
            // Login acknowledged

            handle_login_acknowledged(&data, state);
        },
        // https://wiki.vg/index.php?title=Protocol&oldid=18641#Serverbound_5
        (3, 0) => { // Confirm Teleportation
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 1) => { // Query Block Entity Tag ; Used when F3+I is pressed while looking at a block.
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 2) => { // Change Difficulty ; ONLY OP CAN DO THIS ; 0: peaceful, 1: easy, 2: normal, 3: hard ; Appears to only be used on single-player
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 3) => { // Acknowledge Message
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 4) => { // Chat Command
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 5) => { // Chat Message
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 6) => { // Player Session
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 7) => { // Chunk Batch Received ; Notifies the server that the chunk batch has been received by the client.
                    // The server us es the value sent in this packet to adjust the number of chunks to be sent in a batch.
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 8) => { // Client Status
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 9) => { // Client Information (play) ; Sent when the player connects, or when settings are changed ; Looks like the other Client Information
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x0A) => { // Command Suggestions Request ; Sent when the client needs to tab-complete a minecraft:ask_server suggestion type.
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x0B) => { // Acknowledge Configuration ; Sent by the client upon receiving a Start Configuration packet from the server.
                       // This packet switches the connection state to configuration.
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x0C) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x0D) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x0E) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x0F) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x10) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x11) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x12) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x13) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x14) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x15) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x16) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x17) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x18) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x19) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (3, 0x1A) => {
            info!("Play / {}\nbuffer:{:?}", data[1], data);
        },
        (4, 0) => {
            // Client info
            // TODO => Allow the server configuration to send other packets like texture packs (https://wiki.vg/index.php?title=Protocol&oldid=18641#Configuration)

            handle_client_info(&data, state); // Read the client information packet

            // ===========================================================================================================================================================================
            // Sent by the server to notify the client that the configuration process has finished. The client answers with its own Finish Configuration whenever it is ready to continue.
            // ===========================================================================================================================================================================
            let packet_finish_config = send_packet(0x01, 0x02, None);
            stream.write_all(&packet_finish_config).unwrap();
            info!("Finish config packet sent!");

            handle_finish_configuration(&data, state);
        },
        (4, 2) => {
            // Finish Configuration (from client)


        }
        _ => {}
    }
}

// ==============================================================
// Handle the "Handshake" package and switch to the target state.
// ==============================================================
fn handle_handshake(data: &Vec<u8>, state: &mut u8) {
    let total_length = data[0] as usize;
    let protocol_version = vec![data[2],data[3]];

    let next_state = read_var_int(&[data[total_length]]).unwrap();

    info!(
        "Packet type: Handshake\nTotal length: {:?}\nID: {:?}\nProtocol version: {:?}\nNext state: {:?}",
        read_var_int(&[data[0]]).unwrap(),
        read_var_int(&[data[1]]).unwrap(),
        read_var_int(&protocol_version).unwrap(),
        next_state
    );

    if read_var_int(&protocol_version).unwrap() != MC_PROTOCOL_VERSION.parse().unwrap() {
        if read_var_int(&protocol_version).unwrap() < 754 {
            warn!("multiplayer.disconnect.outdated_client");
        } else {
            warn!("multiplayer.disconnect.incompatible");
        }
    }

    if next_state == 2{
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

    let uuid_str = uuid // UUID to string
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("");

    let player_name_str = String::from_utf8(player_name.to_vec()).unwrap(); // Conversion des octets en UTF-8

    info!(
        "Packet type: Login start\nPlayer name: {:?}\nUUID: {:?}",
        player_name_str, uuid_str
    );

    (uuid_str, player_name_str) // Retourne l'UUID et le nom du joueur
}

// =================================
// Send the "Login Success" package.
// =================================
fn send_login_success(_data: &Vec<u8>, _state: &mut u8, uuid: &String, pn: &String) -> Vec<u8>{
    let id = 0x02; // Packet ID (login success)

    // Transform UUID => Bytes
    let uuid_bytes = uuid.replace('-', "");
    let uuid_bytes = (0..32).step_by(2)
        .map(|i| u8::from_str_radix(&uuid_bytes[i..i+2], 16).unwrap())
        .collect::<Vec<u8>>();

    // Construct the player name in a VarInt
    let player_name_bytes = pn.as_bytes();
    let mut data = Vec::new();
    data.extend_from_slice(&uuid_bytes);
    data.extend_from_slice(&write_var_int(player_name_bytes.len() as i32));
    data.extend_from_slice(player_name_bytes);

    // Properties
    data.extend_from_slice(&write_var_int(0)); // No properties
    // TODO => Setting properties for future...

    // Calculate the length of the packet
    let length = write_var_int(data.len() as i32).len() + data.len(); // length + ID + data

    // Building the packet
    let mut packet = Vec::new();
    packet.extend_from_slice(&write_var_int(length as i32)); // Total packet length
    packet.push(id as u8); // Packet's ID
    packet.extend_from_slice(&data);

    info!("Login success packet sent");
    packet
}

// =========================================================================
// Handle the "Login Acknowledged" packet and change state to Configuration.
// =========================================================================
fn handle_login_acknowledged(data: &Vec<u8>, state: &mut u8){
    info!("Packet type: Login acknowledged\nID: {:?}",
        read_var_int(&[data[1]]).unwrap());

    *state = 4; // We set the state to configuration...
    info!("State = 4 (Configuration)")
}

// ========================================
// Handle the "Client information" package.
// ========================================
fn handle_client_info(data: &Vec<u8>, _state: &mut u8) { // TODO => The info in this can change some process in the server
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
// Handle "Finish Configuration" packet and change state to Play.
// ==============================================================
fn handle_finish_configuration(data: &Vec<u8>, state: &mut u8){

    info!("Packet type: Finish Configuration\nID: {}",
        read_var_int(&[data[1]]).unwrap());

    *state = 3;
    info!("State = 3 (Play)");
}

// ===================================================================
// Take length, ID and data and put them together in one clean packet.
// ===================================================================
fn send_packet(length: u8, id: u8, data: Option<Vec<u8>>) -> Vec<u8> {
    // If data = None ; create an empty vector
    let data = data.unwrap_or_else(Vec::new);

    let mut packet = Vec::with_capacity(2 + data.len());
    packet.push(length);
    packet.push(id);
    packet.extend_from_slice(&data);
    packet
}
