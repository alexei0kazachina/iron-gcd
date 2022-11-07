extern crate iron;
#[macro_use] extern crate mime;
extern crate router;
use router::Router;
use iron::prelude::*;
use iron::status;
fn main() {
	let mut router = Router::new();
	router.get("/", get_form, "root");
	router.post("/gcd", post_gcd, "gcd");
	println!("Открывается на http://localhost:3000...");
	Iron::new(router).http("localhost:3000").unwrap();
}
extern crate urlencoded;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;
fn post_gcd(request: &mut Request) -> IronResult<Response> {
		let mut response = Response::new();
		let form_data = match request.get_ref::<UrlEncodedBody>() {
				Err(e) => {
						response.set_mut(status::BadRequest);
						response.set_mut(format!("Ошибка при разборе данных формы: {:?}\n", e));
						return Ok(response);
				}
				Ok(map) => map
		};
		let unparsed_numbers = match form_data.get("n") {
				None => {
						response.set_mut(status::BadRequest);
						response.set_mut(format!("данные формы не имеют 'n' параметр\n"));
						return Ok(response);
				}
				Some(nums) => nums
			};
			let mut numbers = Vec::new();
			for unparsed in unparsed_numbers {
					match u64::from_str(&unparsed) {
							Err(_) => {
									response.set_mut(status::BadRequest);
									response.set_mut(
											format!("Значение для параметра 'n' не является числом: {:?}\n",
											
	 unparsed));
									return Ok(response);
							}
							Ok(n) => { numbers.push(n); }
					}
			}
			let mut d = numbers[0];
			for m in &numbers[1..] {
					d = gcd(d, *m);
			}
			response.set_mut(status::Ok);
			response.set_mut(mime!(Text/Html; Charset=Utf8));
			response.set_mut(
					format!("Наибольший общий делитель чисел {:?} является <b>{}</b>\n",
					
	 numbers, d));
			Ok(response)
	}	
	fn gcd(mut n: u64, mut m: u64) -> u64 {
		assert!(n != 0 && m != 0);
		while m != 0 {
				if m < n {
					let t = m;
					m = n;
					n = t;
			}
			m = m % n;
	}
	n
}			
fn get_form(_request: &mut Request) -> IronResult<Response> {
	let mut response = Response::new();
	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut(r#"
			<title>GCD Calculator</title>
			<form action="/gcd" method="post">
				<input type="text" name="n"/>
				<input type="text" name="n"/>
				<button type="submit">Compute GCD</button>
			</form>
	"#);
	Ok(response)
}