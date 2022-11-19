// Standard Uses
use std::collections::HashSet as Set;

// Crate Uses

// External Uses
use syn;
use syn::{Token, Ident, parse_macro_input};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use proc_macro::TokenStream;
use syn::ItemFn;


pub struct ArgsHoldingIdents {
    pub idents: Set<Ident>,
}

impl Parse for ArgsHoldingIdents {
    fn parse(args: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![.]>::parse_terminated(args)?;
        Ok(ArgsHoldingIdents {
            idents: vars.into_iter().collect(),
        })
    }
}



pub fn route_macro2(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ArgsHoldingIdents);
    let input = parse_macro_input!(input as ItemFn);

    // Parse the item into name and fn body
    let fn_name = input.sig.ident;

    // let output = args.fold_item_fn(&input);

    // Parse the args into Header and PacketType
    let args_str = args
        .idents
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<_>>();

    if args.idents.len() < 2 {
        panic!("Two arguments are needed: the header and the packet type");
    }

    // let header = &args_str[0];
    let packet_type = &args_str[1];

    // Build the function
    TokenStream::from(quote::quote! {
        fn #fn_name(pure_packet: Box<dyn Packet>) {
            let packet = pure_packet.downcast_ref::<#packet_type>().unwrap();
        }
    })
}

