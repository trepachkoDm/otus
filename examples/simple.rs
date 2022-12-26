use smart_home::devices::{SmartSocket, SmartThermometer};
use smart_home::home::SmartHouse;
use smart_home::providers::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};

fn main() {
    let socket1 = SmartSocket::new("socket1");
    let socket2 = SmartSocket::new("socket2");
    let thermo1 = SmartThermometer::new("thermometer1");

    let house = SmartHouse::new("Smart House")
        .add_room("room 1", &["socket1"])
        .add_room("room 2", &["socket2", "thermometer1"]);
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

    let info_provider_2 = BorrowingDeviceInfoProvider {
        thermo: &thermo1,
        socket: &socket2,
    };

    let report1 = house.create_report(&info_provider_1);
    let report2 = house.create_report(&info_provider_2);
    println!("Report #1:\n {report1}");
    println!("Report #2:\n {report2}");
}
