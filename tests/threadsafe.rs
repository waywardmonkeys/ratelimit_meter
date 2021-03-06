extern crate ratelimit_meter;

use ratelimit_meter::{GCRA, Decider, Decision};
use std::thread;
use std::time::{Instant, Duration};

#[test]
fn simple_operation() {
    let mut lim = GCRA::for_capacity(5).unwrap().build_sync();
    assert_eq!(Decision::Yes, lim.check().unwrap());
}

#[test]
fn actual_threadsafety() {
    let mut lim = GCRA::for_capacity(20).unwrap().build_sync();
    let now = Instant::now();
    let ms = Duration::from_millis(1);
    let mut children = vec![];

    lim.check_at(now).unwrap();
    for _i in 0..20 {
        let mut lim = lim.clone();
        children.push(thread::spawn(move || {
            lim.check_at(now).unwrap();
        }));
    }
    for child in children {
        child.join().unwrap();
    }
    assert!(!lim.check_at(now).unwrap().is_compliant());
    assert_eq!(Decision::Yes, lim.check_at(now+ms*1000).unwrap());
}
