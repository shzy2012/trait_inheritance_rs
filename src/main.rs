use mongodb::bson::doc;
use trait_inheritance_rs::example::{IExample, Table1, Table2};

fn main() {
    // 操作Table1
    let t1 = Table1 {
        title: "The Grapes of Wrath".to_string(),
        author: "John Steinbeck".to_string(),
    };
    Table1::insert(t1); //新增

    let filter = doc! {
       "title": "The Grapes of Wrath"
    };
    let get_t1 = Table1::find(filter); //查找
    println!("{:?}", &get_t1);

    // 操作Table2
    let t2 = Table2 {
        name: "xunzi".to_string(),
        age: Some(235),
    };
    Table2::insert(t2); //新增

    let filter = doc! {
       "name": "xunzi"
    };
    let update = doc! {
       "$set": {
           "age":Some(75),
       }
    };
    let get_t2 = Table2::update(filter, update); //更新
    println!("{:?}", &get_t2);

    println!("Hello, world!");
}
