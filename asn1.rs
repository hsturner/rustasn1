/*    Rust Assignment 1: Struggling Against The Borrow Checker

This assignment will require you to first read the sample programs
that I've given you, especially the "what you must know first"
program, plus code demonstrated in class (bubblesort), code given
here, as well online documentation on various aspects of Rust,
especially vectors (google vectors in rust).

The assignment asks you to write a couple of functions on vectors in Rust.
But it's important you read and understand the samples and hints I've
provided.
*/

/* optional:
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
*/
use std::mem::{swap,replace,take};   // these might be useful

// The following generic function takes a borrow of a vector of :Copy values 
// and returns the reverse of it (non-destructively).
fn rev_copy<T:Copy>(v:&Vec<T>) -> Vec<T>
{
   let mut rev = Vec::new(); // rust should be able to infer the type
   let mut i = v.len(); // prepare backwards loop
   while i>0 { rev.push(v[i-1]); i-=1; }
   rev  // do not put ; here
}
// Note that the type of i is usize, an unsigned integer type, which is
// why the while loop is written the way it is.  In other languages
// you might write this as while i>=0, which will stop when i becomes -1.
// But an unsigned value can't become -1 (Rust will panic).

// But what if T can't be assumed to implement the Copy trait?  To
// reverse a list, we have two choices:
fn rev1<T>(v:&mut Vec<T>) -> Vec<T>
{
  let mut rv = Vec::new();
  while v.len()>0 {
    rv.push( v.pop().unwrap() );
  }//while
  rv // don't add ; here
}
/*
  This (above) function takes a mutable borrow to a vector and erases it by
  calling .pop, placing the new values in the new vector in reverse order.
  It returns ownership of the locally constructed vector rv.
  Note that v.pop() returns a value of type Option<T>, because v can be
  empty.  v.pop().unwrap() assumes that v is non-empty returns the value.
  .unwrap() will panic if v was empty, but we've checked that v.len()>0.

  Rust does not use exceptions, which can't be statically checked.  Instead
  it uses the Option and Result "monads" for safe error-handling (similar
  to the Maybe type in elm).  Un-handled errors results in a panic!.
*/

// But what if we don't want to destroy the original vector?  How can
// I create a reverse "copy" of the vector when the values can't be copied?
// I can create instead a vector of borrows, arranged in reverse order.
fn flip<T>(v:&Vec<T>) -> Vec<&T>
{
  let mut rv = Vec::new(); // hopefully can infer type of vector
  let mut i=v.len();
  while (i>0) { rv.push(&v[i-1]); i-=1; }
  rv
}
/* This function takes a borrow of a vector and returns a vector of
 borrows.  Each borrow in the returned vector refers to value of the
 original vector.  This in most cases is as good as a non-destructive
 "copy" of the vector.

//Note: don't attempt to write this function in the following way:
fn flip2<T>(v:Vec<T>) -> Vec<&T>
{
  let mut rv = Vec::new(); 
  let mut i=v.len();
  while (i>0) { rv.push(&v[i-1]); i-=1; }
  rv
}
//This will give one of the most dreaded compiler errors in Rust:

             "expected named lifetime parameter"

Advice to beginners: if you see this error, ABANDON ALL HOPE AND START OVER.
If you write a function that returns borrows, remember that these borrows
cannot outlive the values that they're borrowing.  Since v is a local var
that owns the Vec<T>, its lifetime cannot possibly extend beyond the function
call.  So returning a borrow to any part of it would be futile.  Forget
about trying to use lifetime parameters to get it to work: you won't succeed.
However, the above flip function is OK because rust will infer that the
lifetime of the borrows in the returned vector is the same as the lifetime
of the borrowed values in *v.

However, there are also situations that require you to declare lifetime
variables.  Consider the following function, which finds the intersection
of two vectors: it returns a vector containing borrows of all values that
appear in both vectors v1 and v2:
*/
fn intersection<'t,'u,T:Eq>(v1:&'t Vec<T>, v2:&'u Vec<T>) -> Vec<&'t T>
{
    let mut iv = Vec::new(); // intersection to be returned
    for x in 0..v1.len()
    {
       for y in 0..v2.len()
       {
          if &v1[x]==&v2[y] { iv.push(&v1[x]); } // can it be iv.push(&v2[y])?
       }
    }
    return iv;  // same as iv without ;
}//intersection
/*
   The trait Eq allows you to use the == symbol between two values.  The
   generic variables 't and 'u are LIFETIME parameters.  They allow you
   to say more accurately what is the lifetime of the arguments and return
   value of the function.  Notice how we contructed the intersection vector:
   all values are references to the first vector (v1), and so the return
   type is Vec<&'t T> and not Vec<'u T>.

   Be careful: lifetime parameters do not allow you to somehow "extend" the
   lifetime of a borrow.  The lifetime of something is at most the static
   scope in which it appears, then it's dropped (deallocated from memory):
   it won't exist and you can't make it exist again with some magical
   declaration.  If you changed the return type to Vec<&'u T> it won't
   compile: you can't just "allow" your returned borrows to live longer.
*/



/*   ////////////////// YOUR ASSIGNMENT /////////////////////

1. Write a function 'flap', in the style of flip, that reverses the
left half and the right half of a vector (of non-:COPY values and non
:Clone). The idea is that flap([a,b,c,d,e,f]) will give [c,b,a,f,e,d].
If there's a middle value, it should stay in place: flap([a,b,c,d,e])
should give [b,a,c,e,d].  However, you will have to return a vector of
references, just as flip does.  The function you write MUST have the
following signature (name and type):

   fn flap<T>(v:&Vec<T>) -> Vec<&T>


2. Write a function with the following signature:

   fn mutflip<T>(v:&mut Vec<T>) -> Vec<&mut T>

   This function should work like flip, but takes a mutable borrow of a
   vector and returns a vector of mutable borrows of the values in v in
   reverse.  We should be able to use these 'ref muts' to make destructive
   changes to the original vector (see main).

   Hint: if you don't have to reverse the ordering you can just do
   for x in v { mv.push(&mut *x) }, where mv is the vector you want to return.

   let mut x = 2;
   let mx = &mut x;

3. Write a function (similar to intersection) that returns the UNION of two
vectors.  FURTHERMORE, the union may not contain borrows of duplicate
value.  You will have to figure out the signature of the function (first line)
but you MAY NOT assume the :Copy or :Clone traits!  Your function must work
on vectors of generic type T:Eq, just like in the intersection function.

Hint: first write a function to count how many times a value is contained
inside a vector.  This function had better take borrows and return usize.

*/

/* this won't compile until you've completed the exercises:

/////////// The following function must be called from main and
/// MAY NOT BE MODIFIED.
fn test() 
{
  let strs = ["c","kotlin","scala","perl","f#","c#","rust","c++","mongoose"];
  let mut v= Vec::<String>::new();
  for s in strs.iter() { v.push((*s).to_owned()) }
  // note: strs is a vector containing values of type &str, which is a
  // "string slice", which can be copied so *s is safe.  But type String
  // (produced by .to_owned()) can't be copied.
  for s in &v { print!("{} ",s) }  println!();
  let flipv = flip(&v);
  {
  let flapv = flap(&v);
  for s in &flipv { print!("{} ",s) }  println!();
  for s in &flapv { print!("{} ",s) }  println!();
  } // these braces limit the lifetime of flapv
  let mut rv = mutflip(&mut v);
  // the following loop changes v using mut borrows in rv:
  for i in 0..rv.len() { *(rv[i]) = (i.to_string()+"abc").to_owned(); }
  for s in &v { print!("{} ",s) }  println!();
  let flapv = flap(&v);  
  for s in &flapv { print!("{} ",s) }  println!();

  let v1 = vec!["ab","cd","ab","xz"];
  let v2 = vec!["ef","ab","cd","gh"];
  let rv3 = union(&v1,&v2);
  println!("{:?}",rv3); //{:?} prints anything that implements Debug trait
} // DO NOT MODIFY THIS FUNCTION

*/


fn main()
{
    //test();  // uncomment when test() compiles
    // you may put additional tests in main.
}//main

/*
EXPECTED OUTPUT OF PROGRAM: (should match yours exactly)

c kotlin scala perl f# c# rust c++ mongoose
mongoose c++ rust c# f# perl scala kotlin c
perl scala kotlin c f# mongoose c++ rust c#
8abc 7abc 6abc 5abc 4abc 3abc 2abc 1abc 0abc
5abc 6abc 7abc 8abc 4abc 0abc 1abc 2abc 3abc
["ab", "cd", "xz", "ef", "gh"]
*/
