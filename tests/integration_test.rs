use smart_home::devices::{SmartSocket, SmartThermometer};
use smart_home::home::{Room, SmartHouse};
use std::rc::Rc;

#[test]
fn integration_test1() {
    let socket1 = Rc::new(SmartSocket::new("Socket 1"));
    let socket2 = Rc::new(SmartSocket::new("Socket 2"));
    let thermo1 = Rc::new(SmartThermometer::new("Thermometer 1"));

    let house = SmartHouse::new("Smart House")
        .add_room(Room::new("Room 1").add_device(socket1))
        .add_room(Room::new("Room 2").add_device(socket2))
        .add_room(Room::new("Room 3").add_device(thermo1));
    let report = house.create_report();

    assert!(report.contains("Room 1"));
    assert!(report.contains("Room 2"));
    assert!(report.contains("Room 3"));
}
