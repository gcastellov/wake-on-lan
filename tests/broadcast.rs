use std::net::Ipv4Addr;
use wake_on_lan::*;

#[test]
fn gets_proper_broadcast_address() {
    let localhost_ip = Ipv4Addr::LOCALHOST;
    let localhost_mask = Ipv4Addr::new(255, 0, 0, 0);

    let broadcast_address = get_broadcatst_address(localhost_ip.octets(), localhost_mask.octets());
    let expected_address = Ipv4Addr::new(127, 255, 255, 255);
    assert_eq!(broadcast_address, expected_address);
}

#[test]
fn invalid_mac_address_1() {
    let result = get_datagram_from_mac_address("123424234542345");
    assert_eq!(result.is_none(), true);
}

#[test]
fn invalid_mac_address_2() {
    let result = get_datagram_from_mac_address("66-CD-E1-KK-7A-F4");
    assert_eq!(result.is_none(), true);
}

#[test]
fn invalid_mac_address_3() {
    let result = get_datagram_from_mac_address("66;CD;E1;38;7A;F4");
    assert_eq!(result.is_none(), true);
}

#[test]
fn invalid_mac_address_4() {
    let result = get_datagram_from_mac_address("0E-EB-06-54-96");
    assert_eq!(result.is_none(), true);
}

#[test]
fn invalid_mac_address_5() {
    let result = get_datagram_from_mac_address("91:C7:37:5D:44");
    assert_eq!(result.is_none(), true);
}

#[test]
fn valid_mac_address_1() {
    let result = get_datagram_from_mac_address("F2-0C-F8-10-01-F7");
    assert_eq!(result.is_some(), true);
}

#[test]
fn valid_mac_address_2() {
    let result = get_datagram_from_mac_address("F2:0C:F8:10:01:F7");
    assert_eq!(result.is_some(), true);
}
