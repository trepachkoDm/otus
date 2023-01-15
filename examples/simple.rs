use smart_home::devices::{SmartSocket, SmartThermometer};
use smart_home::home::{Room, SmartHouse};
use std::rc::Rc;

fn main() {
    let socket1 = Rc::new(SmartSocket::new("Socket 1"));
    let socket2 = Rc::new(SmartSocket::new("Socket 2"));
    let thermo1 = Rc::new(SmartThermometer::new("Thermometer 1"));

    let mut house = SmartHouse::new("Smart House")
        .add_room(Room::new("Room 1").add_device(socket1))
        .add_room(Room::new("Room 2").add_device(socket2))
        .add_room(Room::new("Room 3").add_device(thermo1));

    let report = house.create_report();
    println!("Report #Add:\n {report}");

    house.remove_room("Room 1");
    let report_remove = house.create_report();
    println!("Report #Remove:\n {report_remove}");

    house.get_room_list();
    let report_get_room_list = house.create_report();
    println!("Report #Get room list:\n {report_get_room_list}");

    house.get_room_device_list("Room 2");
    let report_get_room_device_list = house.create_report();
    println!("Report #Device list:\n {report_get_room_device_list}");
}
