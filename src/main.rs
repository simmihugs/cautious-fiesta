#[derive(Debug)]
enum Mode {
    Off,
    Idle,
    Critical,
}

struct NuclearPowerPlant {
    power_generated: f64,
    #[allow(dead_code)]
    mode: Mode,
}

impl NuclearPowerPlant {
    fn melts(&self) {
        match self.mode {
            Mode::Critical => {
                println!("OH NOOOOOOOOO, WE ARE HAVING A MELTDOWN!");
            }
            _ => {
                println!("Cannot melt in the current state {:?}", self.mode);
            }
        }
    }

    fn save_us_jesus(&mut self) {
        match self.mode {
            Mode::Critical => {
                println!("Help us Jebus!");
                self.mode = Mode::Idle;
                self.shutdown();
            }
            Mode::Off => {
                println!("Not running");
            }
            Mode::Idle => {
                println!("Not in crisis");
            }
        }
    }

    fn new() -> Self {
        println!("Powerplant build.");
        NuclearPowerPlant {
            power_generated: 0.0,
            mode: Mode::Off,
        }
    }

    fn start(&mut self) {
        match self.mode {
            Mode::Off => {
                println!("Powerplant started.");
                self.mode = Mode::Idle;
            }
            _ => {
                println!("Cannot start in this state: {:?}", self.mode);
            }
        }
    }

    fn produces_energy(&mut self) {
        match self.mode {
            Mode::Idle => {
                self.power_generated += 10.0;
                println!("Dumdidum energy generated...");
            }
            _ => {
                println!(" {:?}", self.mode);
            }
        }
    }

    fn shutdown(&mut self) {
        match self.mode {
            Mode::Idle => {
                println!("Shuting the plant down...");
                println!("This much energy produced: {}", self.power_generated);
                self.mode = Mode::Off;
            }
            Mode::Off => {
                println!("Cannot shutdown, already shutdown!");
            }
            Mode::Critical => {
                println!("Cannot shutdown! EMERGENCY!!!!");
            }
        };
    }

    fn meltdown(&mut self) {
        match self.mode {
            Mode::Idle => {
                println!("Oh my god!!");
                self.mode = Mode::Critical;
            }
            Mode::Critical => {
                println!("Already PANIC!!!");
            }
            Mode::Off => {
                println!("Nothing to see here");
            }
        }
    }
}

fn main() {
    let mut power_plant = NuclearPowerPlant::new();
    power_plant.start();
    for _ in 0..10 {
        power_plant.produces_energy();
    }
    power_plant.meltdown();
    power_plant.melts();
    power_plant.save_us_jesus();
}
