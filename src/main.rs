extern crate ini;
extern crate discord;

use ini::Ini;
use discord::Discord;
use discord::model::Event;

fn main() {
    // Login and establish connection.
    let conf = Ini::load_from_file("conf.ini").unwrap();
    let client = Discord::from_bot_token(&conf["discord"]["token"]).expect("Login failed.");
    let (mut conn, _) = client.connect().expect("Connection failed.");

    // Here's where stuff would normally be done.
    loop {
        let event = conn.recv_event().expect("Could not recieve events.");
        if let Event::MessageCreate(message) = event {
            if message.content == "!ping" {
                match client.send_message(message.channel_id, "*pong*", "", false) {
                    Err(e) => println!("[WARN] Failed to send message: {:?}", e),
                    _ => {}
                }
            }
        }
    }
}
