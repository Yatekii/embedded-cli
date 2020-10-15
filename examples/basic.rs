use embedded_cli::*;
fn main() {
    // cli! {
    //     f32
    // }
    // println!("{}", kek);
}

cli! {
    "f0" > f32,
    "log" > {
        "adc",
        "vtg"
    }
}

// cli! {
//     "f0" > f32,
//     "log" > {
//         "adc" > ("on" | "off") => bool,
//         "vtg" > ("on" | "off"),
//     }
// }
