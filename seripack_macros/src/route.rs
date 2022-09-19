// Standard Uses

// Crate Uses
use crate::utils::attribute_args_ext::AttributeArgsExt;

// External Uses
use syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::AttributeArgs;


pub fn route_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemFn);
    let args = syn::parse_macro_input!(args as AttributeArgs);

    // Parse the item into name and fn body
    let fn_name = input.sig.ident;
    // let output = args.fold_item_fn(&input);

    // Parse the args into Header and PacketType
    if args.iter().len() < 2 {
        panic!("Two arguments are needed: the header and the packet type");
    }

    let (_, _header) = args.get_key_value_pair();
    let (_, packet_type) = args.get_key_value_pair();

    // Build the function
    TokenStream::from(quote! {
        fn #fn_name(pure_packet: Box<dyn Packet>) {
            let packet = pure_packet.downcast_ref::<#packet_type>().unwrap();
            // #output
        }
    })
}

