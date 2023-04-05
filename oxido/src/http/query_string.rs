// Cases:
// a=1&b=2&c&d=&e===&d=7&d=abc
pub mod query_string {
    use std::collections::HashMap;
    
    #[derive(Debug)]
    pub struct QueryString<'buffer> {
        data: HashMap<&'buffer str, Value<'buffer>>,
    }

    #[derive(Debug)]
    pub enum Value<'buffer> {
        Single(&'buffer str),
        // The "multiple" option should be a dynamic-sized array stored in the Heap.
        // For that Rust used Vectors.
        Multiple(Vec<&'buffer str>)
    }

    impl<'buffer> QueryString<'buffer>  {
        pub fn get(&self, key: &str) -> Option<&Value> {
            self.data.get(key)
        }
    }

    // a=1&b=2&c&d=&e===&d=7&d=abc
    impl<'buffer> From<&'buffer str> for QueryString<'buffer> {
        fn from(s: &'buffer str) -> Self {
            let mut data = HashMap::new();

            for sub_str in s.split('&') {
                let mut key = sub_str;
                let mut val = "";

                if let Some(i)= sub_str.find('=') {
                    key = &sub_str[..i];
                    val = &sub_str[i+1..];
                }

                data.entry(key)
                    .and_modify(|existing: &mut Value| match existing {
                        Value::Single(prev_val) => {
                            // Option 1: Simpler option, but three lines of code;
                            // let mut vec = Vec::new();
                            // vec.push(val);
                            // vec.push(prev_val);

                            // Option 2: Uses Rust's vec! macro. Creates new vect with values
                            // let mut vec = vec![prev_val, val];
                            *existing = Value::Multiple(vec![prev_val, val]);
                        }
                        Value::Multiple(vec) => vec.push(val)
                    })
                    .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
    }
}