pub struct SmartSocket {
    pub name: String,
    pub data: bool,
    pub power: i32,
}
pub struct SmartThermometer {
    pub name: String,
    pub data: bool,
    pub temperature: i32,
}

impl SmartSocket {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            data: true,
            power: 1,
        }
    }
}

impl SmartThermometer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            data: true,
            temperature: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_structs_socket() {
        let socket = SmartSocket::new("Socket 1");
        assert_eq!(socket.name, "Socket 1");
        assert_eq!(socket.power, 1);
    }
    #[test]
    fn test_structs_thermometer() {
        let thermometer = SmartThermometer::new("Thermometer 1");
        assert_eq!(thermometer.name, "Thermometer 1");
        assert_eq!(thermometer.temperature, 1);
    }
}
