use bech32::u5;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <address> <new_prefix>", args[0]);
        return;
    }

    let address = &args[1];
    let new_prefix = &args[2];
    if let Some(address) = address.strip_prefix("0x") {
        if new_prefix != "evmos" {
            println!("Cannot convert to non-evmos address from ethereum address");
            return;
        }
        let data = hex::decode(address).unwrap();
        let encoded = bech32::encode(new_prefix, encode_to_u5(data), bech32::Variant::Bech32).unwrap();
        println!("{}", encoded);
        return;
    }
    if new_prefix == "evmos" {
        println!("To convert to evmos address, input ethereum address");
        return;
    }
    let data = bech32::decode(address).unwrap();
    let encoded = bech32::encode(new_prefix, data.1, data.2).unwrap();

    println!("{}", encoded);
}

fn encode_to_u5(data: Vec<u8>) -> Vec<u5> {
    let mut result: Vec<u5> = Vec::with_capacity((data.len() + 3) / 4 * 5);
    for chunk in data.chunks(5) {
        let buf = {
            let mut buf = [0u8; 5];
            buf[..chunk.len()].copy_from_slice(chunk);
            buf
        };
        result.push(u5::try_from_u8((buf[0] & 0xF8) >> 3).unwrap());
        result.push(u5::try_from_u8(((buf[0] & 0x07) << 2) | ((buf[1] & 0xC0) >> 6)).unwrap());
        result.push(u5::try_from_u8((buf[1] & 0x3E) >> 1).unwrap());
        result.push(u5::try_from_u8(((buf[1] & 0x01) << 4) | ((buf[2] & 0xF0) >> 4)).unwrap());
        result.push(u5::try_from_u8(((buf[2] & 0x0F) << 1) | (buf[3] >> 7)).unwrap());
        result.push(u5::try_from_u8((buf[3] & 0x7C) >> 2).unwrap());
        result.push(u5::try_from_u8(((buf[3] & 0x03) << 3) | ((buf[4] & 0xE0) >> 5)).unwrap());
        result.push(u5::try_from_u8(buf[4] & 0x1F).unwrap());
    }
    result
}
