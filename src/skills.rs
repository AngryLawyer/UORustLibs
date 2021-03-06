//! Skill objects represent named skills that appear in UO's Skills menu.
//! They also contain a flag denoting whether they are clicked to activate

use mul_reader::MulReader;
use std::ffi::CString;
use std::io::{Read, Result, Seek};
use std::path::Path;
use std::str::from_utf8;

pub struct Skill {
    pub clickable: bool,
    pub name: String,
}

impl Skill {
    pub fn new(clickable: bool, name: String) -> Skill {
        Skill {
            clickable: clickable,
            name: name,
        }
    }

    /**
     * Convert a skill back into its canonical mul representation
     */
    pub fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![if self.clickable { 1 } else { 0 }];
        let name = CString::new(self.name.clone()); //FIXME: There must be a way to do this without copying?
        vec.extend_from_slice(name.unwrap().as_bytes_with_nul());
        vec
    }
}

/// A reader of a Skills file-like object
///
/// Skills are traditionally stored in skills.mul/skills.idx
///
/// Skills are encoded as a list of
/// |clickable:u8|name:c-string|
pub struct Skills {
    pub skills: Vec<Skill>,
}

impl Skills {
    pub fn from_mul<T: Seek + Read>(reader: &mut MulReader<T>) -> Skills {
        //Unpack the lot
        let mut result = vec![];
        let mut id = 0;

        loop {
            match reader.read(id) {
                Ok(record) => {
                    let slice = &record.data[1..record.data.len() - 1];
                    result.push(Skill::new(
                        record.data[0] == 1,
                        String::from(from_utf8(slice).unwrap()),
                    )); //FIXME: Don't unwrap
                }
                _ => {
                    break;
                }
            }
            id += 1;
        }

        Skills { skills: result }
    }

    pub fn new(index_path: &Path, mul_path: &Path) -> Result<Skills> {
        let maybe_reader = MulReader::new(index_path, mul_path);
        match maybe_reader {
            Ok(mut reader) => Ok(Skills::from_mul(&mut reader)),
            Err(io_error) => Err(io_error),
        }
    }
}
