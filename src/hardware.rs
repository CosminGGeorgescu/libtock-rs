use crate::adc::AdcDriver;
use crate::buttons::ButtonDriver;
use crate::console::ConsoleDriver;
use crate::gpio::GpioDriver;
use crate::led::LedDriver;
use crate::result::OtherError;
use crate::result::TockError;
use crate::result::TockResult;
use crate::rng::RngDriver;
use crate::sensors::ninedof::NinedofDriver;
use crate::sensors::AmbientLightSensor;
use crate::sensors::HumiditySensor;
use crate::sensors::TemperatureSensor;
use crate::simple_ble::BleAdvertisingDriver;
use crate::simple_ble::BleScanningDriver;
use crate::temperature::TemperatureDriver;
use crate::timer::DriverContext;
use core::cell::Cell;

/// Struct containing all drivers constructible through retrieve_hardware()
pub struct Hardware {
    pub console_driver: ConsoleDriver,
    pub led_driver: LedDriver,
    pub timer_context: DriverContext,
    pub gpio_driver: GpioDriver,
    pub temperature_driver: TemperatureDriver,
    pub button_driver: ButtonDriver,
    pub adc_driver: AdcDriver,
    pub rng_driver: RngDriver,
    pub ble_advertising_driver: BleAdvertisingDriver,
    pub ble_scanning_driver: BleScanningDriver,
    pub ambient_light_sensor: AmbientLightSensor,
    pub temperature_sensor: TemperatureSensor,
    pub humidity_sensor: HumiditySensor,
    pub ninedof_driver: NinedofDriver,
}

/// Retrieve Hardware struct. Returns Hardware only once.
pub fn retrieve_hardware() -> TockResult<Hardware> {
    match unsafe { HARDWARE_SINGLETON.take() } {
        Some(hardware) => Ok(hardware),
        None => Err(TockError::Other(OtherError::DriverAlreadyTaken)),
    }
}

/// Retrieve [Hardware] struct without check whether it has already been taken
/// at a different point.
/// # Safety
/// This shall only used in special situations where drivers cannot be passed as arguments
/// as in the panic handler. Otherwise global mutable state (as shared buffers) may be exposed
/// in an unsafe manner.
pub unsafe fn retrieve_hardware_unsafe() -> Hardware {
    HARDWARE
}

#[allow(clippy::declare_interior_mutable_const)]
const HARDWARE: Hardware = Hardware {
    console_driver: ConsoleDriver {
        _unconstructible: (),
    },
    led_driver: LedDriver {
        _unconstructible: (),
    },
    timer_context: DriverContext {
        active_timer: Cell::new(None),
    },
    gpio_driver: GpioDriver {
        _unconstructible: (),
    },
    temperature_driver: TemperatureDriver {
        _unconstructible: (),
    },
    button_driver: ButtonDriver {
        _unconstructible: (),
    },
    adc_driver: AdcDriver {
        _unconstructible: (),
    },
    rng_driver: RngDriver {
        _unconstructible: (),
    },
    ble_advertising_driver: BleAdvertisingDriver {
        _unconstructible: (),
    },
    ble_scanning_driver: BleScanningDriver {
        _unconstructible: (),
    },
    ambient_light_sensor: AmbientLightSensor {
        _unconstructible: (),
    },
    temperature_sensor: TemperatureSensor {
        _unconstructible: (),
    },
    humidity_sensor: HumiditySensor {
        _unconstructible: (),
    },
    ninedof_driver: NinedofDriver {
        _unconstructible: (),
    },
};

static mut HARDWARE_SINGLETON: Option<Hardware> = Some(HARDWARE);

#[cfg(test)]
mod test {
    use crate::retrieve_hardware;
    #[test]
    pub fn can_be_retrieved_once() {
        reset_hardware_singleton();

        assert!(retrieve_hardware().is_ok());
        assert!(retrieve_hardware().is_err());
    }

    fn reset_hardware_singleton() {
        unsafe {
            super::HARDWARE_SINGLETON = Some(super::HARDWARE);
        };
    }
}
