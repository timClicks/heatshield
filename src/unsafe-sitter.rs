use prettyplease;
use syn;
use tree_sitter_rust;
use tree_sitter::{Language, Parser, Query, QueryCursor};

fn main() {
    let lang = tree_sitter_rust::language();

    let mut parser = Parser::new();
    parser.set_language(lang).unwrap();

    let src = "fn main() { unsafe { () }; unsafe { todo!(); }

    let lol = unsafe {
        std::mem::transmute::<f32>(123);
    }

}";
    let tree = parser.parse(src, None).unwrap();

    let query = Query::new(lang, "(unsafe_block (_) @dangerous)").expect("invalid query");

    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, tree.root_node(), src.as_bytes());

    let mut unsafe_blocks = vec![];
    for m in matches {
        for capture in m.captures {
            unsafe_blocks.push(capture.node.range());
        }
    }

    for block in unsafe_blocks {
        let start = block.start_byte;// + 1; // remove {
        let end = block.end_byte;// - 1;     // remove }
        let span = &src.as_bytes()[start..end];
        let mut kludge = String::from("fn main () { unsafe ");
        let code = std::str::from_utf8(span).unwrap();
        kludge.push_str(code);
        kludge.push('}');
        let dangerous = syn::parse_str(&kludge).unwrap();
    //     let dangerous = syn::parse_str::<syn::Expr>(&kludge).unwrap();
    //     // println!("{dangerous:?}");
    //     let mut p = prettyplease::Printer::new();
    //     p.expr(&dangerous);
    // // p.file(file);
    //     p.eof();
        let formatted = prettyplease::unparse(&dangerous);
        // let fragment = parser.parse(formatted, None).unwrap();


        println!("{formatted}");
    }
}
