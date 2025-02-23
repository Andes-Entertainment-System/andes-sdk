use std::{fs, io::Write};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use super::ResCompilerArgs;

#[derive(Serialize, Deserialize)]
pub struct PaletteDef {
    id: String,
    path: String,
}

pub fn compile(
    ResCompilerArgs {
        ref mut header_buffer,
        ref mut source_buffer,
        res_config,
        res_path,
        ..
    }: &mut ResCompilerArgs,
) -> anyhow::Result<()> {
    header_buffer.write_all(b"\n// ---- palettes ----\n")?;
    source_buffer.write_all(b"\n// ---- palettes ----\n")?;

    for item in res_config.palettes.iter() {
        let content = fs::read_to_string(res_path.join(&item.path))?;
        let content_lines = content.split('\n');

        let mut color_amount = 0;

        source_buffer.write_fmt(format_args!(
            "const unsigned char DATA_{}[][3] = {{ ",
            item.id
        ))?;

        for line in content_lines.skip(3) {
            if line.is_empty() {
                continue;
            }

            let channels: Vec<u16> = line
                .split(' ')
                .map(|x| {
                    x.parse::<u16>()
                        .with_context(|| format!("Failed to parse palette \"{}\"", item.id))
                })
                .collect::<anyhow::Result<Vec<u16>>>()?;

            let color = (channels[0] >> 2, channels[1] >> 2, channels[2] >> 2);

            source_buffer.write_fmt(format_args!(
                "{{ 0x{:x}, 0x{:x}, 0x{:x} }}, ",
                color.0, color.1, color.2
            ))?;
            color_amount += 1;
        }

        source_buffer.write_fmt(format_args!(" }};\n",))?;

        header_buffer.write_fmt(format_args!("extern PaletteResource RES_{};\n", item.id))?;
        source_buffer.write_fmt(format_args!(
            "PaletteResource RES_{} = {{ .data = DATA_{}, .size = {} }};\n",
            item.id,
            item.id,
            color_amount * 3
        ))?;
    }

    Ok(())
}
