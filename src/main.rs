struct Off;
struct Running;

struct Engine<Mode> {
    mode: Mode,
    power: f64,
}

fn start(engine: &mut Engine<Off>) -> Engine<Running> {
    println!("{}", "Starting!!!");
    Engine {
        mode: Running,
        power: engine.power,
    }
}

fn stop(engine: &mut Engine<Running>) -> Engine<Off> {
    println!("{}", "Stopping!!!");
    Engine {
        mode: Off,
        power: engine.power,
    }
}

fn main() {
    let mut engine: Engine<Off> = Engine {
        mode: Off,
        power: 2.0,
    };
    let mut engine = start(&mut engine);
    let _engine = stop(&mut engine);
}
