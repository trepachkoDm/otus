use smart_home::devices::{SmartSocket, SmartThermometer};
use smart_home::home::SmartHouse;
use smart_home::providers::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};

fn main() {
    let socket1 = SmartSocket::new("socket 1");
    let socket2 = SmartSocket::new("socket 2");
    let thermo = SmartThermometer::new("thermometer1");

    let house = SmartHouse::new("Smart House")
        .add_room("room 1", &["socket 1"])
        .add_room("room 2", &["socket 2", "thermo 1"]);
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };

    let report1 = house.create_report(&info_provider_1);
    let report2 = house.create_report(&info_provider_2);
    println!("Report #1:\n {report1}");
    println!("Report #2:\n {report2}");
}
