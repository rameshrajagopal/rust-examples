
struct Response {
    res : String,
}

impl Response {
    fn new() -> Response {
        Response {
            res: String::new(),
        }
    }

    fn push(&mut self, chunk: String) {
        self.res.push_str(&chunk);
    }

    fn get(&self) -> &String {
        &self.res
    }
}

fn main() {
    let mut response = Response::new();
    response.push("HelloWorld".to_owned());
    response.push(" my dear world".to_owned());
    println!("{}", response.get());
}
