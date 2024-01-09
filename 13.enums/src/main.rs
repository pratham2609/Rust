// Defining enums
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// const home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// const loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };


// <T> means it can hold data of any type
enum Option<T> {
    None,
    Some(T),
}

fn main(){
    let some_number = Some(5);
    let some_char = Some('e');


    let absent_number: Option<i32> = Option::<i32>::None; // we have to specify its type
}