# wake-on-lan
RUST implementation for WOL (Wake On Lan)

## Using sender
The sender object will send the magic packet like:

```
let sender: Sender = Sender::from("127.0.0.1", "255.0.0.0");
let _ = sender.send("00:00:00:00:00:01", 3);
```

## Using receiver
The receiver is only used to prove the magic packet is sent via broadcasting UDP. 
Maybe useful for some testing implementations.

```
let receiver = Receiver::from("127.0.0.1", 3);
receiver.listen(None);
```

## Send and receive
As shown in the tests.

```
#[test]
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
```
