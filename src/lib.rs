pub mod cards;
pub mod big;

pub mod utility
{
  use std::collections::HashSet;
  
  pub fn choose(n:usize, r:usize)->Option<usize>
  {  
    let mut k = r;
 
    if n == r{
      return Some(1);
    }
    if r > n {
      return Some(0);
    }
    // Since C(n, k) = C(n, n-k)  
    if k > (n - k){  
        k = n - k; 
    } 

    let mut res:usize = 1;  
  
    // Calculate value of  
    // [n * (n-1) *---* (n-k+1)] / [k * (k-1) *----* 1]  
    for i in 0..k
    {
      let m = res.checked_mul(n-i);
      match m{
        Some(val) => {res = val;},
        None => {return None;}
      }
        
      res /= i + 1;  
    }  
  
    Some(res)  
  } 
   
  // returns none on overflow
  pub fn factorial(i: usize) -> Option<usize> {
    let mut acc:usize = 1;
    for num in 2..=i {
        let result = acc.checked_mul(num);
        match result{
          Some(val) => {acc = val;},
          None => {return None;}
        };
      }
    
    Some(acc)
  }

  pub fn digits_to_int(v : &Vec<u8>)->usize
  {
    let mut p:usize = 1;
    let mut n:usize = 0;
    let offset = v.len() - 1;

    for i in 0..v.len(){
      let d = v[offset-i] as usize;
      n = n + d * p;
      p = p * 10;
    }

    n
  }

  pub fn int_to_digits(n:usize)->Vec<u8>
  {
    let mut m_n = n;
    let mut v:Vec<u8> = Vec::new();
    
    loop
    {
      if m_n < 10{
        v.insert(0, m_n as u8);
        break;  
      }
     
      let k = m_n / 10;
      let d = m_n - k * 10;
    
      v.insert(0, d as u8);
      m_n = k;
    }

    v
  }

  fn gen_pandigital_inner<F>(f : &mut F, arr : &mut [usize], sz : usize, n : usize, exit : &mut bool) where
  F : FnMut(usize)->bool {
    if *exit
    {
      return;
    }
    
    if sz == 1
    {
      let j = to_num(arr);
      if f(j) == true
      {
        *exit = true;
      }
      return;
    }

    for k in 0..sz 
    {
      gen_pandigital_inner(f, arr, sz-1, n, exit);

      if sz % 2 == 1
      {
        arr.swap(0, sz-1);
      }
      else
      {
        arr.swap(k, sz-1);
      }  
    }
  }

  fn gen_permutation_inner<F>(f : &mut F, arr : &mut [usize], sz : usize, n : usize, exit : &mut bool) where
  F : FnMut(&[usize])->bool {
    if *exit
    {
      return;
    }
    
    if sz == 1
    {
      if f(arr) == true
      {
        *exit = true;
      }
      return;
    }

    for k in 0..sz 
    {
      gen_permutation_inner(f, arr, sz-1, n, exit);

      if sz % 2 == 1
      {
        arr.swap(0, sz-1);
      }
      else
      {
        arr.swap(k, sz-1);
      }  
    }
  }

  pub fn gen_pandigital<F>(f : &mut F) where
  F : FnMut(usize)->bool
  {
    let mut arr:[usize; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut exit : bool = false;
    gen_pandigital_inner(f, &mut arr, 10, 10, &mut exit);
  }

  pub fn gen_permutation<F>(pattern : &[usize], f : &mut F) where
  F : FnMut(&[usize])->bool
  {
    let mut v = pattern.to_vec();
    let sz = v.len();
    let mut arr = &mut v[..];
    let mut exit : bool = false;
    gen_permutation_inner(f, &mut arr, sz, sz, &mut exit);
  }

  pub fn is_pandigital_z(n:u64)->bool
  {
    let mut x = 0;
    let s = n.to_string();
    
    for c in s.chars()
    {
      let p = c as u32 - '0' as u32;
      x = x | (1 << p);
    }
    
    x >= 0x3FE
  }

  // (1-9) pandigital test, does not restrict duplicate digits.
  pub fn is_pandigital(n:usize)->bool
  {
    let mut u = n;
    let mut digits = 0;
    let mut count = 0;
    let mut _tmp;

    while u > 0
    {
      _tmp = digits;
      digits = digits | (1 << (u-((u/10)*10)-1));
      if _tmp == digits
      {
        return false;
      }

      u = u / 10;
      count = count + 1;
    }
     
     return digits == (1 << count) - 1;
  }

  fn to_num(arr : &[usize])->usize{

    let mut s = String::new();
    for j in 0..10{
      let k = arr[j].to_string();
      s = s + &k;
    }

    let r:usize = s.parse().unwrap();
    r
  }

  pub fn pentagonal(n:usize)->usize
  {
    n * (3 * n - 1) / 2
  }

  pub fn is_pentagonal(n:usize)->bool
  {
    let f:f64 = (1.0 + f64::sqrt(24.0 * (n as f64) + 1.0)) / 6.0;
    f - f.floor() == 0.0
  }

  pub fn hexagonal(n:usize)->usize
  {
    n*(2*n-1)
  }

  pub fn is_hexagonal(n:usize)->bool
  {
    let k = 8 * n + 1;
    let t = (f64::sqrt(k as f64) + 1.0) / 4.0;

    t - t.floor() == 0.0
  }

  pub fn triangular(n:usize)->usize
  {
    n*(n+1)/2
  }

  pub fn is_triangular(n:usize)->bool
  {
    let j = n as i64;

    // Considering the equation n*(n+1)/2 = num 
    // The equation is  : a(n^2) + bn + c = 0"; 
    let c:i64 = -2 * j; 
    let b:i64 = 1;
    let a:i64 = 1; 
    let d = (b * b) - (4 * a * c); 
  
    if d < 0 {
      return false; 
    }

    // Find roots of equation 
    let root1:f64 = ( -(b as f64) + f64::sqrt(d as f64)) / (2.0 * (a as f64)); 
    let root2:f64 = ( -(b as f64) - f64::sqrt(d as f64)) / (2.0 * (a as f64)); 
  
    // checking if root1 is natural 
    if root1 > 0.0 && root1 - f64::floor(root1) == 0.0 {
        return true; }
  
    // checking if root2 is natural 
    if root2 > 0.0 && root2 - f64::floor(root2) == 0.0 {
        return true; }
  
    false 
  }

  pub fn gen_primes(max:usize)->HashSet<usize>
  {
    let mut not_prime:HashSet<usize> = HashSet::new();
    let mut prime:HashSet<usize> = HashSet::new();
    
    for i in 2..max+1{
      if !&not_prime.contains(&i){
        &prime.insert(i);
        
        for j in i+1..max+1{
          if j % i == 0{
            &not_prime.insert(j);
          }
        }
      }
    }

    prime
  }

  pub fn is_prime(n:usize)->bool
  {

    let m = n / 2;
    for i in 2..(m+1){

      if n % i == 0{
        return false;
      }

    }
  
    true
  } 

  pub fn prime_factors(n:usize)->Vec<usize>  
  {  
    let mut k = n;
    let mut output:Vec<usize> = Vec::new();

    // Print the number of 2s that divide n
    while k % 2 == 0
    {
      output.push(2);
      k = k / 2;
    }

    // n must be odd at this point. So we can skip  
    // one element (Note i = i +2)  
    let mut i = 3;
    while (i as f64) < f64::sqrt(k as f64)
    {
      while k % i == 0
      {
        output.push(i);
        k = k / i;
      }

      i = i + 2;
    }

    if k > 2
    {
      output.push(k);
    } 

    output 
  }  

  pub fn is_permutation(x:usize, y:usize)->bool{
    let s1 = x.to_string();
    let s2 = y.to_string();
    
    if s1.len() != s2.len()
    {
      return false;
    }
    
    for c1 in s1.chars(){
      if !s2.contains(c1){
        return false;
      }
    }

    for c2 in s2.chars(){
      if !s1.contains(c2){
        return false;
      }
    }

    true
  }

  pub fn is_num_palindrome(n:u128)->bool{
    if n < 10{
      return true;
    }

    let mut k = n;
    let mut r = num_reverse(n);

    while k > 0 && r > 0{
      let d1 = k % 10;
      let d2 = r % 10;
      if d1 != d2{
        return false;
      }

      k = k / 10;
      r = r / 10;
    }

    k == r
  }

  pub fn get_digit_count(n:u128)->usize{
    let mut c = 0;
    let mut r = n;
    while r > 0{
      r = r / 10;
      c += 1;
    }

    c
  }

  pub fn get_sum_of_digits(n:usize)->usize{
    let mut s = 0;
    let mut r = n;
    while r > 0{
      let d = r % 10;
      s += d;
      r = r / 10;
    }

    s
  }

  pub fn num_reverse(n:u128)->u128{
    if n < 10{
      return n;
    }

    let mut k = n;
    let mut r = 0;

    while k > 0{
      let d = k % 10;
      r = r * 10 + d;
      k = k / 10;
    }

    r
  }

  pub fn is_lychrel(n:usize)->bool{
    if n < 10{
      return false;
    }

    let mut k:u128 = n as u128;
    for _ in 0..50{
      let r = num_reverse(k);
      k = k + r;
      if is_num_palindrome(k){
        return false;
      }
    }
  
    true
  }

/////////////////////////////////////////////////////////////////////////////////////////////////

  #[test]
  fn test_it()
  {
    println!("{}", is_lychrel(349) );
    println!("{}", is_lychrel(9999) );
  }

  #[test]
  fn test_primes()
  {
    let v = gen_primes(11);
    println!("{:?}", v);

    assert!(v.len() == 5);
  }

  #[test]
  fn test_perm()
  {
    let mut c = 0;
    gen_permutation(&[1,2,3,4], &mut | arr |->bool
    {
      println!("{:?}", arr);
      c = c + 1;
      false
    });

    assert!(c == 1*2*3*4);
  }

  #[test]
  fn test_factor()
  {
    let t = prime_factors(644);
    assert!(t.len() == 4);
    assert!(t[0]==2 && t[1]==2 && t[2]==7 && t[3]==23);
  }

  #[test]
  fn test_prime()
  {
    let tst = is_prime(7);
    assert!(tst);

    let tst2 = is_prime(6);
    assert!(!tst2)
  }

  #[test]
  fn test_to_digits()
  {
    let n = 12872;
    let v = int_to_digits(n);
    println!("{:?}", v);
  }

  #[test]
  fn test_from_digits()
  {
    let j = 12872;
    let v = int_to_digits(j);
    let n = digits_to_int(&v);
    assert!(n == 12872);
  }

  #[test]
  fn test_pandigital()
  {
    let tst = is_pandigital(123456789);
    assert!(tst);

    let tst2 = is_pandigital(122456789);
    assert!(!tst2)
  }

  #[test]
  fn test_triangular()
  {
    for n in 1..10{
      let t = triangular(n);
      assert!(is_triangular(t));
    }
  }

  #[test]
  fn test_hexagonal()
  {
    for n in 1..10{
      let h = hexagonal(n);
      assert!(is_hexagonal(h));
    }
  }

  #[test]
  fn test_pentagonal()
  {
    for n in 1..10{
      let h = pentagonal(n);
      assert!(is_pentagonal(h));
    }
  }

  #[test]
  fn test_pandigital_capture()
  {
    let mut v:usize = 0;

    gen_pandigital(&mut |n : usize|->bool
    {
      v = n;
      return true;
    });
  
    let tst = is_pandigital(v);

    assert!(tst);
  }

  #[test]
  fn test_is_perm()
  {
    let tst = is_permutation(1234, 4213);
    assert!(tst);

    let tst2 = is_permutation(1013, 1307);
    assert!(!tst2);
  }

  #[test]
  fn test_factorial()
  {
    let f = factorial(5).unwrap_or(0);

    assert!(f == 120);
  }

  #[test]
  fn test_choose()
  {
    let p = choose(10, 5).unwrap();
    assert!(p == 252);
  }

}

