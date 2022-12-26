use crate::providers::DeviceInfoProvider;
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, PartialEq, Eq)]
pub struct SmartHouse {
    pub name: String,
    pub rooms: HashMap<String, Vec<String>>,
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            rooms: HashMap::new(),
        }
    }

    pub fn get_rooms(&self) -> Option<Vec<&String>> {
        match self.rooms.len() {
            0 => None,
            _ => Some(self.rooms.keys().collect()),
        }
    }

    pub fn devices(&self, room: &str) -> Option<&Vec<String>> {
        self.rooms.get(room)
    }

    pub fn add_room(mut self, room_name: &str, device_name: &[&str]) -> Self {
        let device_list = device_name.iter().map(|d| d.to_string()).collect();
        self.rooms.insert(room_name.into(), device_list);
        self
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, info_provider: &T) -> String {
        let mut report = format!("");
        for (room_name, device) in self.rooms.iter() {
            for device_name in device {
                write!(report, "{} - Device {} : ", room_name, device_name).unwrap();
                match info_provider.get_device_info(device_name) {
                    Ok(s) => writeln!(report, "{}", s).unwrap(),
                    Err(e) => writeln!(report, "{:?}", e).unwrap(),
                };
            }
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fn_new() {
        let home = SmartHouse::new("SM");
        assert_eq!(home.name, "SM");
        assert_eq!(home.rooms, HashMap::new());
    }

    #[test]
    fn test_get_rooms() {
        let home = SmartHouse::new("SM").add_room("room_name", &["device_name"]);
        assert_ne!(home.get_rooms(), None);
    }

    #[test]
    fn test_get_device() {
        let home = SmartHouse::new("SM");
        let room = home.add_room("room_name", &["device_name"]);
        assert_eq!(room.devices("room"), None);
    }
}
