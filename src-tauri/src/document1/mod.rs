use std::cmp::Ordering;
use tantivy::{Document, Index};
use tantivy::schema::Field;

pub struct Document1(pub(crate) Document);

fn transform(doc: &Document, field: Vec<Field>) -> Vec<String>{
    let mut list: Vec<String> = Vec::new();
    for i in field {
        let val = doc.get_first(i).expect("拿到属性列表").as_text().expect("拿到属性错误");
        list.push(val.to_string());
    }
    list
}



impl Eq for Document1 {}


impl Ord for Document1 {

    fn cmp(&self, other: &Self) -> Ordering {
        let index = Index::open_in_dir("index").unwrap();
        let schema = index.schema();
        let fields = vec![
            schema.get_field("name").unwrap(),
            schema.get_field("type").unwrap(),
            schema.get_field("creat").unwrap(),
            schema.get_field("modify").unwrap(),
        ];
        let list1 = transform(&self.0, fields.clone());
        let list2 = transform(&other.0, fields);
        match list1.cmp(&list2) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Greater
        }
    }
}
impl PartialEq<Self> for Document1 {
    fn eq(&self, other: &Self) -> bool {
        let index = Index::open_in_dir("index").unwrap();
        let schema = index.schema();
        let FIELDS = vec![
            schema.get_field("name").unwrap(),
            schema.get_field("type").unwrap(),
            schema.get_field("creat").unwrap(),
            schema.get_field("modify").unwrap(),
        ];
        transform(&other.0, FIELDS.clone()) == transform(&self.0, FIELDS)
    }
}

impl PartialOrd<Self> for Document1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let index = Index::open_in_dir("index").unwrap();
        let schema = index.schema();
        let FIELDS = vec![
            schema.get_field("name").unwrap(),
            schema.get_field("type").unwrap(),
            schema.get_field("creat").unwrap(),
            schema.get_field("modify").unwrap(),
        ];
        let list1 = transform(&self.0, FIELDS.clone());
        let list2 = transform(&other.0, FIELDS);
        match list1.cmp(&list2) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => Some(Ordering::Equal),
            Ordering::Greater => Some(Ordering::Greater)
        }
    }
}


