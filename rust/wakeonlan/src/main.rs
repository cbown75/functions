use std::env;
extern crate wake_on_lan;

fn main() {
    let args: Vec<String> = env::args().collect();

    for i in 0..args.len() {
        println!("{}", args[i]);

        let mac_address: [u8; 6] = [0x0F, 0x1E, 0x2D, 0x3C, 0x4B, 0x5A];
        let magic_packet = wake_on_lan::MagicPacket::new(&mac_address);
        let result = magic_packet.send();

        println!("{:?}", result);
    }
}
