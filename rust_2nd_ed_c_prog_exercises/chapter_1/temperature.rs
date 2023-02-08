fn main() {
    let mut fahr = 0.0;
    let upper = 300.0;
    let step = 20.0;
    let border = "********************\n";

    println!("{}TEMP CONVERTER\n{}", border, border);

    while fahr <= upper {
        let celcius = 5.0 * (fahr - 32.0) / 9.0;
        println!("fahr: {:>3} ---- celcius: {:>5.2}\n", fahr, celcius);
        fahr += step;
    }
}
