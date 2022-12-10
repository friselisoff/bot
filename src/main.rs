#![feature(let_chains)]

#[warn(unused_variables)]
///! A bot that logs chat messages sent in the server to the console.
use azalea::prelude::*;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use azalea::Vec3;
use azalea_protocol::packets::game::serverbound_client_command_packet::ServerboundClientCommandPacket;
use azalea_protocol::packets::game::serverbound_interact_packet::{ActionType, InteractionHand};
use azalea_protocol::packets::game::serverbound_use_item_on_packet;
use azalea_protocol::packets::game::serverbound_use_item_on_packet::BlockHitResult;

#[tokio::main]
async fn main() {
    let account = Account::microsoft("jejfjkdskdkalfd@gmail.com").await.unwrap();

    azalea::start(azalea::Options {
        account,
        address: "157.90.56.39",
        state: State::default(),
        plugins: plugins![],
        handle,
    })
        .await
        .unwrap();
}

fn start() {}

#[derive(Default, Clone)]
pub struct State {
    pub init: Arc<Mutex<bool>>,
    pub ticks_alarm: Arc<Mutex<u32>>,
}

async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    let home_pos: Vec3 = Vec3 {
        x: 45882.5 as f64,
        y: 70 as f64,
        z: 27744.5 as f64,
    };
    let start_tp: Vec3 = Vec3 {
        x: 45886.5 as f64,
        y: 70 as f64,
        z: 27739.5 as f64,
    };

    let mut owners = vec!["friselis", "uJoBuddy", "Ihaveatrashaim", "bytecoding", "0x36", "rtm516"];

    match event {
        Event::Chat(m) => {
            if let (Some(sender), content) = m.split_sender_and_content() {
                let split = content.split(" ");
                let _splitted_content: Vec<&str> = split.collect();
                if content.starts_with("!home") {
                    if owners.contains(&&*sender) || owners.contains(&&*("HACKER | ".to_owned() + &&*sender)) {
                        let mut owner_index: f64 = 0 as f64;
                        let mut for_index: f64 = 0 as f64;
                        for owner in &owners {
                            if owner.to_string() == sender {
                                owner_index = for_index;
                            }
                            for_index = for_index + 1.0;
                        }
                        let mut to_go = start_tp.clone();
                        to_go.x = to_go.x - (owner_index * 3.0);
                        bot.set_position(to_go).await?;
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        bot.set_position(home_pos).await?;
                        bot.chat("Welcome home master").await?;
                    } else {
                        bot.chat("You're not allowed to use me").await?;
                    }
                }
                if content.starts_with("!chamber") {
                    if owners.contains(&&*sender) || owners.contains(&&*("HACKER | ".to_owned() + &&*sender)) {
                        bot.chat("Yes master").await?;
                        bot.set_position(home_pos).await?;
                    } else {
                        bot.chat("You're not allowed to use me").await?;
                    }
                }
                if content.starts_with("!bot") {
                    bot.chat("[iambot]").await?;
                }
            }
        }
        Event::Tick => {
            let init = state.init.lock().clone();
            let ticks_alarm = state.ticks_alarm.lock().clone();
            if !init {
                bot.set_position(home_pos).await?;
                bot.chat("Going back to chamber ...").await?;
                *state.init.lock() = true;
            }
            if ticks_alarm == 20 * 10 {
                let players = bot.players.read().clone();
                for (uuid, player) in players.iter() {
                    let mut message: String = String::new();
                    if let Some(entity) = bot.world.read().entity_by_uuid(uuid) {
                        if player.profile.name != bot.profile.name && !owners.contains(&&*player.profile.name) {
                            message = format!("{}{}", player.profile.name, " this is a restricted area please leave");
                        }
                    }
                    if message != "" {
                        bot.chat(&*message).await?;
                        println!("{}", &*message);
                    }
                }
                *state.ticks_alarm.lock() = 0;
            } else {
                *state.ticks_alarm.lock() = ticks_alarm + 1;
            }
        }
        Event::Death(_0) => {
            bot.write_packet(ServerboundClientCommandPacket {
                action: azalea_protocol::packets::game::serverbound_client_command_packet::Action::PerformRespawn,
            }.get()).await?;
            tokio::time::sleep(Duration::from_secs(1)).await;
            bot.set_position(home_pos).await?;
            bot.chat("Going back to chamber ...").await?;
        }
        _ => {}
    }

    Ok(())
}