use wake_on_lan::*;

#[test]
#[ignore]
fn send_to_mac_and_port() {
    let sender: Sender = Sender::from("127.0.0.1", "255.0.0.0");
    let result = sender.send("F2:0C:F8:10:01:F7", 3);
    assert_eq!(result.is_ok(), true);
}
