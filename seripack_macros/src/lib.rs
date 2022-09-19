// Relative Modules
mod utils;
mod route;
mod route_v2;

// Standard Uses

// Crate Uses

// External Uses
use proc_macro::TokenStream;


/// When a router receives a packet it will search for the registered
/// functions with this attribute and its matching Id and call it
#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
   route::route_macro(args, input)
}


/// Same as 'route' macro but with different syntax
#[proc_macro_attribute]
pub fn route2(args: TokenStream, input: TokenStream) -> TokenStream {
   route_v2::route_macro2(args, input)
}

