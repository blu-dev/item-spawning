use switch_server::{Encode, Decode};
use std::io::{
    self,
    Read,
    Write,
    Seek
};

#[derive(Copy, Clone)]
pub enum SpawnKind {
    Player(i32),
    Location(f32, f32)
}

impl Encode for SpawnKind {
    fn encode<W>(&self, writer: &mut W) -> io::Result<()>
    where W: Write + Seek
    {
        match self {
            Self::Player(idx) => {
                0u8.encode(writer)?;
                idx.encode(writer)
            },
            Self::Location(x, y) =>{
                1u8.encode(writer)?;
                x.encode(writer)?;
                y.encode(writer)
            }
        }    
    }
}

impl Decode for SpawnKind {
    fn decode<R>(reader: &mut R) -> io::Result<Self>
    where R: Read + Seek
    {
        let tag = u8::decode(reader)?;
        match tag {
            0 => i32::decode(reader).map(Self::Player),
            1 => {
                let x = f32::decode(reader)?;
                let y = f32::decode(reader)?;
                Ok(Self::Location(x, y))
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, format!("SpawnKind tag '{}' is out of range of SpawnKind", tag)))
        } 
    }
}

#[derive(Clone)]
pub struct ItemSpawn {
    item_kind_hash: u64,
    kind: SpawnKind
}

impl Encode for ItemSpawn {
    fn encode<W>(&self, writer: &mut W) -> io::Result<()>
    where W: Write + Seek
    {
        self.item_kind_hash.encode(writer)?;
        self.kind.encode(writer)    
    }
}

impl Decode for ItemSpawn {
    fn decode<R>(reader: &mut R) -> io::Result<Self>
    where R: Read + Seek
    {
        let item_kind_hash = u64::decode(reader)?;
        let kind = SpawnKind::decode(reader)?;
        Ok(Self {
            item_kind_hash,
            kind
        })    
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StateQuery {
    LastSummonFrame,
    IsFighterExist(i32),
}

impl Encode for StateQuery {
    fn encode<W>(&self, writer: &mut W) -> io::Result<()>
    where W: Write + Seek
    {
        match self {
            Self::LastSummonFrame => 0u8.encode(writer),
            Self::IsFighterExist(idx) => {
                1u8.encode(writer)?;
                idx.encode(writer)
            }
        }    
    }
}

impl Decode for StateQuery {
    fn decode<R>(reader: &mut R) -> io::Result<Self>
    where R: Read + Seek
    {
        let tag = u8::decode(reader)?;
        match tag {
            0 => Ok(Self::LastSummonFrame),
            1 => {
                let index = i32::decode(reader)?;
                Ok(Self::IsFighterExist(index))
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, format!("StateQuery tag '{}' is out of range for StateQuery!", tag)))
        }    
    }
}