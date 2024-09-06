use storage::driver::{Local, S3};



mod storage;



fn main() {
    println!("Hello, world!");
    
    let l = Box::new(Local{});
   
    let s = Box::new(S3{});

    let ls = storage::new(l);

    let s3s = storage::new(s);

    let _ = ls.write(".");
    let _ = s3s.write(".");

    let _ = ls.read(".");
    let _ = s3s.read(".");
}


