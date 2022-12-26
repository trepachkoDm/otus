use crate::devices::{SmartSocket, SmartThermometer};

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}
pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

#[derive(Debug)]
pub enum DeviceError {
    SocketError(String),
    ThermoError(String),
}
#[derive(Debug)]
pub enum ProviderError {
    DeviceError(DeviceError),
    DeviceNotFound(String),
}

pub trait ErrDevice {
    fn state(&self) -> Result<String, DeviceError>;
}

impl ErrDevice for SmartSocket {
    fn state(&self) -> Result<String, DeviceError> {
        Ok(format!(
            "Device name is: {}, state is: {}, power is: {} A.",
            self.name, self.data, self.power
        ))
    }
}

impl ErrDevice for SmartThermometer {
    fn state(&self) -> Result<String, DeviceError> {
        Ok(format!(
            "Device name is: {}, state is: {}, temperature is: {} °C.",
            self.name, self.data, self.temperature
        ))
    }
}

pub trait DeviceInfoProvider {
    fn get_device_info(&self, device: &str) -> Result<String, ProviderError>;
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, device: &str) -> Result<String, ProviderError> {
        if self.socket.name == device {
            Ok(self.socket.state().expect("error socket"))
        } else {
            return Err(ProviderError::DeviceNotFound(device.to_string()));
        }
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn get_device_info(&self, device: &str) -> Result<String, ProviderError> {
        if self.socket.name == device {
            Ok(self.socket.state().expect("error socket"))
        } else if self.thermo.name == device {
            Ok(self.thermo.state().expect("error thermometr"))
        } else {
            return Err(ProviderError::DeviceNotFound(device.to_string()));
        }
    }
}
