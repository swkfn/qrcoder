extern crate image;
extern crate qrcode;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate clap;

use std::io;
use std::path::{Path, PathBuf};
use std::process;

use clap::{Arg};

use image::Luma;

use qrcode::QrCode;
use qrcode::types::QrError;

struct GenerateOptions{
    text: String,
    output: Option<PathBuf>,
}

enum Command{
    GenerateCode(GenerateOptions),
}

type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Fail)]
enum AppError{
    // エラーをどう表示するか
    #[fail(display = "qr code error: {}", err)]
    QrError{ 
        err: QrError,
    },

    #[fail(display = "io error: {}", err)]
    IoError{
        err: io::Error,
    }
}

impl From<QrError> for AppError{
    fn from(err: QrError) -> Self{
        AppError::QrError{ err }
    }
}

impl From<io::Error> for AppError{
    fn from(err: io::Error) -> Self{
        AppError::IoError{ err }
    }
}

fn parse_command() -> AppResult<Command>{
    let app = app_from_crate!()
        // <>は必須, []は任意
        .arg(Arg::from_usage("<TEXT> 'QRコードに埋め込む文字列を指定してください.'"))
        .arg(Arg::from_usage("[OUTPUT] -o --output [FILE] '出力先ファイルパスを指定してください.'"));

    // 引数を解析
    let matches = app.get_matches();
    let text = String::from(matches.value_of("TEXT").unwrap());
    let output = matches.value_of("OUTPUT").map(PathBuf::from);
    let generate_options = GenerateOptions{ text, output };

    Ok(Command::GenerateCode(generate_options))
}

fn output_file(code: &QrCode, path: &Path) -> AppResult<()>{
    // 画像生成
    let image = code.render::<Luma<u8>>().build();
    // ファイル出力
    image.save(path)?;

    Ok(())
}

fn output_stdout(code: &QrCode) -> AppResult<()>{
    let text = code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", text);

    Ok(())
}

fn generate_code(generate_options: &GenerateOptions) -> AppResult<()>{
    // QRコード生成
    let code = QrCode::new(generate_options.text.as_bytes())?;

    // 出力先を指定されていれば画像に, ない場合は標準出力に
    match generate_options.output.as_ref(){
        Some(ref path) => output_file(&code, path),
        None => output_stdout(&code),
    }

}

fn run() -> AppResult<()>{
    match parse_command()?{
        Command::GenerateCode(generate_options) => generate_code(&generate_options),
    }
}

fn main(){
    process::exit(match run(){
        Ok(()) => 0,
        Err(e) => {
            eprintln!("error: {}", e);
            -1
        }
    })
}