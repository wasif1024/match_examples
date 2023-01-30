pub struct MatchingStruct{
    name:String,
    tuple_test:(u32,String)
}
pub fn match_struct(){
let struct_obj=MatchingStruct{name:String::from("wasif"),tuple_test:(123,String::from("test name"))};
let _name_string=String::from("wasif");
let _test_name=String::from("test name");
match struct_obj {
    MatchingStruct{name:_name_string,tuple_test:(123,_test_name)}=>println!("found"),
    _=>println!("Nothing found")
}
}