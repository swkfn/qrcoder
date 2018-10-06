extern crate clap;
extern crate image;
extern crate qrcode;

#[macro_use]
extern crate failure;

use std::io;
use std::path::{Path, PathBuf};
use std::process;

use clap::{App, Arg, SubCommand};

use image::Luma;

use qrcode::QrCode;
use qrcode::types::QrError;

struct GenerateOptions{
    text: String,
    output: Option<PathBuf>,
}

fn main(){
    let app = App::new("clapex")
        .version("0.1.0") // バージョン情報
        .author("swkfn") // 作者
        .about("Clap exsample") // ツールについて
        .arg(Arg::with_name("pa") // 位置引数を定義
        .help("sample positional argument") // ヘルプメッセージ
        .required(true) // 引数は必須
        )
        .arg(Arg::with_name("flg") // フラグを定義
            .help("sample flag") // ヘルプメッセージ
            .short("f") // ショートコマンド
            .long("flag") // ロングコマンド
        )
        .arg(Arg::with_name("opt") // オプションを定義
            .help("sample option") // ヘルプメッセージ
            .short("o") // ショートコマンド
            .long("opt") // ロングコマンド
            .takes_value(true) // 値を持つことを定義
        )
        .subcommand(SubCommand::with_name("sub") // サブコマンドを定義
            .about("sample subcommand")
            .arg(Arg::with_name("subfig")
                .help("sample flag by sub")
                .short("f")
                .long("flag")
            )
        );
    
    // 引数を解析
    let matches = app.get_matches();

    // paが指定されていれば値を表示
    if let Some(o) = matches.value_of("pa"){
        println!("Value for pa {}", o);
    }

    // optが指定されていれば値を表示
    if let Some(o) = matches.value_of("opt"){
        println!("Value for opt {}", o);
    }

    // flgのON/OFFで表示するメッセージを切り替え
    println!("flg is {}", if matches.is_present("flg") {"ON"} else {"OFF"});

    // subサブコマンドの解析結果を取得

    if let Some(ref matches) = matches.subcommand_matches("sub"){
        println!("used sub"); // subが指定されていればメッセージを表示
        println!("subflg is {}", if matches.is_present("subflg") {"ON"} else {"OFF"});
    }
}
