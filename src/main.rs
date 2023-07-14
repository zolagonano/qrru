use clap::{App, Arg, SubCommand};
use image::Luma;
use qrcode::QrCode;
use rqrr::PreparedImage;

fn main() {
    let mut app = App::new("qrru")
        .version("0.1.1")
        .author("Zola Gonano <zolagonano@protonmail.com>")
        .about("Encodes/Decodes qrcodes in the command line")
        .subcommand(
            SubCommand::with_name("encode")
                .about("Encodes a string")
                .arg(Arg::with_name("input_data").required(true))
                .arg(
                    Arg::with_name("file_name")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("width")
                        .short("w")
                        .long("width")
                        .takes_value(true)
                        .default_value("250"),
                )
                .arg(
                    Arg::with_name("height")
                        .short("h")
                        .long("height")
                        .takes_value(true)
                        .default_value("250"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("decode")
                .about("Decodes a qrcode")
                .arg(
                    Arg::with_name("input_file")
                        .takes_value(true)
                        .required(true),
                ),
        );

    let mut help = Vec::new();
    app.write_long_help(&mut help).unwrap();

    let matches = app.get_matches();

    if matches.is_present("encode") {
        let subcommands = matches.subcommand_matches("encode").unwrap();
        let input_data = subcommands.value_of("input_data").unwrap();
        let file_name = subcommands.value_of("file_name").unwrap();
        let width = subcommands.value_of("width").unwrap();
        let height = subcommands.value_of("height").unwrap();
        let verbose = subcommands.is_present("verbose");

        let encode_result = qr_encode(
            input_data.as_bytes(),
            width.parse().unwrap(),
            height.parse().unwrap(),
            file_name,
        );

        match encode_result {
            Ok(_v) => {
                if verbose {
                    eprintln!("Image successfully saved at {file_name}");
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    } else if matches.is_present("decode") {
        let subcommands = matches.subcommand_matches("decode").unwrap();
        let input_file = subcommands.value_of("input_file").unwrap();

        match qr_decode(input_file) {
            Ok(v) => println!("{v}"),
            Err(e) => eprintln!("{e}"),
        }
    } else {
        println!("{}", String::from_utf8(help).unwrap());
    }
}

fn qr_encode(input_data: &[u8], width: u32, height: u32, output_file: &str) -> Result<(), String> {
    let code = QrCode::new(input_data).map_err(|e| format!("Error: {e}"))?;
    let image = code
        .render::<Luma<u8>>()
        .max_dimensions(width, height)
        .build();

    image.save(output_file).map_err(|e| format!("Error: {e}"))
}

fn qr_decode(input_qr: &str) -> Result<String, &'static str> {
    let qrcode_image = image::open(input_qr)
        .map_err(|_| "Error: Failed to load this image")?
        .to_luma8();

    let mut prepared_qr = PreparedImage::prepare(qrcode_image);
    let qr_grids = prepared_qr.detect_grids();
    if let Ok((_meta, content)) = qr_grids[0].decode() {
        Ok(content)
    } else {
        Err("Error: Cannot decode qr grids")
    }
}
