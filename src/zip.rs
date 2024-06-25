use std::{fs::File, io::{Read, Write}, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::huffman::tree::{HuffmanError, HuffmanTree};

pub struct GeneZipper {
    huffman: HuffmanTree<String>,
    args: GeneZipperArgs,
    target: String
}

impl GeneZipper {
    pub fn new(args: GeneZipperArgs) -> Result<Self> {
        return match args.clone().cmd {
            GeneZipCommands::Compress { target, output: _ } => {
                let mut target_file = File::open(target)?;
                let mut text = String::new();

                let _ = target_file.read_to_string(&mut text)?;

                let temporary_until_runs_implemented = text.chars().map(|ch| format!("{}", ch)).collect::<Vec<String>>();

                let huffman = HuffmanTree::from_data(&temporary_until_runs_implemented);

                Ok(Self {
                    huffman,
                    args,
                    target: text
                })
            },
            GeneZipCommands::Decompress { target, key, output: _ } => {
                let mut target_file = File::open(target.clone())?;
                let mut text = String::new();

                let _ = target_file.read_to_string(&mut text);

                let huffman = HuffmanTree::from_json(key)?;


                Ok(Self {
                    huffman,
                    args,
                    target: text
                })
            }
        }
    }

    pub fn process(&mut self) -> Result<()> {
        match self.args.cmd.clone() {
            GeneZipCommands::Compress { target: _, output } => {
                //Run text through huffman, iterate through keys and build up vec of encodings,
                //write encodings to file
                let mut res: Vec<Vec<bool>> = vec![];

                for character in self.target.chars() {
                    let huff_encoding = self.huffman.encodings_to(&format!("{}", character));
                    if let Some(encoding) = huff_encoding {
                        res.push(encoding.clone());
                    } else {
                        return Err(GeneZipError::HuffmanError(HuffmanError::InvalidHuffmanEncodingError));
                    }
                }

                let out_file = File::create_new(output.clone())?;

                serde_json::to_writer(out_file, &res)?;
                self.huffman.to_json(format!("{}.gzpky", output.to_str().unwrap()))?;

            },
            GeneZipCommands::Decompress { target: _, key: _, output } => {
                //Run encodings in target through huffman to get vals, build up vec of values,
                //write decodings to output
                let deser_encodings: Vec<Vec<bool>> = serde_json::from_str(&self.target)?;
                let mut res = String::new();
                for encoding in deser_encodings {
                    res = format!("{}{}", res, self.huffman.get_to(encoding)?);
                }

                let mut out_file = File::create_new(output)?;
                out_file.write_all(res.as_bytes())?;
            }
        }
        Ok(())
    }
}

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct GeneZipperArgs {
    #[command(subcommand)]
    cmd: GeneZipCommands,

}

#[derive(Subcommand, Debug, Clone)]
pub enum GeneZipCommands {
    Compress{
        target: PathBuf,
        output: PathBuf
    },
    Decompress{
        target: PathBuf,
        key: PathBuf,
        output: PathBuf
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GeneZipError {
    #[error("Error Generating Huffman Tree: {0}")]
    HuffmanError(#[from] crate::huffman::tree::HuffmanError),
    #[error("File IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error serializing data: {0}")]
    SerialzeError(#[from] serde_json::Error)
}

pub type Result<T> = std::result::Result<T, GeneZipError>;
