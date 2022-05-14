use core::any::type_name;
use mongodb::bson::{doc, Bson, Document};
use mongodb::sync::{Client, Database};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

// 定义DB增删改查接口
pub trait IExample<T>
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    fn insert(doc: T) -> Bson {
        let table_name = table_name::<T>();
        let collection = db().collection::<T>(table_name.as_str());
        collection.insert_one(doc, None).unwrap().inserted_id
    }

    fn update(filter: Document, doc: Document) -> u64 {
        let table_name = table_name::<T>();
        let collection = db().collection::<T>(table_name.as_str());
        collection
            .update_one(filter, doc, None)
            .unwrap()
            .modified_count
    }

    fn find(filter: Document) -> T {
        let table_name = table_name::<T>();
        let collection = db().collection::<T>(table_name.as_str());
        collection.find_one(filter, None).unwrap().unwrap()
    }

    fn delete(filter: Document) -> u64 {
        let table_name = table_name::<T>();
        let collection = db().collection::<T>(table_name.as_str());
        collection.delete_one(filter, None).unwrap().deleted_count
    }
}

// 定义Table1
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Table1 {
    pub title: String,
    pub author: String,
}

// 定义Table2
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Table2 {
    pub name: String,
    pub age: Option<u32>,
}

// 实现Table1增删改查
impl IExample<Table1> for Table1 {}
// 实现Table2增删改查
impl IExample<Table2> for Table2 {}

/**************************************************
 *  无关代码
 * ************************************************/
// 获取类型名称
fn table_name<T>() -> String {
    let table = type_name::<T>().split("::").last().unwrap();
    table.to_lowercase()
}
// 获取指定数据库
fn db() -> Database {
    // "mongodb://localhost:27017"
    let url = "mongodb://admin:X%5DpCY%29g8Hs%259hlss%2CY%5Di@localhost:27017";
    let client = Client::with_uri_str(url).unwrap();
    let database = client.database("mydb");
    database
}
