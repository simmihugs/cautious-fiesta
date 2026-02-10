fn main() {
    let temperatures = (-100..100).step_by(5);
    temperatures.for_each(|t| {
        let f = celcius_to_farenheit(t as f64);
        let d = farenheit_to_celcius(f);
        let are_the_same = d == t as f64;

        print!("the temperature is {t:4}°C and {f:4}°F.\t");

        println!("c={t}°C -> f={f}°F -> d={d}°C => d == c ?: {are_the_same}");
    });
}

#[allow(dead_code)]
fn farenheit_to_celcius(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.0 / 9.0)
}

fn celcius_to_farenheit(temperature: f64) -> f64 {
    temperature * (9.0 / 5.0) + 32.0
}
