extern crate ini;
extern crate serenity;

use ini::Ini;
use serenity::{
    client::{Client, EventHandler},
    prelude::Context,
    model::channel::Message
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(e) = msg.reply("*pong*") {
                println!("[WARN] Failed to send message: {:?}", e);
            }
        }
    }
}

fn main() {
    // Login and establish connection.
    let conf = Ini::load_from_file("conf.ini").unwrap();
    let mut client = Client::new(&conf["discord"]["token"], Handler).unwrap();

    if let Err(e) = client.start() {
        println!("[FATAL] The bot encountered an error: {:?}", e);
    }
}
