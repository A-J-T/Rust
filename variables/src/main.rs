fn main() {
    let mut x = 5;
    println!("The value of x is: {}",x);
    x = 6;
    println!("The value of x is: {}",x);
	let x = 2.0;
	println!("The value of x is: {}",x);
	let y: f32 = 3.0;
	println!("The value of y is: {}",y);
	let t = true;
	let f: bool = false;
	println!("The value of t is: {}",t);
	println!("The value of f is: {}",f);
	let c = 'z';
	let z = 'ℤ';
	let heart_eyed_cat = '😻';
	println!("The value of c is: {}",c);
	println!("The value of z is: {}",z);
	println!("The value of heart_eyed_cat is: {}",heart_eyed_cat);
	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let five_hundred = tup.0;
	let six_point_four = tup.1;
	let one = tup.2;
	println!("The value of 1st element in tuple is: {}",five_hundred);
	let tup = (600,7.4,2);
	let (x, y, z) = tup;
	println!("The value of y is: {}",y);
	let a = [1,2,3,4,5];
	let months = ["January", "February", "March", "April", "May", "June", "July",
"August", "September", "October", "November", "December"];

	let one = a[0];
	let jan = months[0];

	println!("The value of one is: {}",one);
	println!("The value of jan is: {}",jan);
}
