// object type for different object
#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
}

// different object for evaluation
#[derive(Debug, Clone, Copy)]
pub enum Object {
    Integer(i64),  // Int
    Boolean(bool), // Boolean
    None,          // Null
}

// the method for Object
impl Object {
    // return a ObjectType
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(..) => ObjectType::Integer,
            Object::Boolean(..) => ObjectType::Boolean,
            Object::None => ObjectType::Null,
        }
    }

    // for inspect string
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => format!("{}", value),
            Object::Boolean(boolean) => format!("{}", boolean),
            Object::None => "null".to_string(),
        }
    }
}
