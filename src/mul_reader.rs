use std::io;
use std::io::ReaderUtil;
use std::option;
use std::result;
use std::path;
use std::u32;

static undef_record:u32 = 0xFEFEFEFF;
static INDEX_SIZE: uint = 12;

pub struct MulRecord {
    data: ~[u8],
    start: u32,
    length: u32,
    opt1: u16,
    opt2: u16
}

pub struct MulReader {
    idx_reader: @io::Reader,
    data_reader: @io::Reader
}

impl MulReader {

    pub fn new(idx_path: &path::Path, mul_path: &path::Path) -> result::Result<MulReader, ~str> {
        io::file_reader(idx_path).chain(|idx_reader| {
            io::file_reader(mul_path).chain(|data_reader| {
                result::Ok(MulReader {
                    idx_reader: idx_reader,
                    data_reader: data_reader
                })
            })
        })
    }

    pub fn read(&self, index: uint) -> option::Option<MulRecord> {
        //Wind the idx reader to the index position
        self.idx_reader.seek((index * INDEX_SIZE) as int, io::SeekSet);

        let idx_reader = self.idx_reader;
        
        let start: u32 = idx_reader.read_le_uint_n(4) as u32;
        let length: u32 = idx_reader.read_le_uint_n(4) as u32;
        let opt1: u16 = idx_reader.read_le_uint_n(2) as u16;
        let opt2: u16 = idx_reader.read_le_uint_n(2) as u16;

        //Check for empty cell
        if (start == undef_record || start == u32::max_value) { 
            //error!("Trying to read out of bounds record %u, with a start of %u", index, start as uint);
            return option::None;
        };
        
        self.data_reader.seek(start as int, io::SeekSet);
        let reader_util = self.data_reader; 
        return option::Some(MulRecord {
            data: reader_util.read_bytes(length as uint),
            start: start,
            length: length,
            opt1: opt1,
            opt2: opt2
        });
    }
}
