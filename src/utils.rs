use std::io::{Read, Result, Write};

pub fn write_u16<W>(mut wtr: W, data: u16) -> Result<()>
where
    W: Write,
{
    wtr.write_all(&data.to_le_bytes())?;
    Ok(())
}

pub fn read_u16<R>(mut rdr: R) -> Result<u16>
where
    R: Read,
{
    let mut bytes = [0; 2];
    rdr.read_exact(&mut bytes)?;
    Ok(u16::from_le_bytes(bytes))
}

pub fn write_i16<W>(mut wtr: W, data: i16) -> Result<()>
where
    W: Write,
{
    wtr.write_all(&data.to_le_bytes())?;
    Ok(())
}

pub fn read_i16<R>(mut rdr: R) -> Result<i16>
where
    R: Read,
{
    let mut bytes = [0; 2];
    rdr.read_exact(&mut bytes)?;
    Ok(i16::from_le_bytes(bytes))
}
