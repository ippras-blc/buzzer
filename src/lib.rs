use esp_idf_svc::{
    hal::{
        delay::Delay,
        gpio::OutputPin,
        ledc::{config::TimerConfig, LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver},
        peripheral::Peripheral,
        prelude::*,
    },
    sys::EspError,
};

/// Buzzer
pub struct Buzzer<'a> {
    driver: LedcDriver<'a>,
    delay: Delay,
}

impl<'a> Buzzer<'a> {
    pub fn new<T: LedcTimer + 'a>(
        channel: impl Peripheral<P = impl LedcChannel<SpeedMode = <T as LedcTimer>::SpeedMode>> + 'a,
        timer: impl Peripheral<P = T> + 'a,
        pin: impl Peripheral<P = impl OutputPin> + 'a,
    ) -> Result<Self, EspError> {
        let timer_driver =
            LedcTimerDriver::new(timer, &TimerConfig::default().frequency(500.Hz()))?;
        let driver = LedcDriver::new(channel, timer_driver, pin)?;
        Ok(Self {
            driver,
            delay: Delay::new_default(),
        })
    }

    pub fn alarm(&mut self, count: u8) -> Result<(), EspError> {
        for _ in 0..count {
            self.driver.enable()?;
            self.driver.set_duty(self.driver.get_max_duty())?;
            self.delay.delay_ms(900);
            self.driver.disable()?;
            self.delay.delay_ms(100);
        }
        Ok(())
    }
}
