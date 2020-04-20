mod dbus_ble;
use dbus_ble::dbus_ble_repo::DbusBleRepo;
use std::time::Duration;

use std::thread;

fn main() {
    let dbus_ble_repo = DbusBleRepo::new();

    dbus_ble_repo.start_scan();

    let mut i = 30;
    while i >= 0 {
        thread::sleep(Duration::from_secs(1));
        i = i -1;
    }

    dbus_ble_repo.stop_scan();
}
