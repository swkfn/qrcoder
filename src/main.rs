extern crate image;
extern crate qrcode;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate clap;

use std::io;
use std::path::{Path, PathBuf};
use std::process;

use clap::{App, Arg, SubCommand, ArgGroup};

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
    let app = App::new(crate_name!())
        // <>は必須, []は任意
        .arg(Arg::from_usage("<TEXT> 'QRコードに埋め込む文字列を指定してください.'"))
        .arg(Arg::from_usage("-o --output [OUTPUT] '出力先ファイルパスを指定してください.'"))
        .arg(Arg::from_usage("[flg] -f --flag 'sample flag'"))
        .subcommand(SubCommand::with_name("sub")
            .about("sample subcommand") 
            .arg(Arg::from_usage("[subflg] -f --flag 'sample flag by sub'"))
        )
        .args_from_usage("--verb 'verbose mode: level group'
                          --debug 'debug mode: level group'
                          --info 'info mode: level group'")
        .group(ArgGroup::with_name("level")
            .args(&["verb", "debug", "info"]) // グループを所属させる引数を設定
        );

    // 引数を解析
    let matches = app.get_matches();
    let text = String::from(matches.value_of("TEXT").unwrap());
    let output = matches.value_of("OUTPUT").map(PathBuf::from);
    let generate_options = GenerateOptions{ text, output };

    // flgのON/OFFで表示するメッセージを切り替え
    println!("flg is {}", if matches.is_present("flg") {"ON"} else {"OFF"});

    // subサブコマンドの解析結果を取得
    if let Some(ref matches) = matches.subcommand_matches("sub"){
        println!("used sub"); // subが指定されていればメッセージを表示
        println!("subflg is {}", if matches.is_present("subflg") {"ON"} else {"OFF"});
    }
    
    if matches.is_present("level"){
        let(verb, debug, _) = (matches.is_present("verb"),
                            matches.is_present("debug"),
                            matches.is_present("info"));
        println!("level is {}", if verb {"verb"} else if debug {"debug"} else {"info"});
    }
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