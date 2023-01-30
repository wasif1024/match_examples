#[allow(dead_code)]
enum Color{
    Black,
    Red,
    Green,
    RGB(u32,u32,u32),
}
pub fn match_enum(){
let color_obj=Color::RGB(1, 2, 3);
match color_obj{
    Color::Red=>println!("Color s Red"),
Color::Black=>println!("Color s Black"),
Color::Green=>println!("Color s Green"),
Color::RGB(1,2 ,3 )=>println!("Color found"),
_=>println!("Nothing found"),
}
}