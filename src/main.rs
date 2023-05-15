use clap::Parser;
use image::ImageBuffer;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // QRコードに埋め込む文字列
    text: String,

    // 出力先のファイルパス
    #[arg(short = 'o', long, default_value = "./")]
    output: String,
}

fn main() {
    let args = Args::parse();
    let text = args.text;
    let output = args.output;
    let err_correction_level: QrCodeEcc = QrCodeEcc::Low;

    // for print to console
    // {
    //     let err_correction_level: QrCodeEcc = QrCodeEcc::Low;
    //     let qr_string: QrCode = QrCode::encode_text(&text, err_correction_level).unwrap();
    //     let border: i32 = 4;
    //     for y in -border .. qr_string.size() + border {
    //         for x in -border .. qr_string.size() + border {
    //             let c: char = if qr_string.get_module(x, y) { '█' } else { ' ' };
    //             print!("{0}{0}", c);
    //         }
    //         println!();
    //     }
    //     println!();
    // }

    let qr_binary = QrCode::encode_binary(&text.as_bytes(), err_correction_level).unwrap();
    let mut path = PathBuf::from(output);
    let filename = format!("{}.png", text);
    path.push(filename);
    let mut imgbuf = ImageBuffer::new(qr_binary.size() as u32, qr_binary.size() as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let color = if qr_binary.get_module(x as i32, y as i32) {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        };
        *pixel = color;
    }
    imgbuf.save(path).unwrap();
}
