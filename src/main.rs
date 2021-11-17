#![allow(incomplete_features)]
#![feature(unsized_locals, unsized_fn_params)]

use std::env;
use commander;
use std::thread;
use std::time::Duration;
use win32_notification::NotificationBuilder;
struct notif_config {
    title: &str
    text: &str
}

fn notify(config: notif_config) {
    let notification = NotificationBuilder::new()
    .title_text(config.title)
    .info_text(config.text)
    .build()
    .expect("Could not create notification");
    notification.show().expect("Failed to show notification");
    thread::sleep(Duration::from_secs(5));
    notification
        .delete()
        .expect("Failed to delete notification");
}
impl notif_config {
    fn new(&[String]) -> notif_config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let tit = args[1].clone();
        let txt = args[2].clone();
        notif_config {&tit, &txt}
    }
    
}
fn main() {
    let args: Vec<String> = env::args().collect();
cfg: notif_config = parse_args(&args);
    //    let tit: &str = &args[1];
//    let txt: &str = &args[2];
    
    notify(&cfg);
}
