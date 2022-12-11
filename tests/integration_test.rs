use smart_home::providers::OwningDeviceInfoProvider;
use smart_home::{devices::SmartSocket, home::SmartHouse};

#[test]
fn integration_test1() {
    let home = SmartHouse::new("Smart Home");
    let room = home.add_room("room 1", &["socket 1"]);
    let socket1 = SmartSocket::new("socket1");
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report = room.create_report(&info_provider_1);
    assert!(report.contains("room 1"));
}
