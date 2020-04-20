// This code was autogenerated with `dbus-codegen-rust -s -g -m None -d org.bluez -p /org/bluez/hci0`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
use dbus::arg;
use dbus::blocking;

/// ADAPTER
pub trait OrgBluezAdapter1 {
    fn start_discovery(&self) -> Result<(), dbus::Error>;
    fn stop_discovery(&self) -> Result<(), dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::SyncConnection>> OrgBluezAdapter1 for blocking::Proxy<'a, C> {

    fn start_discovery(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "StartDiscovery", ())
    }

    fn stop_discovery(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "StopDiscovery", ())
    }
}

/// OBJECT MANAGER
#[derive(Debug)]
pub struct OrgFreedesktopDBusObjectManagerInterfacesAdded {
    pub object: dbus::Path<'static>,
    pub interfaces: ::std::collections::HashMap<String, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>,
}

impl arg::AppendAll for OrgFreedesktopDBusObjectManagerInterfacesAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.object, i);
        arg::RefArg::append(&self.interfaces, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusObjectManagerInterfacesAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusObjectManagerInterfacesAdded {
            object: i.read()?,
            interfaces: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusObjectManagerInterfacesAdded {
    const NAME: &'static str = "InterfacesAdded";
    const INTERFACE: &'static str = "org.freedesktop.DBus.ObjectManager";
}
