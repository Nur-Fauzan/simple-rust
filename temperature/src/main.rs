fn change(celcius: i32) -> i32 {
    (celcius * 9 / 5) + 32
}

fn main() {
    let celcius = -40;
    println!("Celcius = {}\nFahrenheit = {}", celcius, change(celcius));
}
