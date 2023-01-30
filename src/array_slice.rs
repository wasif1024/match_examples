pub fn match_array(){
    let array=[5,2,6];
    match array{
        [1,second,third]=>println!("First 1 second {:?},third {:?}",second,third),
        [4, ..]  => println!("First is `4` and the rest doesn't matter"),
        [5, second,6]  => println!("First is `5` last is 6 rest {}",second),
        _=>println!("Nothing matched"),
    }
    }