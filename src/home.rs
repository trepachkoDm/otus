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

    pub fn get_rooms(&self) -> Vec<&String> {
        self.rooms.keys().collect()
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
        {
            let mut report = format!("{:?}", self.name);
            for (room_name, device_name) in self.rooms.iter() {
                for name in device_name {
                    if let Some(info) = info_provider.info(room_name, name) {
                        writeln!(report, "{}", info).unwrap();
                    }
                }
            }
            report
        }
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
        assert_eq!(home.get_rooms(), vec!["room_name"]);
    }

    #[test]
    fn test_get_device() {
        let home = SmartHouse::new("SM");
        let room = home.add_room("room_name", &["device_name"]);
        assert_eq!(room.devices("room"), None);
    }
}
