use wake_on_lan::*;
use core::time::Duration;

#[test]
fn receive_timeouts_return_error() {
    let receiver = Receiver::from("127.0.0.1", 3);
    let result = receiver.listen(Some(Duration::new(1, 0)));
    assert_eq!(result.is_err(), true);
}