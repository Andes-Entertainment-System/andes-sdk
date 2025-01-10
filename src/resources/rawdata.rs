use std::{
    fs::{self, File},
    io::{BufWriter, Seek, SeekFrom, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use super::ResConfig;

#[derive(Serialize, Deserialize)]
pub struct RawDataDef {
    id: String,
    path: String,
}

pub fn compile(
    res_path: &PathBuf,
    res_config: &ResConfig,
    data_buffer: &mut BufWriter<File>,
    header_buffer: &mut BufWriter<File>,
    source_buffer: &mut BufWriter<File>,
) -> anyhow::Result<()> {
    header_buffer.write_all(b"\n// ---- rawdata ----\n")?;
    source_buffer.write_all(b"\n// ---- rawdata ----\n")?;

    for item in res_config.rawdata.iter() {
        let data_address = data_buffer.seek(SeekFrom::Current(0))?;
        data_buffer.write_all(&fs::read(res_path.join(&item.path))?)?;
        header_buffer.write_fmt(format_args!(
            "extern struct RawDataResource RES_{};\n",
            item.id
        ))?;
        source_buffer.write_fmt(format_args!(
            "struct RawDataResource RES_{} = {{ .address = {}, .size = {} }};\n",
            item.id,
            data_address,
            data_buffer.seek(SeekFrom::Current(0))? - data_address
        ))?;
    }

    Ok(())
}
