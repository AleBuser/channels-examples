#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_lib_rs::prelude::iota_client;
use iota_streams::app_channels::{
    api::tangle::{ Address,Subscriber}
    , message
};
use iota_streams::app::transport::Transport;
use iota_streams::app::transport::tangle::client::SendTrytesOptions;
use failure::{Fallible, ensure};


pub struct Channel{
    subscriber: Subscriber,
    client: iota_client::Client<'static>,
    send_options: SendTrytesOptions,
    channel_address:String,
}

impl Channel {

    pub fn new(seed: &str, node_ulr: &'static str, channel_address:String) -> Channel{

        let subscriber = Subscriber::new(seed, true);
        let mut send_opt = SendTrytesOptions::default();

        send_opt.min_weight_magnitude = 9;
        send_opt.local_pow = false;

        Self {
            subscriber: subscriber,
            client: iota_client::Client::new(node_ulr),
            send_options: send_opt,
            channel_address: channel_address,
        }
    }


    pub fn connect(&mut self, announce_message_identifier: &str) -> Fallible<()>{
       
        let announcement_link = Address::from_str(&self.channel_address, &announce_message_identifier).unwrap();
 
        println!("Receiving announcement messages");
     
        let list = self.client.recv_messages_with_options(&announcement_link, () )?;
        
        let mut found_valid_msg = false;
        for tx in list.iter() {
            let header = tx.parse_header()?;
            ensure!(header.check_content_type(message::announce::TYPE));
            self.subscriber.unwrap_announcement(header.clone())?;
            println!("Found and authenticated {} message", header.content_type());
            found_valid_msg = true;
            break;
        }
        // Make sure that at least one of the messages were valid 
        ensure!(found_valid_msg);
        println!("Subscribing to channel");
        // Send a Subscribe message to the first valid Announce message that was found on the Tangle
        let subscription = self.subscriber.subscribe(&announcement_link)?;
        println!("Subscribe message identifier  {}", subscription.link.msgid);
        self.client.send_message_with_options(&subscription, self.send_options)?;
        println!("Sent Subscribe message");
        Ok(())
    }

    pub fn read(&mut self, signed_message_identifier: &str) -> Fallible<()>{

        let message_link = Address::from_str(&self.channel_address, &signed_message_identifier).unwrap();
 
        println!("Receiving signed messages");
    
        // Use the IOTA client to find transactions with the corresponding channel address and tag
        let list = self.client.recv_messages_with_options(&message_link, () )?;

        // Iterate through all the transactions and stop at the first valid message
        for tx in list.iter() {
            let header = tx.parse_header()?;
            ensure!(header.check_content_type(message::signed_packet::TYPE));
            let (public_message, private_message) = self.subscriber.unwrap_signed_packet(header.clone())?;
            println!("Found and authenticated messages");
            println!("Public message: {}, private message: {}", public_message, private_message);
            break;
        }
        Ok(())

    }

      
}