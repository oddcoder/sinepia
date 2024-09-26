use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn instrument_parse(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as ItemFn);
    let attrs = input.attrs;
    let vis = input.vis;
    let sig = input.sig;
    let block = input.block;
    let toks = quote!(
        #(#attrs)*
        #vis #sig {
            #block
            .inspect(|item| {
                salsa::plumbing::attach(ctx.db, || {
                    tracing::trace!("{item}");
                })
            })
            .inspect_err(|e| tracing::trace!("Returning an Error, {e:?}"))
        }
    );
    toks.into()
}
