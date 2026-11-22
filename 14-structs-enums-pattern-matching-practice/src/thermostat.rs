#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Heating,
    Cooling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Off,
    Idle,
    Active,
}

#[derive(Debug)]
pub struct Thermostat {
    mode: Mode,
    heat_setpoint: i32,
    cool_setpoint: i32,
    hysteresis: u8,
    current_temperature: Option<i32>,
    status: Status,
    activation_count: u64,
}

impl Thermostat {
    pub fn new(mode: Mode, heat_setpoint: i32, cool_setpoint: i32, hysteresis: u8) -> Self {
        Self {
            mode,
            heat_setpoint,
            cool_setpoint,
            hysteresis,
            current_temperature: None,
            status: Status::Off,
            activation_count: 0,
        }
    }

    pub fn update_temperature(&mut self, temperature: i32) {
        if self.status == Status::Off {
            return;
        }

        self.current_temperature = Some(temperature);
        let hysteresis = i32::from(self.hysteresis);

        let new_status = match self.mode {
            Mode::Heating if temperature <= self.heat_setpoint - hysteresis => Status::Active,
            Mode::Heating if temperature >= self.heat_setpoint => Status::Idle,
            Mode::Cooling if temperature >= self.cool_setpoint + hysteresis => Status::Active,
            Mode::Cooling if temperature <= self.cool_setpoint => Status::Idle,
            _ => self.status,
        };

        if self.status == Status::Idle && new_status == Status::Active {
            self.activation_count += 1;
        }
        self.status = new_status;
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn current_temperature(&self) -> Option<i32> {
        self.current_temperature
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_heat_setpoint(&mut self, temperature: i32) {
        self.heat_setpoint = temperature;
    }

    pub fn set_cool_setpoint(&mut self, temperature: i32) {
        self.cool_setpoint = temperature;
    }

    pub fn set_hysteresis(&mut self, hysteresis: u8) {
        self.hysteresis = hysteresis;
    }

    pub fn turn_on(&mut self) {
        if self.status == Status::Off {
            self.status = Status::Idle;
        }
    }

    pub fn turn_off(&mut self) {
        self.status = Status::Off;
    }

    pub fn activation_count(&self) -> u64 {
        self.activation_count
    }

    pub fn info(&self) -> String {
        let mode = match self.mode {
            Mode::Heating => "Нагрев",
            Mode::Cooling => "Охлаждение",
        };
        let setpoint = match self.mode {
            Mode::Heating => self.heat_setpoint,
            Mode::Cooling => self.cool_setpoint,
        };
        let temperature = self
            .current_temperature
            .map_or_else(|| "неизвестна".to_owned(), |value| format!("{value}°C"));

        format!(
            "Термостат: режим {mode}, уставка {setpoint}°C, гистерезис {}°C\n\
             Текущая температура: {temperature}, статус: {:?}",
            self.hysteresis, self.status
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn heating_thermostat() -> Thermostat {
        let mut thermostat = Thermostat::new(Mode::Heating, 20, 25, 2);
        thermostat.turn_on();
        thermostat
    }

    #[test]
    fn new_thermostat_is_off() {
        let mut thermostat = Thermostat::new(Mode::Heating, 20, 25, 2);
        thermostat.update_temperature(10);

        assert_eq!(thermostat.status(), Status::Off);
        assert_eq!(thermostat.current_temperature(), None);
    }

    #[test]
    fn heating_starts_and_stops_at_setpoint() {
        let mut thermostat = heating_thermostat();
        thermostat.update_temperature(15);
        assert_eq!(thermostat.status(), Status::Active);
        thermostat.update_temperature(20);
        assert_eq!(thermostat.status(), Status::Idle);
        thermostat.update_temperature(25);
        assert_eq!(thermostat.status(), Status::Idle);
    }

    #[test]
    fn cooling_uses_its_own_setpoint() {
        let mut thermostat = Thermostat::new(Mode::Cooling, 20, 25, 2);
        thermostat.turn_on();
        thermostat.update_temperature(27);
        assert_eq!(thermostat.status(), Status::Active);
        thermostat.update_temperature(25);
        assert_eq!(thermostat.status(), Status::Idle);
    }

    #[test]
    fn hysteresis_preserves_status_inside_dead_zone() {
        let mut thermostat = heating_thermostat();
        thermostat.update_temperature(18);
        assert_eq!(thermostat.status(), Status::Active);
        thermostat.update_temperature(19);
        assert_eq!(thermostat.status(), Status::Active);
        thermostat.update_temperature(20);
        assert_eq!(thermostat.status(), Status::Idle);
        thermostat.update_temperature(19);
        assert_eq!(thermostat.status(), Status::Idle);
    }

    #[test]
    fn settings_can_be_changed() {
        let mut thermostat = heating_thermostat();
        thermostat.set_heat_setpoint(22);
        thermostat.set_cool_setpoint(28);
        thermostat.set_hysteresis(1);
        thermostat.set_mode(Mode::Cooling);
        thermostat.update_temperature(29);

        assert_eq!(thermostat.status(), Status::Active);
        assert_eq!(thermostat.current_temperature(), Some(29));
    }

    #[test]
    fn activation_count_and_info_work() {
        let mut thermostat = heating_thermostat();
        thermostat.update_temperature(18);
        thermostat.update_temperature(19);
        thermostat.update_temperature(20);
        thermostat.update_temperature(18);

        assert_eq!(thermostat.activation_count(), 2);
        let info = thermostat.info();
        assert!(info.contains("режим Нагрев"));
        assert!(info.contains("Текущая температура: 18°C"));

        thermostat.turn_off();
        assert_eq!(thermostat.status(), Status::Off);
    }
}
