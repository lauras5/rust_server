use std::collections::HashMap;

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value <'buf>{
    // 2 variants
    Single(&'buf str),
    Multiple(Vec<&'buf str>) // specify len of arr at compile, need to heap allocate to grow dynamically
}

impl <'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key) // return none if key doesn't exist
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> { // using from -> conversion can't fail
    fn from(s: &'buf str) -> Self {
        // empty hashmap
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = s.find('=') { // return option w/ index
                
            }
        }
        QueryString {data}
    }
}