extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate steganography;
use steganography::encoder::*;
use steganography::decoder::*;
use steganography::util::*;

extern crate spinners;
extern crate ansi_escapes;
use spinners::{Spinner, Spinners};

use std::path::Path;
use std::fs;
use std::process;
use std::process::Command;


fn decode(image_path: &Path, out_file_name: String) -> Option<String>{
    //Load the image with the secret message
    let encoded_image = Some(file_as_image_buffer(image_path.to_str().unwrap().to_owned()))?;
    //Create a decoder
    let dec = Decoder::new(encoded_image);
    //Decode the image by reading the alpha channel
    let out_buffer = dec.decode_alpha();
    //If there is no alpha, it's set to 255 by default so we filter those out
    let clean_buffer: Vec<u8> = out_buffer.into_iter().filter(|b| {
                                            *b != 0xff_u8
                                        }).collect();
    //Convert those bytes into a string we can read
    let messg_vec:  Vec<&str> = bytes_to_str(clean_buffer.as_slice()).lines().collect();
    // Separate extension encoded and code body 
    let extension = messg_vec[0].to_owned();
    let raw_code_body = messg_vec[1..messg_vec.len()-1].join("\n");
    let code_body = raw_code_body.trim_matches(char::from(0)); // removes end of text unicode characters
    // Create filename from output path provided
    let file_name = format!("{}.{}",out_file_name,extension);
    // Write file
    fs::write(&file_name, code_body).expect("Unable to write file");
    return Some(file_name);
}


fn encode(code_path: &Path, photo_path: &Path, output_name: String){
    let extension_type = code_path.extension().unwrap().to_str().unwrap();
 
    let file_body = fs::read_to_string(code_path) 
                    .expect("Something went wrong reading the file");
    let mut code_body = extension_type.to_owned() + "\n"; // Add this to encode the file name in the image
    code_body.push_str(&file_body);
    //Convert our string to bytes
    let payload = str_to_bytes(&code_body);
    //Load the image where we want to embed our secret message
    let destination_image = Some(file_as_dynamic_image(photo_path.to_str().unwrap().to_owned()));
    //Create an encoder
    let enc = Encoder::new(payload, destination_image.expect("Something is wrong with the input image file.\n"));
    //Encode our message into the alpha channel of the image
    let result = enc.encode_alpha();
    //Save the new image
    let res = Some(save_image_buffer(result,(output_name + ".png").to_string()));
    if res.is_none() {
        println!("Error: Issue saving image to output file.");
    }
}


fn main(){
    let matches = App::new("codeography")
                        .version("0.1.0")
                        .author("Adam Nieto <anieto1@binghamton.edu>")
                        .about("Create, encode, and decode image code snippets into what they represent pictorially via steganography")
                        .subcommand(SubCommand::with_name("encode")
                            .about("Encodes code snippet image with its actual source using steganography")
                                .arg(Arg::with_name("INPUT_CODE_FILE")
                                .required(true)
                                .help("The code file that will be encoded into INPUT_IMAGE_TO_ENCODE."))
                            .arg(Arg::with_name("INPUT_IMAGE_TO_ENCODE")
                                .required(true)
                                .help("The image that will be encoded using steganography with INPUT_CODE_FILE."))
                            .arg(Arg::with_name("ENCODE_OUTPUT_IMAGE_FILE_NAME")
                                .required(true)
                                .help("Usage: <name>  : The name of the output file that is encoded. Please provide name only.")))
                        .subcommand(SubCommand::with_name("decode")
                            .about("Decodes code snippet image into a corresponding source file using steganography")
                            .arg(Arg::with_name("INPUT_IMAGE_TO_DECODE")
                                .required(true)
                                .help("The image that will be decoded using steganography."))
                            .arg(Arg::with_name("DECODE_OUTPUT_FILE_NAME")
                                .required(true)
                                .help("The name of the output file of the decoded image file.")))
                        .subcommand(SubCommand::with_name("imagify")
                            .about("Creates an image code snippet using Carbon-Now CLI.")
                            .arg(Arg::with_name("INPUT_FILE")
                                .required(true)
                                .help("The code file of the code snippet image.")))
                    .get_matches();
   
   // Spinner
   let sp: Spinner;

    // Logical control flow
    if let Some(matches) = matches.subcommand_matches("encode"){ 
        sp = Spinner::new(Spinners::Dots9, "Encoding image...".into());
        let input_code_file_path = Path::new(matches.value_of("INPUT_CODE_FILE").unwrap_or(""));
        let input_image_path = Path::new(matches.value_of("INPUT_IMAGE_TO_ENCODE").unwrap_or(""));
        // Checks
        if !input_code_file_path.exists() {
            println!("<INPUT_CODE_FILE> argument does not exist.");
            process::exit(1);
        }
        if !input_image_path.exists() {
            println!("<INPUT_IMAGE_TO_ENCODE> argument does not exist.");
            process::exit(1);
        }    
        if !(input_image_path.extension().unwrap() == "png") && !(input_image_path.extension().unwrap() == "svg") {
            println!("The extension of the image file is not a 'png' or 'svg' file.");
            process::exit(1);
        }
  
        let output_image_file_name = matches.value_of("ENCODE_OUTPUT_IMAGE_FILE_NAME").unwrap_or("");
       
        // Encodes image file with code file using steganography 
        // and outputs to the current directly with a specified file name
        encode(&input_code_file_path, &input_image_path, output_image_file_name.to_string());
        sp.stop();
        print!("{}", ansi_escapes::EraseLines(1));
        println!(" ✅  Successfully created: {}.png",output_image_file_name.to_string());

    }else if let Some(matches) = matches.subcommand_matches("decode"){
        
        sp = Spinner::new(Spinners::Dots9, "Decoding image...".into());

        let input_image_path = Path::new(matches.value_of("INPUT_IMAGE_TO_DECODE").unwrap_or(""));
        let output_code_file_name = matches.value_of("DECODE_OUTPUT_FILE_NAME").unwrap_or("output_file");
        // Checks
        if !input_image_path.exists() {
            println!("<INPUT_IMAGE_TO_DECODE> argument does not exist.");
            process::exit(1);
        }
        // Decodes image file into code file using steganography
        let file_name = decode(&input_image_path, output_code_file_name.to_owned());
        sp.stop();
        print!("{}", ansi_escapes::EraseLines(1));
        println!(" ✅  Successfully created: {}",file_name.unwrap());

    }else if let Some(matches) = matches.subcommand_matches("imagify"){
        let input_code_path = Path::new(matches.value_of("INPUT_FILE").unwrap_or(""));
        // Checks
        if !input_code_path.exists() {
            println!("<INPUT_FILE> argument does not exist.");
            process::exit(1);
        }
        // Execute carbon-now command interactively
        Command::new("carbon-now")
            .arg(input_code_path.to_str().unwrap().to_owned())
            .arg("-i")
            .status()
            .expect("carbon-now command failed. Make sure to install it (https://github.com/mixn/carbon-now-cli)");
    }else{
        println!("{}\n Please provide a subcommand such as 'encode', 'decode' or 'imagify'", matches.usage());
    }
}
