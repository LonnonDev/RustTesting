#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]


//* This is Mom object
#[derive(Debug)]
struct Mom<'a> {
	name: &'a str,
	age: u8,
}

//* This is your moms color
enum YourMomsColor {
	Red = 0xff0000,
}

//* This is main function or something
fn main() {
	let name: &str = "Your Mother";
	let age = 27;
	let yourmom = Mom {name, age};
	let y = {
		let x = 2;
		let gaming = 3;

		x + gaming
	};
	println!("{}", 0.1 + 0.2);
	println!("Your mom is \n{:#?}", yourmom);
	println!("Your moms color 0x{:06x?}", YourMomsColor::Red as u32);
	println!("{}", y);
	if yourmom.name == "Your Mother" {
		println!("Your mother");
	} else {
		println!("Your mother's name is {}", yourmom.name);
	}
	seperator(Some("FizzBuzz"));
	//$ LOL IMAGINE BEING EXCLUSIVE ON THE UPPER END
	for i in 0..=100 {
		if i % 15 == 0 {
			println!("fizzbuzz");
		} else if i % 5 == 0 {
			println!("buzz");
		} else if i % 3 == 0 {
			println!("fizz");
		} else {
			println!("{} ", i)
		}
	}
	seperator(Some("Testing Pointers"));

	let funny_string = String::from("Your Mom");
	let string_length = your_mom(&funny_string);

	println!("{}", string_length);

	seperator(Some("If Let"));
	let somebs = Some(0u8);
	if let Some(0) = somebs {
		println!("coolio");
	}
}

fn seperator(additional_info: Option<&str>) {
	let mut new_info = additional_info.unwrap_or("");
	if new_info != "" {
		new_info = "your mom";
	}
	println!("-------{}-------", new_info)
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn your_mom(a: &String) -> usize {
	a.len()
}