use ws::listen;

const PROTOCOL_VERSION: u8 = 1;
const SERVER_VERSION: &str = "0.1";
const SERVER_NAME: &str = "hogera";
const SERVER_TYPE: [u8; 2] = [0x51, 0xED];

const SET_LED: u8 = 0x10;
const INITIALIZE: u8 = 0x11;
const PING: u8 = 0x12;
const REQUEST_SERVER_INFO: u8 = 0xD0;
// const : u8 = 0x10;
const READY: u8 = 0x19;
const PONG: u8 = 0x1A;
const REPORT_SERVER_INFO: u8 = 0xD8;

const SERVER_INFO_LENGE: usize = 44;

fn set_led(payload: &Vec<u8>) {
    // return;
    let mut boder_indexes = (49..94usize).step_by(3);
    print!("\r");
    print!("{:<3}", payload[0]);
    for i in (1..49).step_by(3) {
        print!(
            "\x1b[48;2;{};{};{}m \x1b[m",
            payload[i],
            payload[i + 1],
            payload[i + 2]
        );
        let boder_index = match boder_indexes.next() {
            Some(v) => v,
            None => continue,
        };
        print!(
            "\x1b[38;2;{};{};{}m|\x1b[m",
            payload[boder_index],
            payload[boder_index + 1],
            payload[boder_index + 2]
        );
    }
    print!("           ");
}

fn set_name(server_info: &mut [u8; SERVER_INFO_LENGE], name: String, offset: usize) {
    if name.len() > 16 {
        panic!("Maximum name length is 16")
    }
    let mut iter = name.chars();
    for i in 0..17 {
        let cher = match iter.next() {
            Some(v) => v as u8,
            None => break,
        };
        if cher > 127 {
            panic!("Only ASCII codes can be used for name");
        }
        server_info[i + offset] = cher;
    }
}

fn set_version(server_info: &mut [u8; SERVER_INFO_LENGE], version: String, offset: usize) {
    let versions: Vec<&str> = version.split('.').collect();
    if versions.len() != 2 {
        panic!("not allow micro version");
    }
    for (i, cher) in (0..4).step_by(2).zip(versions) {
        let v: u16 = match cher.parse() {
            Ok(v) => v,
            Err(_) => 0,
        };
        server_info[offset + i] = (v & 0xff) as u8;
        server_info[offset + i + 1] = (v >> 8) as u8;
    }
}
fn main() {
    let mut server_info = [0; SERVER_INFO_LENGE];
    set_name(&mut server_info, SERVER_NAME.to_string(), 0);
    set_name(&mut server_info, "foo".to_string(), 22);
    set_version(&mut server_info, SERVER_VERSION.to_string(), 18);
    set_version(&mut server_info, "0.0".to_string(), 22);
    let server_info = server_info;

    listen("127.0.0.1:50000", |out| {
        move |msg: ws::Message| {
            let msg = msg.into_data();
            let mut response = vec![PROTOCOL_VERSION];
            let command = msg[1];
            // println!("{:?}", msg);
            let need_response = match command {
                PING => {
                    println!("ping");
                    response.extend([PONG, 6]);
                    response.append(&mut msg[3..7].to_vec());
                    response.extend(SERVER_TYPE);
                    true
                }
                INITIALIZE => {
                    println!("initialize");
                    response.extend([READY, 0]);
                    true
                }
                SET_LED => {
                    set_led(&msg[3..].to_vec());
                    false
                }
                REQUEST_SERVER_INFO => {
                    response.extend([REPORT_SERVER_INFO, 44]);
                    response.extend(server_info);
                    true
                }
                _ => false,
            };
            if need_response {
                // println!("{:?}", response);
                let response = ws::Message::binary(response);
                out.send(response)
            } else {
                Ok(())
            }
        }
    })
    .unwrap()
}
