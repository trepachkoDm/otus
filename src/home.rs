use crate::providers::Device;
use std::collections::HashMap;
use std::fmt::Write;
use std::rc::Rc;

pub struct SmartHouse {
    pub name: String,
    pub rooms: HashMap<String, Room>,
}

pub struct Room {
    name: String,
    devices: HashMap<String, Rc<dyn Device>>,
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(mut self, room: Room) -> Self {
        let room_name = room.name.clone();
        self.rooms.insert(room_name, room);
        self
    }

    pub fn remove_room(&mut self, name: &str) -> Option<Room> {
        self.rooms.remove(name)
    }

    pub fn get_room_list(&self) -> Vec<String> {
        self.rooms.keys().cloned().collect()
    }

    pub fn get_room_device_list(&self, room_name: &str) -> Option<Vec<String>> {
        self.rooms.get(room_name).map(|room| room.get_device_list())
    }

    pub fn create_report(&self) -> String {
        let mut report = format!("{} ", self.name);
        for (room_name, room) in self.rooms.iter() {
            for (_device_name, device) in room.devices.iter() {
                write!(report, "{}", room_name).unwrap();
                match device.state() {
                    Ok(ok) => writeln!(report, "{}", ok).unwrap(),
                    Err(e) => writeln!(report, "{:?}", e).unwrap(),
                };
            }
        }
        report
    }
}

impl Room {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            devices: HashMap::new(),
        }
    }

    pub fn add_device(mut self, device: Rc<dyn Device>) -> Self {
        self.devices.insert(device.get_name(), device);
        self
    }

    pub fn remove_device(&mut self, name: &str) -> Option<Rc<dyn Device>> {
        self.devices.remove(name)
    }

    pub fn get_device_list(&self) -> Vec<String> {
        self.devices.keys().cloned().collect()
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_room_new() {
        let home = SmartHouse::new("SM");
        assert_eq!(home.name, "SM");
    }

    #[test]
    fn test_add_room() {
        let home = SmartHouse::new("SM").add_room(Room::new("Room # 1"));
        assert_eq!(home.rooms.len(), 1);
    }

    #[test]
    fn test_remove_room() {
        let mut home = SmartHouse::new("SM")
            .add_room(Room::new("Room # 1"))
            .add_room(Room::new("Room # 2"));
        assert!(home.remove_room("Room # 1").is_some());
        assert_eq!(home.rooms.len(), 1);
    }

    #[test]
    fn test_get_room_list() {
        let home = SmartHouse::new("SM")
            .add_room(Room::new("Room # 1"))
            .add_room(Room::new("Room # 2"));
        let mut room_list = home.get_room_list();
        room_list.sort();
        assert_eq!(room_list, vec!["Room # 1", "Room # 2"]);
    }

    use crate::devices::SmartSocket;
    #[test]
    fn test_device_new() {
        let device = Rc::new(SmartSocket::new("Socket 1"));
        assert_eq!(device.name, "Socket 1");
    }

    #[test]
    fn test_add_device() {
        let device = Rc::new(SmartSocket::new("Socket 1"));
        let room = Room::new("Room # 1").add_device(device);
        assert_eq!(room.devices.len(), 1);
    }

    #[test]
    fn test_remove_device() {
        let device1 = Rc::new(SmartSocket::new("Socket 1"));
        let device2 = Rc::new(SmartSocket::new("Socket 2"));
        let mut room = Room::new("room_1").add_device(device1).add_device(device2);
        assert!(room.remove_device("Socket 1").is_some());
        assert_eq!(room.devices.len(), 1);
    }

    #[test]
    fn test_get_device_list() {
        let device1 = Rc::new(SmartSocket::new("Socket 1"));
        let device2 = Rc::new(SmartSocket::new("Socket 2"));
        let room = Room::new("room_1").add_device(device1).add_device(device2);
        let mut device_list = room.get_device_list();
        device_list.sort();
        assert_eq!(device_list, vec!["Socket 1", "Socket 2"]);
    }
}
