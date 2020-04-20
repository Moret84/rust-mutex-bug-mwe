use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

use dbus::blocking::SyncConnection;
use dbus::blocking::stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged;
use dbus::message::{MatchRule, MessageType, Message};
use dbus::strings::{Interface, Member};

use crate::dbus_ble::bluez_dbus::{OrgBluezAdapter1, OrgFreedesktopDBusObjectManagerInterfacesAdded};

const BLUEZ_DBUS_DESTINATION: &str = "org.bluez";
const BLUEZ_DBUS_DEVICE_INTERFACE: &str = "org.bluez.Device1";
const DBUS_CONNECTION_TIMEOUT_MS: u64 = 5000;

static STANDARD_TIMEOUT: Duration = Duration::from_millis(DBUS_CONNECTION_TIMEOUT_MS);

pub struct DbusBleRepo {
    dbus_connection: Arc<Mutex<SyncConnection>>,
}

impl DbusBleRepo {
    /// Return a new instance of a Dbus ble repo.
    pub fn new() -> DbusBleRepo {

        let connection = SyncConnection::new_system().expect("Error getting dbus connection");

        let dbus_ble_repo = DbusBleRepo {
            dbus_connection: Arc::new(Mutex::new(connection)),
        };

        dbus_ble_repo.add_interface_added_match_rule();
        dbus_ble_repo.add_properties_changed_match_rule();

        thread::spawn({
            let connection = dbus_ble_repo.dbus_connection.clone();
            move || {
            loop {
                connection.lock().unwrap().process(Duration::from_secs(1));
                thread::sleep(Duration::from_secs(1));
            }
        }});

        dbus_ble_repo
    }

    pub fn start_scan(&self) {
        let connection = self.dbus_connection.lock().unwrap();
        //self.dbus_connection.lock().unwrap()
        connection
            .with_proxy(BLUEZ_DBUS_DESTINATION, "/org/bluez/hci0", STANDARD_TIMEOUT)
            .start_discovery().expect("Error starting discovery");
    }

    pub fn stop_scan(&self) {
        self.dbus_connection.lock().unwrap()
            .with_proxy(BLUEZ_DBUS_DESTINATION, "/org/bluez/hci0", STANDARD_TIMEOUT)
            .stop_discovery().expect("Error stopping discovery");
    }

    fn add_interface_added_match_rule(&self) {
        let mut interface_added_match_rule = MatchRule::new();
        interface_added_match_rule.interface = Option::Some(Interface::new("org.freedesktop.DBus.ObjectManager").unwrap());
        interface_added_match_rule.msg_type = Option::Some(MessageType::Signal);
        interface_added_match_rule.member = Option::Some(Member::new("InterfacesAdded").unwrap());

        let on_interface_added = {
            move | p: OrgFreedesktopDBusObjectManagerInterfacesAdded, _: &SyncConnection, m: &Message| {
                // If this is a ble device which has been discovered
                if p.interfaces.contains_key(BLUEZ_DBUS_DEVICE_INTERFACE) {

                    println!("{:#?}", m);
                }
            true
            }
        };

        self.dbus_connection.lock().unwrap().add_match(interface_added_match_rule, on_interface_added).unwrap();
    }

    fn add_properties_changed_match_rule(&self) {
        let mut properties_changed_match_rule = MatchRule::new();
        properties_changed_match_rule.interface = Option::Some(Interface::new("org.freedesktop.DBus.Properties").unwrap());
        properties_changed_match_rule.msg_type = Option::Some(MessageType::Signal);
        properties_changed_match_rule.member = Option::Some(Member::new("PropertiesChanged").unwrap());

        let on_properties_changed = {
            move | p: PropertiesPropertiesChanged, _: &SyncConnection, m: &Message | {
                if p.interface_name == BLUEZ_DBUS_DEVICE_INTERFACE {
                    println!("{:#?}", m);
                }
                true
            }
        };

        self.dbus_connection.lock().unwrap().add_match(properties_changed_match_rule, on_properties_changed).unwrap();
    }
}
