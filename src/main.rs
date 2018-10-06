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

fn main(){
    let app = App::new(crate_name!())
        // <>は必須, []は任意
        .arg(Arg::from_usage("<pa> 'sample positional argument'"))
        .arg(Arg::from_usage("[flg] -f --flag 'sample flag'"))
        .arg(Arg::from_usage("-o --opt [OPT] 'sample option'"))
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
    /*
    let app = App::new(crate_name!())
        .version(crate_version!()) // バージョン情報
        .author(crate_authors!()) // 作者
        .about(crate_description!()) // ツールについて
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
    */
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

    if matches.is_present("level"){
        let(verb, debug, _) = (matches.is_present("verb"),
                            matches.is_present("debug"),
                            matches.is_present("info"));
        println!("level is {}", if verb {"verb"} else if debug {"debug"} else {"info"});
    }
}
