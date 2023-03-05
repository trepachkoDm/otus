use crate::devices::{SmartSocket, SmartThermometer};
use thiserror::Error;

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}
pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Socket error: {0}")]
    SocketError(String),
    #[error("Thermometr error: {0}")]
    ThermoError(String),
}

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Device error: {0}")]
    DeviceError(#[from] DeviceError),
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
}

pub trait Device {
    fn state(&self) -> Result<String, DeviceError>;
    fn get_name(&self) -> String;
}

impl Device for SmartSocket {
    fn state(&self) -> Result<String, DeviceError> {
        Ok(format!(
            " Device name is: {}, state is: {}, power is: {} A.",
            self.name, self.data, self.power
        ))
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Device for SmartThermometer {
    fn state(&self) -> Result<String, DeviceError> {
        Ok(format!(
            " Device name is: {}, state is: {}, temperature is: {} °C.",
            self.name, self.data, self.temperature
        ))
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub trait DeviceInfoProvider {
    fn get_device_info(&self, device: &str) -> Result<String, ProviderError>;
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, device: &str) -> Result<String, ProviderError> {
        if self.socket.name == device {
            Ok(self.socket.state()?)
        } else {
            Err(ProviderError::DeviceNotFound(device.to_string()))
        }
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn get_device_info(&self, device: &str) -> Result<String, ProviderError> {
        if self.socket.name == device {
            Ok(self.socket.state()?)
        } else if self.thermo.name == device {
            Ok(self.thermo.state()?)
        } else {
            Err(ProviderError::DeviceNotFound(device.to_string()))
        }
    }
}
