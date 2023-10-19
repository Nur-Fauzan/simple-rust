fn main() {
    const COUNT: usize = 10;
    let mut fibo = [0; COUNT + 1];

    for number in 0..COUNT + 1 {
        if number == 0 {
            //
        } else if number == 1 {
            fibo[number] = fibo[number] + 1;
            print!("{} ", fibo[number])
        } else {
            fibo[number] = fibo[number - 2] + fibo[number - 1];
            print!("{} ", fibo[number])
        }
    }
}
