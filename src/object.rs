// object type for different object
#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
    Error,
}

// different object for evaluation
#[derive(Debug, Clone)]
pub enum Object {
    Null,          // Null
    Integer(i64),  // Int
    Boolean(bool), // Boolean
    Error(String), // Error message
}

// the method for Object
impl Object {
    // return a ObjectType
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(..) => ObjectType::Integer,
            Object::Boolean(..) => ObjectType::Boolean,
            Object::Null => ObjectType::Null,
            Object::Error(..) => ObjectType::Error,
        }
    }

    // for inspect string
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => format!("{}", value),
            Object::Boolean(boolean) => format!("{}", boolean),
            Object::Null => "null".to_string(),
            Object::Error(err) => format!("Error: {}", err),
        }
    }
}
