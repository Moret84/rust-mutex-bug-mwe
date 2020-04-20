This is a minimal reproductible example of a bug encountered in rust.
[Associated topic on rust lang forum](https://users.rust-lang.org/t/different-behaviour-when-chaining-vs-using-intermediate-value/41293)

This code mainly tries to make a ble scan through bluez dbus api.
It uses [dbus-rs](https://github.com/diwic/dbus-rs) crate to manage dbus communication.

Steps to reproduces:

in src/dbus_ble/dbus_ble_repo.rs, line 47:

The following code *does launch the scan*:
```
    pub fn start_scan(&self) {
        let connection = self.dbus_connection.lock().unwrap();
        connection
            .with_proxy(BLUEZ_DBUS_DESTINATION, "/org/bluez/hci0", STANDARD_TIMEOUT)
            .start_discovery().expect("Error starting discovery");
    }
```

The following code does *not* launch the scan:
```
    pub fn start_scan(&self) {
        self.dbus_connection.lock().unwrap()
            .with_proxy(BLUEZ_DBUS_DESTINATION, "/org/bluez/hci0", STANDARD_TIMEOUT)
            .start_discovery().expect("Error starting discovery");
    }
```

The code in src/dbus_ble/bluez_dbus.rs has been generated using dbus-codegen tool provided by dbus-rs crate author.
It has been slightly modified for convenience (Replaced Connection with SyncConnection).
