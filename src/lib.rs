use core::time::Duration;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::str::FromStr;

pub struct Sender {
    ip_address: Ipv4Addr,
    mask_address: Ipv4Addr
}

pub struct Receiver {
    ip_address: Ipv4Addr,
    port_number: u16
}

impl Sender {
    pub fn from(ip_address: &str, mask_address: &str) -> Self {
        let ip_result = Ipv4Addr::from_str(ip_address);
        let mask_result = Ipv4Addr::from_str(mask_address);

        if ip_result.is_err() {
            panic!("Invalid ip address");
        }

        if mask_result.is_err() {
            panic!("Invalid mask address");
        }

        Sender::new(ip_result.unwrap(), mask_result.unwrap())
    }

    pub fn new(ip_address: Ipv4Addr, mask_address: Ipv4Addr) -> Self {
        Sender { ip_address,mask_address }
    }

    pub fn send(&self, mac_address: &str, port_number: u16) -> Result<(), &str> {
        if let Some(datagram) = get_datagram_from_mac_address(mac_address) {
            let broadcast_address = get_broadcatst_address(self.ip_address.octets(), self.mask_address.octets());            
            let socket = UdpSocket::bind(SocketAddr::from((self.ip_address, 0))).map_err(|_|"Couldn't bind to address")?;
            
            match socket.send_to(&datagram, SocketAddrV4::new(broadcast_address, port_number)) {
                Err(_) => Err("Error while sending data"),
                _ => Ok(()),
            }
        } else {
            Err("Invalid mac address suplied")
        }
    }
}

impl Receiver {
    pub fn from(ip_address: &str, port_number: u16) -> Self {
        let ip_result = Ipv4Addr::from_str(ip_address);
        if ip_result.is_err() {
            panic!("Invalid ip address");
        }

        Receiver::new(ip_result.unwrap(), port_number)
    }

    pub fn new(ip_address: Ipv4Addr, port_number: u16) -> Self {
        Receiver { ip_address, port_number }
    }

    pub fn listen(&self, timeout: Option<Duration>) -> Result<Vec<u8>, &str> {
        let socket = UdpSocket::bind(SocketAddr::from((
            self.ip_address,
            self.port_number,
        )))
        .map_err(|_|"Couldn't bind to address")?;
        socket.set_read_timeout(timeout).map_err(|_|"Invalid timeout setting")?;

        let mut buf: [u8; 102] = [0; 102];
        match socket.recv_from(&mut buf) {
            Err(_) => Err("Error while listening for data"),
            _ => Ok(Vec::from(&buf[..]))
        }
    }
}

pub fn get_broadcatst_address(ip_address: [u8; 4], mask_address: [u8; 4]) -> Ipv4Addr {
    let mut broadcast_address: [u8; 4] = [0; 4];
    for i in 0..4 {
        broadcast_address[i] = ip_address[i] | (mask_address[i] ^ 255);
    }

    Ipv4Addr::from(broadcast_address)
}

pub fn get_datagram_from_mac_address(mac_address: &str) -> Option<[u8; 102]> {
    const COLON: char = ':';
    const HYPHEN: char = '-';
    const OFFSET: usize = 6;

    let split_by: char;
    if mac_address.contains(COLON) {
        split_by = COLON;
    } else if mac_address.contains(HYPHEN) {
        split_by = HYPHEN
    } else {
        return None;
    }

    let mac_segments: Vec<&str> = mac_address.split(split_by).collect();
    if mac_segments.len() != 6 {
        return None;
    }

    let mut datagram: [u8; 102] = [0; 102];
    for i in 0..OFFSET {
        datagram[i] = 0xff;
    }

    for i in 0..16 {
        for x in 0..OFFSET {
            let mac_segment_as_byte = u8::from_str_radix(mac_segments[x], 16);
            if mac_segment_as_byte.is_err() {
                return None;
            }

            datagram[OFFSET + (i * OFFSET) + x] = mac_segment_as_byte.ok().unwrap();
        }
    }

    Some(datagram)
}
