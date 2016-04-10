
#[derive(Debug)]
pub struct Code {/*
    pub argcount: u32,
    pub kwonlyargcount: u32,
    pub nlocals: u32,
    pub stacksize: u32,
    pub flags: u32,*/
    pub code: ObjectRef,/*
    pub consts: Object,
    pub names: Object,
    pub varnames: Object,
    pub freevars: Object,
    pub cellvars: Object,
    pub filename: Object,
    pub name: Object,
    pub firstlineno: u32,
    pub lnotab: Object,*/
}

#[derive(Debug)]
pub enum ObjectContent {
    None,
    True,
    False,
    Int(u32),
    String(::std::string::String),
    Tuple(Vec<ObjectRef>),
    List(Vec<ObjectRef>),
    Code(Code),
    Set(Vec<ObjectRef>),
    FrozenSet(Vec<ObjectRef>),
    Bytes(Vec<u8>),
}

#[derive(Debug)]
pub struct Object {
    content: ObjectContent,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct ObjectRef {
    index: usize,
}

#[derive(Debug)]
pub struct ObjectStore {
    all_objects: Vec<Object>,
}

impl ObjectStore {
    pub fn new() -> ObjectStore {
        ObjectStore { all_objects: Vec::new() }
    }

    pub fn allocate(&mut self, obj: ObjectContent) -> ObjectRef {
        let obj_ref = ObjectRef { index: self.all_objects.len() };
        self.all_objects.push(Object { content: obj });
        obj_ref
    }
}
