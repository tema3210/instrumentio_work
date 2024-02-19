use proc_macro::{TokenStream,TokenTree,Punct,Ident,Group,Literal,Spacing};

macro_rules! nxt {
    ($t:ident,$l:lifetime) => {
        {
            let Some(item) = $t.next() else {
                break $l Err("not present")
            };
            item
        }
    };
    ($p:pat,$t:ident,$l:lifetime) => {
        let $p = nxt!($t,$l) else {
            break $l Err("bad input for pattern");
        };
    }
}



/// works only with literals since i'm lazy
#[proc_macro]
pub fn btreemap(toks: TokenStream) -> TokenStream {


    dbg!(&toks);
    // literal, punct(=), punct(>), literal
    //           s: joint  s: alone
    // punct(,) *

    let mut pairs = Vec::new();

    let mut iter = toks.into_iter().peekable();
    'm: loop { 
        // generates let or throws
        nxt!(TokenTree::Literal(key),iter,'m);

        nxt!(TokenTree::Punct(p),iter,'m);
        if p != '=' { break 'm Err("bad input 1")}

        nxt!(TokenTree::Punct(p),iter,'m);
        if p != '>' { break 'm Err("bad input 2")}

        nxt!(TokenTree::Literal(val),iter,'m);

        pairs.push((key,val));
        // match the comma

        match iter.peek() {
            Some(TokenTree::Punct(comma)) if *comma == ',' => {
                let _ = iter.next();
                continue
            },
            None => break 'm Ok(()),
            Some(_) => break 'm Err("bad input 3")
        }
    }.unwrap();


    // assemble the data insertion code
    let data = pairs.into_iter().fold(String::new(),|mut data, (key,value)| {
        data.push_str(
            &format!("btreemap.insert({},{});\n",key,value)
        );
        data
    });

    // assemble the result
    let result = format!("{}{}{}",
        "
        {
            let mut btreemap = std::collections::BTreeMap::new();
        ",
            data,
        "
            btreemap
        }
        "

    );

    result.parse().unwrap()
}