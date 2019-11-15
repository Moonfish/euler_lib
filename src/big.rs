extern crate num_bigint;
extern crate num_traits;

use num_traits::*;

pub fn num_reverse(n:&num_bigint::BigUint)->num_bigint::BigUint{
  let big_ten = &num_bigint::BigUint::from_u64(10).unwrap();
  let big_zero = &num_bigint::BigUint::from_u64(0).unwrap();

  let mut k:num_bigint::BigUint = n.clone();
 
  if &k < big_ten{
    return k;
  }

  let mut r = big_zero.clone();

  while &k > big_zero {
    let d = &k % big_ten;
    r = r * big_ten + d;
    k = k / big_ten;
  }

  r
}

pub fn is_num_palindrome(n:&num_bigint::BigUint)->bool{
  let big_ten = &num_bigint::BigUint::from_u64(10).unwrap();
  let big_zero = &num_bigint::BigUint::from_u64(0).unwrap();

  if n < big_ten{
    return true;
  }

  let mut k:num_bigint::BigUint = n.clone();
  let mut r = num_reverse(n);

  while &k > big_zero && &r > big_zero{
    let d1 = &k % big_ten;
    let d2 = &r % big_ten;
    if d1 != d2 {
      return false;
    }

    k = k / big_ten;
    r = r / big_ten;
  }

  k == r
}

pub fn get_sum_of_digits(n:&num_bigint::BigUint)->usize{
  let big_zero = &num_bigint::BigUint::from_u64(0).unwrap();
  let big_ten = &num_bigint::BigUint::from_u64(10).unwrap();

  let mut s = 0;
  let mut r = n.clone();
  while &r > big_zero{
    let d = &r % big_ten;
    s += d.to_usize().unwrap();
    r = r / big_ten;
  }

  s
}

pub fn get_digit_count(n:&num_bigint::BigUint)->usize{
  let s = n.to_str_radix(10);
  s.len()
}


#[test]
fn test_bi()
{
  let n = num_bigint::BigUint::from_u64(1234).unwrap();

  println!("{}", get_sum_of_digits(&n));
}