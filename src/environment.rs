use crate::object::Object;
use ahash::AHashMap as HashMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct Environment {
    // 存储环境变量当前作用域
    store: HashMap<String, Object>,
    // 外部环境变量
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    // 初始化函数
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(v) = self.store.get(name) {
            Some(v.clone())
        } else {
            None
        }
    }

    pub fn set(&mut self, name: &str, value: Object) {
        self.store.insert(name.to_string(), value);
    }
}
