#![crate_type="dylib"]
#![feature(quote, plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate keystone;

use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::symbol::Symbol;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder; // A trait for expr_usize.
use syntax::ext::quote::rt::Span;
use rustc_plugin::Registry;
use keystone::*;


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

    let engine = Keystone::new(Arch::X86, MODE_32).expect("Could not initialize Keystone engine");
    engine.option(OptionType::SYNTAX, OPT_SYNTAX_NASM).expect(
        "Could not set option to nasm syntax",
    );
    let result = engine.asm(text, 0).expect("Could not assemble").bytes;

    MacEager::expr(quote_expr!(cx, vec![result]))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("sam", expand_sam);
}
