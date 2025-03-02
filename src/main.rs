use std::env;

fn main() { 
 /*
  for x in  env::args() {  
   println!("{x}") ; 
   } 
*/ 

 let  args:Vec<String>   = env::args().collect()  ;   
 

 if  args.len() < 3  {  
 panic!("Number of arguments should be atleast then 3"); 
 } 


let mut  i = 0 ; 
/*
for elem  in &args  {  
 println!(" at index   {}  , the number is {}"  , i ,  elem) ; 
 i+=1 ;  
} */ 

let num1:&i32  = &args[1].parse().expect("First argument must be a   valid number") ;
let num2:&i32  = &args[3].parse().expect("Last argument must be a valid number");  

 let operator  = &args[2] ; 
// println!(" The operator is:{}" ,  operator  ) ; 
 match operator.as_str() {  
  "+"  => {   let ans = num1  + num2  ;   println!( "{ans}" ) ;  } , 
  "-"  => {  let ans = num1  -  num2  ;   println!( "{ans}" ) ; } ,
  "X"  => {  let ans =  num1  *  num2  ;   println!( "{ans}" ) ; }   ,
  "/"  =>{  let ans =   num1  / num2   ;   println!( "{ans}" ) ;  } , 
  &_ => {  println!("invalid operator provided") ; } 
 } 
 }
