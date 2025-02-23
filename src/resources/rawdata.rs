use std::{
    fs,
    io::{Seek, SeekFrom, Write},
};

use serde::{Deserialize, Serialize};

use super::ResCompilerArgs;

#[derive(Serialize, Deserialize)]
pub struct RawDataDef {
    id: String,
    path: String,
}

pub fn compile(
    ResCompilerArgs {
        ref mut header_buffer,
        ref mut data_buffer,
        ref mut source_buffer,
        res_config,
        res_path,
        ..
    }: &mut ResCompilerArgs,
) -> anyhow::Result<()> {
    header_buffer.write_all(b"\n// ---- rawdata ----\n")?;
    source_buffer.write_all(b"\n// ---- rawdata ----\n")?;

    for item in res_config.rawdata.iter() {
        let data_address = data_buffer.seek(SeekFrom::Current(0))?;
        data_buffer.write_all(&fs::read(res_path.join(&item.path))?)?;
        header_buffer.write_fmt(format_args!("extern RawDataResource RES_{};\n", item.id))?;
        source_buffer.write_fmt(format_args!(
            "RawDataResource RES_{} = {{ .address = {}, .size = {} }};\n",
            item.id,
            data_address,
            data_buffer.seek(SeekFrom::Current(0))? - data_address
        ))?;
    }

    Ok(())
}
