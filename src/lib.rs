mod marshal;
mod objects;
mod processor;
mod sandbox;
mod stack;

use std::fmt;
use std::io;

#[derive(Debug)]
pub enum InterpreterError {
    Io(io::Error),
    Unmarshal(marshal::decode::UnmarshalError),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InterpreterError::Io(ref e) => write!(f, "I/O error:").and_then(|_| e.fmt(f)),
            InterpreterError::Unmarshal(ref e) => write!(f, "Unmarshal error:").and_then(|_| e.fmt(f)),
        }
    }
}

pub fn run_module<R: io::Read, EP: sandbox::EnvProxy>(reader: &mut R, envproxy: &mut EP) -> Result<(), InterpreterError> {
    let mut buf = [0; 12];
    try!(reader.read_exact(&mut buf).map_err(InterpreterError::Io));
    // TODO: do something with the content of the buffer
    let mut store = objects::ObjectStore::new();
    let module = try!(marshal::read_object(reader, &mut store).map_err(InterpreterError::Unmarshal));
    processor::run_code_object(envproxy, &mut store, module);
    Ok(())
}

#[test]
fn test_hello_world() {
    let mut reader: &[u8] = b"\xee\x0c\r\n\x15j\nW\x15\x00\x00\x00\xe3\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00@\x00\x00\x00s\x0e\x00\x00\x00e\x00\x00d\x00\x00\x83\x01\x00\x01d\x01\x00S)\x02z\x0bHello worldN)\x01\xda\x05print\xa9\x00r\x02\x00\x00\x00r\x02\x00\x00\x00\xfa\x0b/tmp/foo.py\xda\x08<module>\x01\x00\x00\x00s\x00\x00\x00\x00";
    let mut envproxy = sandbox::MockEnvProxy::new();
    run_module(&mut reader, &mut envproxy).unwrap();
    assert_eq!(*envproxy.stdout_content.lock().unwrap(), b"Hello world\n");
}
