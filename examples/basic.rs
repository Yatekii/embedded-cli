use embedded_cli::*;
fn main() {
    // cli! {
    //     f32
    // }
    // println!("{}", kek);
}

cli! {
    "f0" > f32 > f32,
    "log" > {
        "adc" > "log3" > {
            "adc" > f32,
            "vtg"
        },
        "vtg"
    },
    "log2" > {
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
