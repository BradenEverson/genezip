use std::{fs::File, io::Read, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::huffman::tree::HuffmanTree;

pub struct GeneZipper {
    huffman: HuffmanTree<String>,
    args: GeneZipperArgs,
    target: File
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
                    target: target_file
                })
            },
            GeneZipCommands::Uncompress { target, key, output } => {
                let target_file = File::open(target.clone())?;
                let huffman = HuffmanTree::from_json(target)?;


                Ok(Self {
                    huffman,
                    args,
                    target: target_file
                })
            }
        }
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
    Uncompress{
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
    IoError(#[from] std::io::Error)
}

pub type Result<T> = std::result::Result<T, GeneZipError>;
