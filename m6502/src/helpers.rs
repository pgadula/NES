pub fn hex_dump(buff: &[u8]) {
    for (i, chunk) in buff.chunks(16).enumerate() {
        print!("{:08X}  ", i * 16);
        for byte in chunk {
            print!("{:02X} ", byte);
        }

        print!(" |");
        for byte in chunk {
            let ch = *byte as char;
            print!("{}", if ch.is_ascii_graphic() { ch } else { '.' });
        }
        println!("|");
    }
}

