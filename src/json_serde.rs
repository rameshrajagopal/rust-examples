#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    count: i32,
    result: String,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    let response = Response { count: 10, result : "Success".to_owned() };
    let ser_res  = serde_json::to_string(&response).unwrap();
    println!("serialized = {}", ser_res);

    let deserialized: Response = serde_json::from_str(&ser_res).unwrap();
    println!("deserialized= {:?}", deserialized);
}
