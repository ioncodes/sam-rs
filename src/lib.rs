#![crate_type="dylib"]
#![feature(quote, plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::symbol::Symbol;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder; // A trait for expr_usize.
use syntax::ext::quote::rt::Span;
use rustc_plugin::Registry;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;

fn expand_sam(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    if args.len() != 1 {
        cx.span_err(
            sp,
            &format!(
                "argument should be a string identifier, but got {} arguments",
                args.len()
            ),
        );
        return DummyResult::any(sp);
    }

    let text = match args[0] {
        TokenTree::Token(_, token::Literal(token::Str_(s), _)) => s.to_string(),
        _ => {
            cx.span_err(sp, "argument should be a string identifier");
            return DummyResult::any(sp);
        }
    };

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "ks.exe", &*text])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("ks")
            .arg(&*text)
            .output()
            .expect("failed to execute process")
    };

    let mut file = File::create("foo.txt").unwrap();
    file.write_all(&output.stdout);

    let result = &String::from_utf8(output.stdout).unwrap();

    MacEager::expr(cx.expr_str(sp, Symbol::intern(result)))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("sam", expand_sam);
}
