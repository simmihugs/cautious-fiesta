struct Off;
struct Idle;
struct Critical;

struct NuclearPowerPlant<Mode> {
    power_generated: f64,
    #[allow(dead_code)]
    mode: Mode,
}

impl NuclearPowerPlant<Critical> {
    fn melts(&self) {
        println!("OH NOOOOOOOOO, WE ARE HAVING A MELTDOWN!");
    }

    fn save_us_jesus(&self) -> NuclearPowerPlant<Off> {
        println!("Help us Jebus!");
        let pp = NuclearPowerPlant {
            power_generated: self.power_generated,
            mode: Idle,
        };
        pp.shutdown()
    }
}

impl NuclearPowerPlant<Off> {
    fn new() -> Self {
        println!("Powerplant build.");
        NuclearPowerPlant {
            power_generated: 0.0,
            mode: Off,
        }
    }

    fn start(&self) -> NuclearPowerPlant<Idle> {
        println!("Powerplant started.");
        NuclearPowerPlant {
            power_generated: 0.0,
            mode: Idle,
        }
    }
}

impl NuclearPowerPlant<Idle> {
    fn produces_energy(&mut self) {
        self.power_generated += 10.0;
        println!("Dumdidum energy generated...");
    }

    fn shutdown(&self) -> NuclearPowerPlant<Off> {
        println!("Shuting the plant down...");
        println!("This much energy produced: {}", self.power_generated);
        NuclearPowerPlant {
            power_generated: self.power_generated,
            mode: Off,
        }
    }

    fn meltdown(&self) -> NuclearPowerPlant<Critical> {
        println!("Oh my god!!");
        NuclearPowerPlant {
            power_generated: self.power_generated,
            mode: Critical,
        }
    }
}

fn main() {
    let power_plant = NuclearPowerPlant::new();

    let mut power_plant = power_plant.start();
    for _ in 0..10 {
        power_plant.produces_energy();
    }
    let power_plant = power_plant.meltdown();
    power_plant.melts();
    power_plant.save_us_jesus();
}
