extern crate image;

use image::GenericImageView;
use std::env;
use std::path::Path;
use image::io::Reader;
use image::DynamicImage;
use image::imageops::FilterType;

/*
-- COMMANDS --
resize -s 100 -> resize the image to closest size smaller than 100b
resize -p 50 -> resize the image by 50% of the original dimensions
resize -h 100 -> resize the image to 100px in height
resize -w 100 -> resize the image to 100px in width
help -> display list of commands
*/
#[derive(Clone)]
enum Specifier {
	FileSize,
	Percentage,
	Height,
	Width
}

#[derive(Clone)]
struct ResizeData {
	img : DynamicImage,
	sz : f64,
	spec : Specifier,
}

impl ResizeData {
	fn new(img : DynamicImage, sz : f64, spec : Specifier) -> ResizeData {
		ResizeData {
			img,
			sz,
			spec,
		}
	}
}

fn parse_cmd(args : Vec<String>) -> Result<ResizeData, String> {
	println!("{:?}", args[0]);

	// check number of arguments
	if args.len() < 2 {
		return Err(String::from("No command found"))
	}
	else if args.len() > 5 {
		return Err(String::from("Too many arguments found"))
	}

	// parse arguments
	let cmd = args[1].clone();
	let size : f64;
	let spec : Specifier;
	if cmd == String::from("resize") && args.len() == 5 {
		let flag = args[2].clone();
		if flag == String::from("-s") {
			size = args[3].parse::<f64>().unwrap();
			spec = Specifier::FileSize;

			if size < 1. {
				return Err(String::from("-h flag only takes values greater than 1"))
			}
		}
		else if flag == String::from("-p") {
			size = args[3].parse::<f64>().unwrap() * 0.01;
			spec = Specifier::Percentage;

			if size >= 1. || size <= 0. {
				return Err(String::from("-p flag only takes values in between 0-100"))
			}
		}
		else if flag == String::from("-h") {
			size = args[3].parse::<f64>().unwrap();
			spec = Specifier::Height;

			if size < 1. {
				return Err(String::from("-h flag only takes values greater than or equal to 1"))
			}
		}
		else if flag == String::from("-w") {
			size = args[3].parse::<f64>().unwrap();
			spec = Specifier::Width;
			if size < 1. {
				return Err(String::from("-w flag only takes values greater than or equal to 1"))
			}
		}
		else {
			return Err(String::from("Unknown flag for resize"))
		}
	}
	else if cmd == String::from("help") && args.len() == 2 {
		return Err(String::from("Help"))
	}
	else {
		return Err(String::from("Unknown command"))
	}

	// get the image file
	let mut path = env::current_dir().unwrap();
	path.push(Path::new(&args[4]));
	let img = match Reader::open(path) {
		Ok(r) => match r.decode() {
			Ok(r) => r,
			Err(_) => return Err(String::from("Unable to open image")),
		},
		Err(_) => return Err(String::from("Unable to open image")),
	};

	Ok(ResizeData::new(img, size, spec))
}

fn resize_image(data : ResizeData) -> DynamicImage {
	let (orig_img, target_size, spec) = (data.img, data.sz, data.spec);

	let orig_dim = (orig_img.dimensions().0 as f64, orig_img.dimensions().1 as f64);
	match spec {
		Specifier::FileSize => orig_img.resize_exact(target_size as u32, target_size as u32, FilterType::Gaussian),
		Specifier::Percentage => orig_img.resize_exact((orig_dim.0*target_size) as u32, (orig_dim.1*target_size) as u32, FilterType::Gaussian),
		Specifier::Height => orig_img.resize_exact((target_size/orig_dim.1 * orig_dim.0) as u32, target_size as u32, FilterType::Gaussian),
		Specifier::Width => orig_img.resize_exact(target_size as u32, (target_size/orig_dim.0 * orig_dim.1) as u32, FilterType::Gaussian),
	}
}

fn save_image(new_img : DynamicImage, filename : &str) -> Result<String, String> {
	let mut path = env::current_dir().unwrap();
	
	path.push(String::from("resized_") + filename);
	match new_img.save(path.clone()) {
		Ok(_) => Ok(String::from(path.to_str().unwrap())),
		Err(_) => return Err(String::from("Could not save backup.")),
	}
}

fn main() {
	let args : Vec<String> = env::args().collect();
	//println!("{:?}", env::current_dir());
	match parse_cmd(args.clone()) {
		Ok(data) => {
			println!("Data found");
			
			let img = resize_image(data);
			match save_image(img, &args[4]) {
				Ok(p) => println!("SUCCESS : Image saved at {:?}", p),
				Err(e) => println!("ERROR while saving image : {:?}.", e),
			}
		},
		Err(e) => println!("ERROR while parsing command : {:?}.", e),
	}
}