use std::{
    fs,
    io::{Seek, SeekFrom, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use super::ResCompilerArgs;

#[derive(Serialize, Deserialize)]
pub struct AudioDef {
    id: String,
    path: PathBuf,
}

pub fn compile(
    ResCompilerArgs {
        ref mut header_buffer,
        ref mut data_buffer,
        ref mut source_buffer,
        res_config,
        ..
    }: &mut ResCompilerArgs,
) -> anyhow::Result<()> {
    header_buffer.write_all(b"\n// ---- audio ----\n")?;
    source_buffer.write_all(b"\n// ---- audio ----\n")?;

    for item in res_config.audio.iter() {
        let data_address = data_buffer.seek(SeekFrom::Current(0))?;
        data_buffer.write_all(&fs::read(&item.path)?)?;
        header_buffer.write_fmt(format_args!("extern AudioResource RES_{};\n", item.id))?;
        source_buffer.write_fmt(format_args!(
            "AudioResource RES_{} = {{ .address = {}, .size = {} }};\n",
            item.id,
            data_address,
            data_buffer.seek(SeekFrom::Current(0))? - data_address
        ))?;
    }

    Ok(())
}
