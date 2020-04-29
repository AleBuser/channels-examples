use std::{thread, time};

mod channel_lite_writer;
mod channel_lite_reader;

fn main() {

    //create the channel
    let seed = "AUTHOR9SEED";
    let node = "https://nodes.devnet.iota.org:443";
    let mut channel_writer = channel_lite_writer::Channel::new(seed, node);
    
    // open the channel
    let announce_identifier = channel_writer.open().unwrap();
    println!("announce_identifier: {}", announce_identifier);

    // get channel address
    let channel_address: String = channel_writer.get_channel_address();
    println!("channel_address: {}", channel_address);

    // start writing to channel
    for _i in 0..10{
        let payload: &str = &format!("PAYLOAD");
        let signed_packet_identifier = channel_writer.write(&announce_identifier, payload, "").unwrap();
        println!("signed_packet_identifier: {}", signed_packet_identifier);

        thread::sleep(time::Duration::from_millis(5));
    }
/*
    let seed = "SUB9SEED";
    let node = "https://nodes.devnet.iota.org:443";
    let mut channel_reader: channel_lite_reader::Channel = channel_lite_reader::Channel::new(seed,node,channel_address);
    match channel_reader.connect("ICOTSLXXTKVXDNWFPG9LOFUQRJS"){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }


    match channel_reader.read(announce_identifier){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }
*/
}
