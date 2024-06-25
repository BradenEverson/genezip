use clap::Parser;
use genezip::zip::{GeneZipError, GeneZipper, GeneZipperArgs};

fn main() -> Result<(), genezip::zip::GeneZipError> {
    let args = GeneZipperArgs::parse();
    let mut genezipper = GeneZipper::new(args)?;

    match genezipper.process() {
        Ok(_) => {
            println!("Your file was processed sucessfully!!");
        }, 
        Err(error) => {
            match error {
                GeneZipError::HuffmanError(h_err) => println!("Error generating huffman encoding file: \n{}", h_err),
                GeneZipError::IoError(io_err) => println!("Error reading or writing to file: \n{}", io_err),
                GeneZipError::SerialzeError(serde_err) => println!("Error serializing file: \n{}", serde_err),
            }
        }
    }

    Ok(())
}
