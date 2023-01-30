pub fn match_tuple(){
let triple=(5,2,6);
match triple{
    (1,second,third)=>println!("First 1 second {:?},third {:?}",second,third),
    (4, ..)  => println!("First is `4` and the rest doesn't matter"),
    (5, second,6)  => println!("First is `5` last is 6 rest {}",second),
    _=>println!("Nothing matched"),
}
}