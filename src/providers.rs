use crate::devices::{SmartSocket, SmartThermometer};
use std::fmt::Write;

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}
pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

pub trait DeviceInfoProvider {
    fn info(&self, room_name: &str, device_name: &str) -> Option<String>;
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn info(&self, room_name: &str, device_name: &str) -> Option<String> {
        let mut info_data = format!(" \n{:?} ", room_name);
        if self.socket.name == device_name {
            write!(info_data, " - {:?}", self.socket).unwrap();
        } else {
            write!(info_data, " - Device: <{}> - not found !", device_name).unwrap();
        }
        Some(info_data)
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn info(&self, room_name: &str, device_name: &str) -> Option<String> {
        let mut info_data = format!(" \n{:?} ", room_name);
        if self.socket.name == device_name {
            write!(info_data, " - {:?}", self.socket).unwrap();
        } else if self.thermo.name == device_name {
            write!(info_data, " - {:?}", self.thermo).unwrap();
        } else {
            write!(info_data, " - Device: <{}> - not found !", device_name).unwrap();
        }
        Some(info_data)
    }
}
