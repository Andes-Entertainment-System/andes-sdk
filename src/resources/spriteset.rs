use std::{
    fs::File,
    io::{BufWriter, Seek, SeekFrom, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use super::ResConfig;

#[derive(Clone, Serialize, Deserialize)]
struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SplitMode {
    None,
    Grid,
    Manual,
}

impl Default for SplitMode {
    fn default() -> Self {
        SplitMode::None
    }
}

#[derive(Serialize, Deserialize, Default)]
struct SplitOptions {
    #[serde(default)]
    mode: SplitMode,
    #[serde(default)]
    grid_cols: usize,
    #[serde(default)]
    grid_rows: usize,
    #[serde(default)]
    manual_frames: Vec<Rect>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SpriteDefSettings {
    #[serde(default)]
    split: SplitOptions,
}

/*
impl Default for SpriteDefSettings {
    fn default() -> Self {
        Self {
            split: SplitOptions {
                mode: SplitMode::None,
                grid_,
            },
        }
    }
}*/

#[derive(Serialize, Deserialize)]
pub struct SpriteSetDef {
    id: String,
    path: String,
    settings_path: Option<String>,
    #[serde(default)]
    settings: SpriteDefSettings,
}

fn convert_sprite(bytes: &[u8], region: Rect, image_width: usize) -> Vec<u8> {
    let Rect {
        x,
        y,
        width,
        height,
    } = region;
    let mut out: Vec<u8> = Vec::new();
    let mut trans_pixel_count = 0;

    for py in y..(y + height) {
        for px in x..(x + width) {
            let pixel = bytes[px + py * image_width];

            if pixel == 0 {
                trans_pixel_count += 1;
                if trans_pixel_count == 127 || px == x + width - 1 {
                    out.push(trans_pixel_count | 128);
                    trans_pixel_count = 0;
                }
            } else {
                if trans_pixel_count > 0 {
                    out.push(trans_pixel_count | 128);
                    trans_pixel_count = 0;
                }
                out.push(pixel);
            }
        }
    }

    out
}

pub fn compile(
    res_path: &PathBuf,
    res_config: &ResConfig,
    data_buffer: &mut BufWriter<File>,
    header_buffer: &mut BufWriter<File>,
    source_buffer: &mut BufWriter<File>,
) -> anyhow::Result<()> {
    header_buffer.write_all(b"\n// ---- spritesets ----\n")?;
    source_buffer.write_all(b"\n// ---- spritesets ----\n")?;

    for item in res_config.spritesets.iter() {
        let decoder = png::Decoder::new(File::open(res_path.join(&item.path))?);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        let bytes = &buf[..info.buffer_size()];
        let width = info.width as usize;
        let height = info.height as usize;

        let data_address = data_buffer.seek(SeekFrom::Current(0))?;

        header_buffer.write_fmt(format_args!(
            "extern const struct SpriteSetFrame FRAMES_{}[];\n",
            item.id
        ))?;
        source_buffer.write_fmt(format_args!(
            "const struct SpriteSetFrame FRAMES_{}[] = {{\n",
            item.id
        ))?;

        let settings = match &item.settings_path {
            Some(path) => &serde_yml::from_reader(File::open(res_path.join(path))?).unwrap(),
            None => &item.settings,
        };

        match &settings.split.mode {
            SplitMode::None => {
                source_buffer.write_fmt(format_args!(
                    "  {{ .offset = 0, .width = {}, .height = {} }},\n",
                    width, height
                ))?;

                data_buffer.write_all(&convert_sprite(
                    bytes,
                    Rect {
                        x: 0,
                        y: 0,
                        width,
                        height,
                    },
                    width,
                ))?;
            }
            SplitMode::Grid => {
                let sprite_width = width / settings.split.grid_cols;
                let sprite_height = height / settings.split.grid_rows;

                for y in (0..height).step_by(sprite_height) {
                    for x in (0..width).step_by(sprite_width) {
                        source_buffer.write_fmt(format_args!(
                            "  {{ .offset = {}, .width = {}, .height = {} }},\n",
                            data_buffer.seek(SeekFrom::Current(0))? - data_address,
                            sprite_width,
                            sprite_height
                        ))?;

                        data_buffer.write_all(&convert_sprite(
                            bytes,
                            Rect {
                                x,
                                y,
                                width: sprite_width,
                                height: sprite_height,
                            },
                            width,
                        ))?;
                    }
                }
            }
            SplitMode::Manual => {
                for region in settings.split.manual_frames.iter() {
                    source_buffer.write_fmt(format_args!(
                        "  {{ .offset = {}, .width = {}, .height = {} }},\n",
                        data_buffer.seek(SeekFrom::Current(0))? - data_address,
                        region.width,
                        region.height
                    ))?;

                    data_buffer.write_all(&convert_sprite(bytes, region.clone(), width))?;
                }
            }
        }

        header_buffer.write_fmt(format_args!(
            "extern struct SpriteSetResource RES_{};\n",
            item.id
        ))?;
        source_buffer.write_fmt(format_args!(
            "}};\nstruct SpriteSetResource RES_{} = {{ .address = {}, .size = {}, ",
            item.id,
            data_address,
            data_buffer.seek(SeekFrom::Current(0))? - data_address,
        ))?;
        source_buffer.write_fmt(format_args!(".frames = FRAMES_{} }};\n\n", item.id,))?;
    }

    Ok(())
}
