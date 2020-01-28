extern crate clap;
extern crate dirs;
extern crate regex;
// extern crate yaml_rust;

use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// use yaml_rust::{YamlEmitter, YamlLoader};

#[derive(Copy, Clone)]
enum OutputType {
    HEX,
    RGB,
    HSL,
}

impl std::fmt::Display for OutputType {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            OutputType::HEX => write!(f, "hex value"),
            OutputType::RGB => write!(f, "RGB notation"),
            OutputType::HSL => write!(f, "HSL notation")
        }
    }
 }

fn main() {
    let config_dir: String = {
        match dirs::config_dir() {
            Some(v) => String::from({ v.to_str().unwrap() }),
            _ => String::new(),
        }
    };
    let filename: String = String::from("colorconverter.yaml");

    let config_path = Path::new(&config_dir);
    let config_file = config_path.join(filename);
    let display = &config_file.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file: File = match File::open(&config_file) {
        Err(_) => {
            println!("Could not find config file, creating it");
            match File::create(&config_file) {
                Err(why) => panic!(
                    "Couldn't create config file {}: {}",
                    display,
                    why.description()
                ),
                Ok(file) => file,
            }
        }
        Ok(file_opened) => {
            print!("");
            file_opened
        }
    };
    let mut config_content = String::new();
    match file.read_to_string(&mut config_content) {
        Err(why) => panic!(
            "Could not read config file at {}: {}",
            display,
            why.description()
        ),
        Ok(_) => print!(""),
    }

    // let config = YamlLoader::load_from_str(&config_content.to_string()).unwrap();

    let matches = App::new("Color converter")
        .version("0.1")
        .author("Markus Flür <markusfluer@markusfluer.de>")
        .about("Convert color values easily")
        .arg(
            Arg::with_name("red")
                .short("r")
                .long("red")
                .value_name("RED")
                .help("The red value in an rgb color")
                .takes_value(true)
                .requires_all(&["green", "blue"]),
        )
        .arg(
            Arg::with_name("green")
                .short("g")
                .long("green")
                .value_name("GREEN")
                .help("The green value in an rgb color")
                .takes_value(true)
                .requires_all(&["red", "blue"]),
        )
        .arg(
            Arg::with_name("blue")
                .short("b")
                .long("blue")
                .value_name("BLUE")
                .help("The blue value in an rgb color")
                .takes_value(true)
                .requires_all(&["green", "red"]),
        )
        .arg(
            Arg::with_name("hex")
                .short("x")
                .long("hex")
                .value_name("HEX")
                .help("the color in a 6 digit hex value")
                .takes_value(true)
                .required_unless_one(&["red", "hue"]),
        )
        .arg(
            Arg::with_name("hue")
                .short("h")
                .long("hue")
                .value_name("HUE")
                .help("The hue value in an hsl color")
                .takes_value(true)
                .requires_all(&["saturation", "luminance"]),
        )
        .arg(
            Arg::with_name("saturation")
                .short("s")
                .long("saturation")
                .value_name("SATURATION")
                .help("The saturation value in an hsl color")
                .takes_value(true)
                .requires_all(&["hue", "luminance"]),
        )
        .arg(
            Arg::with_name("luminance")
                .short("l")
                .long("luminance")
                .value_name("LUMINANCE")
                .help("The luminance value in an hsl color")
                .takes_value(true)
                .requires_all(&["saturation", "hue"]),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("The output format. Must be one of  'HEX', 'RGB' or 'HSL'")
                .takes_value(true)
                .required(true)
                .validator(|value: String| -> Result<(), String> {
                    let test_reg_ex = Regex::new(r"^((HEX)|(RGB)|(HSL))$").unwrap();
                    if test_reg_ex.is_match(&value) {
                        Ok(())
                    } else {
                        Err(String::from("Output must be one of 'HEX', 'RGB' or 'HSL' "))
                    }
                }),
        )
        .get_matches();

    let result: String;

    let output_type = match matches.value_of("output").unwrap() {
        "HEX" => OutputType::HEX,
        "RGB" => OutputType::RGB,
        "HSL" => OutputType::HSL,
        _ => panic!("No valid output type selected")
    };

    if matches.is_present("hex") {
        let hex_color: String = String::from(matches.value_of("hex").unwrap());
        
        let red_value = to_dec(&hex_color[0..2]);
        let green_value = to_dec(&hex_color[2..4]);
        let blue_value = to_dec(&hex_color[4..]);

        result = create_output(red_value, green_value, blue_value, output_type);
    } else if matches.is_present("red")  {
        let red_value = match String::from(matches.value_of("red").unwrap()).parse::<i16>() {
            Ok(val) => val,
            Err(_) => panic!("Red value not a valid number")
        };
        let green_value = match String::from(matches.value_of("green").unwrap()).parse::<i16>() {
            Ok(val) => val,
            Err(_) => panic!("Green value not a valid number")
        };
        let blue_value = match String::from(matches.value_of("blue").unwrap()).parse::<i16>() {
            Ok(val) => val,
            Err(_) => panic!("Blue value not a valid number")
        };

        result = create_output(red_value, green_value, blue_value, output_type);
    } else if matches.is_present("hue") {
        let hue_value: f32 = match String::from(matches.value_of("hue").unwrap()).parse::<f32>() {
            Ok(val) => val,
            Err(_) => panic!("Hue value not a valid number")
        };
        let luminance_value: f32 = match String::from(matches.value_of("luminance").unwrap()).parse::<f32>() {
            Ok(val) => val/100.0,
            Err(_) => panic!("Luminance value not a valid number")
        };
        let saturation_value: f32 = match String::from(matches.value_of("saturation").unwrap()).parse::<f32>() {
            Ok(val) => val/100.0,
            Err(_) => panic!("Saturation value not a valid number")
        };

        if saturation_value == 0.0_f32 {
            let color_value = (255.0 * luminance_value) as i16;
            result = create_output(color_value, color_value, color_value, output_type);
        } else {
            let temporary_1: f32;
            if luminance_value < 0.5 { 
                temporary_1 = luminance_value * (1.0 + saturation_value);
            } else {
                temporary_1 = (saturation_value + luminance_value) - (saturation_value * luminance_value);
            }

            let temporary_2: f32 = 2.0*&luminance_value-&temporary_1;
            let hue_grad: f32 = &hue_value/360.0;

            let temporary_red: f32 = {
                if hue_grad+0.333 > 1.0 {
                    hue_grad+0.33-1.0
                } else {
                    hue_grad+0.33
                }
            };
            let temporary_green: f32 = hue_grad;
            let temporary_blue: f32 = {
                if hue_grad-0.333 < 0.0 {
                    hue_grad-0.333+1.0
                } else {
                    hue_grad-0.333
                }
            };

            let red = recalculate_rgb(&temporary_red, &temporary_1 ,&temporary_2);
            let green = recalculate_rgb(&temporary_green, &temporary_1 ,&temporary_2);
            let blue = recalculate_rgb(&temporary_blue, &temporary_1 ,&temporary_2);

            result = create_output(red, green, blue, output_type);
        }
    } else {
        result = String::from("This should never happen");
    }
    println!("Converted to {} your value is:\n {}", &output_type, &result);
}

fn to_dec(value: &str) -> i16 {
    match i16::from_str_radix(value, 16) {
        Ok(val) => val,
        Err(_) => panic!("Not a valid hex value: {:?}", value)
    }
}

fn to_hex(value: &i16) -> String {
    format!("{:X}", value)
}

fn get_hsl(red: i16, green: i16, blue: i16) -> String {
    let float_red = red as f32;
    let float_green = green as f32;
    let float_blue = blue as f32;

    let parted_red = float_red / 255.0;
    let parted_green = float_green / 255.0;
    let parted_blue = float_blue / 255.0;

    let max: f32 = get_max(&parted_red, &parted_green, &parted_blue);
    let min: f32 = get_min(&parted_red, &parted_green, &parted_blue);

    let luminance: f32 = (max+min)/2.0;
    let saturation: f32;
    if max==min {
        saturation = 0.0;
    } else {
        if luminance < 0.5 {
            saturation = (max-min)/(max+min);
        } else {
            saturation = (max-min)/(2.0-max-min);
        }
    }
    
    let hue_grad: f32 = match max {
        v if v == parted_red => {(parted_green-parted_blue)/(max-min)},
        v if v == parted_green => {2.0 + (parted_blue-parted_red)/(max-min)},
        v if v == parted_blue => {4.0 + (parted_red-parted_green)/(max-min)}
        _ => panic!("Error evaluating a number")
    };
    
    let hue = 60*(hue_grad as i32);
    
    format!("{},{}%,{}%", hue, (saturation*100.0).round() as i32, (luminance*100.0).round() as i32)
}

fn create_output(red: i16, green: i16, blue: i16, output: OutputType) -> String {
    match output {
        OutputType::HEX => format!("==>  #{}{}{}",to_hex(&red),to_hex(&green),to_hex(&blue)),
        OutputType::RGB => format!("==>  rgb({:?},{:?},{:?})", red, green, blue),
        OutputType::HSL => format!("==>  hsl({})", get_hsl(red, green, blue)),
    }
}

fn get_max(v1: &f32, v2: &f32, v3: &f32) -> f32 {
    if v1 < v2 {
        if v2 < v3 {
            return v3.clone();
        }
        return v2.clone();
    } 
    if v1 < v3 {
        return v3.clone();
    }
    return v1.clone();
}

fn get_min(v1: &f32, v2: &f32, v3: &f32) -> f32 {
    if v1 > v2 {
        if v2 > v3 {
            return v3.clone();
        }
        return v2.clone();
    }
    if v1 > v3 {
        return v3.clone();
    }
    return v1.clone();
}

fn recalculate_rgb (temp_color: &f32, t1: &f32, t2: &f32) -> i16 {
    if (6.0*temp_color) < 1.0 {
        ((t2+(t1-t2)*6.0*temp_color)*255.0) as i16
    } else if (2.0*temp_color) < 1.0 {
        (t1*255.0) as i16
    } else if (3.0*temp_color) < 2.0 {
        ((t2+(t1-t2)*(0.666-temp_color)*6.0)*255.0) as i16
    } else {
        (t2*255.0) as i16
    }
}