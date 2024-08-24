use crate::{MC_GAME_VERSION, MC_PROTOCOL_VERSION};
use chrono::{DateTime, Utc};
use log::{error, info};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

const SERVER_PROPERTIES_FILENAME: &str = "server.properties";

pub fn server_properties_init() -> io::Result<()> {
    let filename = Path::new(SERVER_PROPERTIES_FILENAME);

    // Ouvre le fichier avec OpenOptions
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;

    let now: DateTime<Utc> = Utc::now();
    let time = now.format("%a %b %d %H:%M:%S GMT %Y").to_string();

    let content = format!(
        r#"#Minecraft server properties (for {} with protocol version {})
# {}
allow-flight=false
allow-nether=true
broadcast-console-to-ops=true
broadcast-rcon-to-ops=true
difficulty=easy
enable-command-block=false
enable-jmx-monitoring=false
enable-query=false
enable-rcon=false
enable-status=true
enforce-secure-profile=true
enforce-whitelist=false
entity-broadcast-range-percentage=100
force-gamemode=false
function-permission-level=2
gamemode=survival
generate-structures=true
generator-settings={}
hardcore=false
hide-online-players=false
initial-disabled-packs=
initial-enabled-packs=vanilla
level-name=world
level-seed=
level-type=minecraft:normal
log-ips=true
max-chained-neighbor-updates=1000000
max-players=20
max-tick-time=60000
max-world-size=29999984
motd=A Minecraft Server
network-compression-threshold=256
online-mode=true
op-permission-level=4
player-idle-timeout=0
prevent-proxy-connections=false
pvp=true
query.port=25565
rate-limit=0
rcon.password=
rcon.port=25575
require-resource-pack=false
resource-pack=
resource-pack-prompt=
resource-pack-sha1=
server-ip=
server-port=25565
simulation-distance=10
spawn-animals=true
spawn-monsters=true
spawn-npcs=true
spawn-protection=16
sync-chunk-writes=true
text-filtering-config=
use-native-transport=true
view-distance=10
white-list=false
"#,
        MC_GAME_VERSION, MC_PROTOCOL_VERSION, time, ""
    );

    // Écrire le contenu dans le fichier
    if let Err(e) = file.write_all(content.as_bytes()) {
        error!("Failed to write to file '{}': {}", filename.display(), e);
        return Err(e);
    }

    info!("Server properties created");
    Ok(())
}

pub fn check_server_properties() -> io::Result<()> {
    let server_path = Path::new(SERVER_PROPERTIES_FILENAME);

    if !server_path.exists() {
        return server_properties_init();
    }

    info!("server.properties file found");
    Ok(())
}

pub fn delete_server_properties() {
    let server_path = Path::new(SERVER_PROPERTIES_FILENAME);

    match fs::remove_file(server_path) {
        Ok(_) => info!("server.properties deleted"),
        Err(e) => error!("Error when deleting '{}': {}", server_path.display(), e),
    }
}
