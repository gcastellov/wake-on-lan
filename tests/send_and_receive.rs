use wake_on_lan::*;
use core::time::Duration;

#[test]
#[ignore]
fn send_and_receive_successfully() {
    let receiver = Receiver::from("127.0.0.1", 3);
    let sender: Sender = Sender::from("127.0.0.1", "255.0.0.0");

    let receiver_handle = std::thread::spawn(move || {
        let result = receiver.listen(Some(Duration::new(10, 0)));
        if result.is_ok() {
            println!("{:?}", result.ok());
        }
    });

    let _ = sender.send("00:00:00:00:00:01", 3);
    receiver_handle.join().unwrap();
}