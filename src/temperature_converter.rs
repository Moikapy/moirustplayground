pub mod temperature_converter {
    pub fn converter_manager(degree: i32, unit: char) -> i32 {
        let mut converted_degree: i32 = 0;
        match unit {
            'c' => {
                converted_degree = celsius_to_fahrenheit(degree);
            }
            'f' => {
                converted_degree = fahrenheit_to_celsius(degree);
            }
            _ => {
                println!("Invalid unit");
            }
        }
        return converted_degree;
    }

    pub fn fahrenheit_to_celsius(f: i32) -> i32 {
        let c = (f - 32) * 5 / 9;
        println!("{} degrees Fahrenheit is {} degrees Celcius", f, c);
        return c;
    }
    pub fn celsius_to_fahrenheit(c: i32) -> i32 {
        let f = (c * 9 / 5) + 32;
        println!("{} degrees Celcius is {} degrees Fahrenheit", c, f);
        return f;
    }
}
