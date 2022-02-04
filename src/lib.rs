use switch_server::{Encode, Decode};
use std::io::{
    self,
    Read,
    Write,
    Seek
};

pub mod packets;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PacketTag {
    ItemSpawn,
    StateQuery,
    StateQueryResponse
}

impl Encode for PacketTag {
    fn encode<W>(&self, writer: &mut W) -> io::Result<()>
    where W: Write + Seek
    {
        match self {
            PacketTag::ItemSpawn => 0u8.encode(writer),
            PacketTag::StateQuery => 1u8.encode(writer),
            PacketTag::StateQueryResponse => 2u8.encode(writer)
        }    
    }
}

impl Decode for PacketTag {
    fn decode<R>(reader: &mut R) -> io::Result<Self>
    where R: Read + Seek
    {
        match u8::decode(reader)? {
            0 => Ok(Self::ItemSpawn),
            1 => Ok(Self::StateQuery),
            2 => Ok(Self::StateQueryResponse),
            x => Err(io::Error::new(io::ErrorKind::InvalidData, format!("PacketTag value '{}' is out of range of PacketTag!", x))),
        }    
    }
}

#[derive(Copy, Clone)]
pub enum Packet {
    ItemSpawn(packets::ItemSpawn),
    StateQuery(packets::StateQuery),
    StateQueryResponse(packets::StateQueryResponse),
}

impl Encode for Packet {
    fn encode<W>(&self, writer: &mut W) -> io::Result<()>
    where W: Write + Seek
    {
        match self {
            Self::ItemSpawn(item_spawn) => {
                PacketTag::ItemSpawn.encode(writer)?;
                item_spawn.encode(writer)
            },
            Self::StateQuery(state_query) => {
                PacketTag::StateQuery.encode(writer)?;
                state_query.encode(writer)
            },
            Self::StateQueryResponse(query_response) => {
                PacketTag::StateQueryResponse.encode(writer)?;
                query_response.encode(writer)
            }
        }    
    }
}

impl Decode for Packet {
    fn decode<R>(reader: &mut R) -> io::Result<Self>
    where R: Read + Seek
    {
        match PacketTag::decode(reader)? {
            PacketTag::ItemSpawn => packets::ItemSpawn::decode(reader).map(Self::ItemSpawn),
            PacketTag::StateQuery => packets::StateQuery::decode(reader).map(Self::StateQuery),
            PacketTag::StateQueryResponse => packets::StateQueryResponse::decode(reader).map(Self::StateQueryResponse)
        }    
    }
}