use std::fs::{self, File};

use png::Transformations;
use regex::Regex;
use tar::Archive;

use crate::util::FigResult;

pub struct AssetLoader<'a> {
    dir: &'a str,
    tar: bool,
}

impl<'a> AssetLoader<'a> {
    pub fn from_tar(tar: &'a str) -> Self {
        AssetLoader {
            dir: tar,
            tar: true,
        }
    }

    pub fn from_dir(dir: &'a str) -> Self {
        AssetLoader {
            dir,
            tar: false,
        }
    }

    pub fn load_png(&mut self, asset: &str) -> FigResult<Vec<u32>> {
        if self.tar {
            let mut archive = Archive::new(File::open(self.dir)?);
            for file in archive.entries()? {
                let file = file?;
                if &file.path()?.to_str().expect("oops")[2..] == asset {
                    let mut decoder = png::Decoder::new(file);
                    decoder.set_transformations(Transformations::ALPHA);
                    let mut reader = decoder.read_info()?;
                    let mut u8_buf = vec![0; reader.output_buffer_size()];
                    reader.next_frame(&mut u8_buf)?;
                    
                    let mut buf = vec![0u32; reader.output_buffer_size()];
                    for (rgba, argb) in u8_buf.chunks_mut(4).zip(buf.iter_mut()) {
                        let r = rgba[0] as u32;
                        let g = rgba[1] as u32;
                        let b = rgba[2] as u32;
                        let a = rgba[3] as u32;

                        *argb = a << 24 | r << 16 | g << 8 | b;
                    }

                    return Ok(buf);
                }
            }
        } else {
            let file = File::open(format!("{}/{}", self.dir, asset))?;
            let mut decoder = png::Decoder::new(file);
            decoder.set_transformations(Transformations::ALPHA);
            let mut reader = decoder.read_info()?;
            let mut u8_buf = vec![0; reader.output_buffer_size()];
            reader.next_frame(&mut u8_buf)?;
            
            let mut buf = vec![0u32; reader.output_buffer_size()];
            for (rgba, argb) in u8_buf.chunks_mut(4).zip(buf.iter_mut()) {
                let r = rgba[0] as u32;
                let g = rgba[1] as u32;
                let b = rgba[2] as u32;
                let a = rgba[3] as u32;

                *argb = a << 24 | r << 16 | g << 8 | b;
            }

            return Ok(buf);
        }

        Err(format!("Could not find asset {}!", asset).into())
    }

    pub fn load_png_dir(&mut self, asset_dir: &str) -> FigResult<Vec<Vec<u32>>> {
        let mut pngs = vec![];
        if self.tar {
            let re = Regex::new(&format!(r"{}/(?<fname>\w+)", asset_dir))?;
            let mut archive = Archive::new(File::open(self.dir)?);
            for file in archive.entries()? {
                let file = file?;
                if let Some(_) = re.captures(&file.path()?.to_str().expect("oops")) {
                    let mut decoder = png::Decoder::new(file);
                    decoder.set_transformations(Transformations::ALPHA);
                    let mut reader = decoder.read_info()?;
                    let mut u8_buf = vec![0; reader.output_buffer_size()];
                    reader.next_frame(&mut u8_buf)?;
                    
                    let mut buf = vec![0u32; reader.output_buffer_size()];
                    for (rgba, argb) in u8_buf.chunks_mut(4).zip(buf.iter_mut()) {
                        let r = rgba[0] as u32;
                        let g = rgba[1] as u32;
                        let b = rgba[2] as u32;
                        let a = rgba[3] as u32;

                        *argb = a << 24 | r << 16 | g << 8 | b;
                    }

                    pngs.push(buf);
                }
            }
        } else {
            let dir = format!("{}/{}", self.dir, asset_dir);
            for file in fs::read_dir(dir)? {
                let file = File::open(file?.path())?;
                let mut decoder = png::Decoder::new(file);
                decoder.set_transformations(Transformations::ALPHA);
                let mut reader = decoder.read_info()?;
                let mut u8_buf = vec![0; reader.output_buffer_size()];
                reader.next_frame(&mut u8_buf)?;
                
                let mut buf = vec![0u32; reader.output_buffer_size()];
                for (rgba, argb) in u8_buf.chunks_mut(4).zip(buf.iter_mut()) {
                    let r = rgba[0] as u32;
                    let g = rgba[1] as u32;
                    let b = rgba[2] as u32;
                    let a = rgba[3] as u32;

                    *argb = a << 24 | r << 16 | g << 8 | b;
                }

                pngs.push(buf);
            }
        }

        if pngs.len() == 0 {
            Err(format!("Could not find asset {}!", asset_dir).into())
        } else {
            Ok(pngs)
        }
    }
}