use std::thread;
use std::time::Duration;
use clap::Parser;
use human_bytes::human_bytes;
use notify_rust::{Notification, NotificationHandle};
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

#[derive(Parser, Debug)]
#[command(name = "Low RAM notifier")]
#[command(author, about, version, long_about = None)]
struct Args {
    /// Amount of available RAM (in bytes) below which a notification will be given.
    warn_below: u64,

    /// Interval (in ms) to check the RAM status.
    #[arg(short, long, default_value_t = 500)]
    interval: u64,
}

fn main() {
    let args = Args::parse();

    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram())
    );

    test_notification(args.warn_below).expect("Failed to show notification");
    println!("Started, you should have seen a test notification");

    let mut notification_shown = false;
    loop {
        thread::sleep(Duration::from_millis(args.interval));
        sys.refresh_memory();

        if !notification_shown {
            if sys.available_memory() < args.warn_below {
                show_notification(sys.available_memory()).expect("Failed to show notification");
                notification_shown = true;
            }
        } else if sys.available_memory() >= args.warn_below {
            notification_shown = false;
        }
    }
}

fn show_notification(available: u64) -> notify_rust::error::Result<NotificationHandle> {
    Notification::new()
        .appname("Low RAM notifier")
        .summary("Memory low")
        .body(format!("Less than {} of RAM available!", human_bytes(available as f64)).as_str())
        .icon("device_mem")
        .show()
}

fn test_notification(warn_below: u64) -> notify_rust::error::Result<NotificationHandle> {
    Notification::new()
        .appname("Low RAM notifier")
        .summary("Started succesfully")
        .body(
            format!("You will get a notification like this if there is less than {} of RAM available",
                    human_bytes(warn_below as f64)).as_str()
        )
        .icon("device_mem")
        .show()
}
