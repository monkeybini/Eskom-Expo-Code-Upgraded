
// trait/function that calculates the gcd of 2 numbers.
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
  assert!(n != 0 && m != 0);
  while m != 0 {
    if m < n {
      std::mem::swap(&mut m, &mut n);
    }
    m %= n;
  }
  n
}


//function a(n) that takes 3 inputs.
// n is the term number
// h is the starting term number. e.g a(1)
// k is what the starting term is. e.g a(1) = 7

fn a(n:u64,h:u64,k:u64) -> u64{
	if n < h {
		return 0
	} 

	else if n==h {
		return k
	} 
	
	else {
		let prev = a(n-1, h, k);
		println!("gcd: {:?}, prev: {:?}, n:{:?}", gcd(prev, n),prev,n);
		return prev + gcd(prev, n)
	}
}


fn main() {
	println!("{:?}", a(5,1,7));
}
