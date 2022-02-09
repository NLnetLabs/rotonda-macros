extern crate proc_macro;
use std::iter::Iterator;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn test_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemStruct);
    let name = &input.ident;

    let attr = parse_macro_input!(attr as syn::ExprTuple);

    let attrs = attr.elems.iter().collect::<Vec<_>>();

    let _ip_af = match attrs[0] {
        syn::Expr::Type(t) => t,
        _ => panic!("Expected Family Type"),
    };

    let strides = match attrs[1] {
        syn::Expr::Array(a) => {
            let array = a
                .elems
                .iter()
                .map(|e| match e {
                    syn::Expr::Lit(s) => {
                        if let syn::Lit::Int(i) = &s.lit {
                            i.base10_parse::<u8>().unwrap()
                        } else {
                            panic!("Expected an integer")
                        }
                    }
                    _ => {
                        panic!("Expected a literal")
                    }
                })
                .collect::<Vec<u8>>();
            quote! { [#( #array ),*] }
        }
        syn::Expr::Path(s) => {
            let array = s.path.segments[0].ident.clone();
            quote! { #array.to_vec() }
        }
        _ => panic!("Expected a const or static"),
    };

    let result = quote! {

        pub(crate) struct #name {
            strides: [u8; 4],
        }

        impl #name {
            fn a() -> impl IntoIterator<Item = u8> {
                #strides
            }
        }

    };

    TokenStream::from(result)
}

#[proc_macro]
pub fn test_macro2(input: TokenStream) -> TokenStream {
    input
}


#[proc_macro_attribute]
pub fn stride_sizes(attr: TokenStream, input: TokenStream) -> TokenStream {
    // The arguments for the macro invocation
    let attrs = parse_macro_input!(attr as syn::ExprTuple);

    let attrs = attrs.elems.iter().collect::<Vec<_>>();

    let input = parse_macro_input!(input as syn::ItemStruct);
    let type_name = &input.ident;
    let ip_af = match attrs[0] {
        syn::Expr::Path(t) => t,
        _ => panic!("Expected Family Type"),
    };

    // The name of the Struct that we're going to generate
    // We'll prepend it with the name of the TreeBitMap struct
    // that the user wants, so that our macro is a little bit
    // more hygienic, and the user can create multiple types
    // of TreeBitMap structs with different stride sizes.
    let buckets_name = if ip_af.path.is_ident("IPv4") {
        format_ident!("{}NodeBuckets4", type_name)
    } else {
        format_ident!("{}NodeBuckets6", type_name)
    };
    let store_bits = if ip_af.path.is_ident("IPv4") {
        quote! {

            fn len_to_store_bits(len: u8, level: u8) -> Option<&'static u8> {
                // (vert x hor) = len x level -> number of bits
                [
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 0
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 1 - never exists
                    [2, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 2 - never exists
                    [3, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 3
                    [4, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // 4
                    [5, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // 5
                    [6, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // 6
                    [7, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // 7
                    [8, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // 8
                    [9, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // 9
                    [10, 0, 0, 0, 0, 0, 0, 0, 0, 0],      // 10
                    [11, 0, 0, 0, 0, 0, 0, 0, 0, 0],      // 11
                    [12, 0, 0, 0, 0, 0, 0, 0, 0, 0],      // 12
                    [12, 13, 0, 0, 0, 0, 0, 0, 0, 0],     // 13
                    [12, 14, 0, 0, 0, 0, 0, 0, 0, 0],     // 14
                    [12, 15, 0, 0, 0, 0, 0, 0, 0, 0],     // 15
                    [12, 16, 0, 0, 0, 0, 0, 0, 0, 0],     // 16
                    [12, 17, 0, 0, 0, 0, 0, 0, 0, 0],     // 17
                    [12, 18, 0, 0, 0, 0, 0, 0, 0, 0],     // 18
                    [12, 19, 0, 0, 0, 0, 0, 0, 0, 0],     // 19
                    [12, 20, 0, 0, 0, 0, 0, 0, 0, 0],     // 20
                    [12, 21, 0, 0, 0, 0, 0, 0, 0, 0],     // 21
                    [12, 22, 0, 0, 0, 0, 0, 0, 0, 0],     // 22
                    [12, 23, 0, 0, 0, 0, 0, 0, 0, 0],     // 23
                    [12, 24, 0, 0, 0, 0, 0, 0, 0, 0],     // 24
                    [12, 24, 25, 0, 0, 0, 0, 0, 0, 0],    // 25
                    [4, 8, 12, 16, 20, 24, 26, 0, 0, 0],  // 26
                    [4, 8, 12, 16, 20, 24, 27, 0, 0, 0],  // 27
                    [4, 8, 12, 16, 20, 24, 28, 0, 0, 0],  // 28
                    [4, 8, 12, 16, 20, 24, 28, 29, 0, 0], // 29
                    [4, 8, 12, 16, 20, 24, 28, 30, 0, 0], // 30
                    [4, 8, 12, 16, 20, 24, 28, 31, 0, 0],  // 31
                    [4, 8 , 12, 16, 20, 24, 28, 32, 0, 0], // 32
                ][len as usize]
                    .get(level as usize)
            }

        }
    } else {
        quote! {

            fn len_to_store_bits(len: u8, level: u8) -> Option<&'static u8> {
                // (vert x hor) = len x level -> number of bits
                [
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 0
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 1 - never exists
                    [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 2
                    [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 3
                    [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 4
                    [5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 5
                    [6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 6
                    [7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 7
                    [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 8
                    [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // len 9
                    [10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],      // len 10
                    [11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],      // len 11
                    [12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],      // len 12
                    [12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 13
                    [12, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 14
                    [12, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 15
                    [12, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 16
                    [12, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 17
                    [12, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 18
                    [12, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 19
                    [12, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 20
                    [12, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 21
                    [12, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 22
                    [12, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 23
                    [12, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 24
                    [12, 24, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],    // 25
                    [12, 24, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],    // 26
                    [12, 24, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],    // 27
                    [12, 24, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],    // 28
                    [12, 24, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 29
                    [12, 24, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 30
                    [12, 24, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 31
                    [12, 24, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 32
                    [12, 24, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 33
                    [12, 24, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 34
                    [12, 24, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 35
                    [12, 24, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 36
                    [12, 24, 36, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 37
                    [12, 24, 36, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 38
                    [12, 24, 36, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 39
                    [12, 24, 36, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 40
                    [12, 24, 36, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 41
                    [12, 24, 36, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 42
                    [12, 24, 36, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 43
                    [12, 24, 36, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 44
                    [12, 24, 36, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 45
                    [12, 24, 36, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 46
                    [12, 24, 36, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 47
                    [12, 24, 36, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 48
                    [4, 8, 12, 24, 28, 48, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 49
                    [4, 8, 12, 24, 28, 48, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 50
                    [4, 8, 12, 24, 28, 48, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 51
                    [4, 8, 12, 24, 28, 48, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 52
                    [4, 8, 12, 24, 28, 48, 52, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 53
                    [4, 8, 12, 24, 28, 48, 52, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 54
                    [4, 8, 12, 24, 28, 48, 52, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 55
                    [4, 8, 12, 24, 28, 48, 52, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 56
                    [4, 8, 12, 24, 28, 48, 52, 56, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 57
                    [4, 8, 12, 24, 28, 48, 52, 56, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 58
                    [4, 8, 12, 24, 28, 48, 52, 56, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 59
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 60
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 61
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 62
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 63
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 64
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],    // 65
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],    // 66
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],    // 67
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],    // 68
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 69
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 70
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 71
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 72
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 73
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // 74
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 75
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 76
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 77
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],      // 78
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 79
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 80
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 81
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],     // 82
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],        // 83
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],        // 84
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // 85
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // 86
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // 87
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],       // 88
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0],      // 89
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0],      // 90
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0],      // 91
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0],      // 92
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 93, 0, 0, 0, 0, 0, 0, 0, 0],     // 93
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 94, 0, 0, 0, 0, 0, 0, 0, 0],     // 94
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 95, 0, 0, 0, 0, 0, 0, 0, 0],     // 95
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 0, 0, 0, 0, 0, 0, 0, 0],     // 96
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 97, 0, 0, 0, 0, 0, 0, 0],    // 97
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 98, 0, 0, 0, 0, 0, 0, 0],    // 98
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 99, 0, 0, 0, 0, 0, 0, 0],        // 99
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 0, 0, 0, 0, 0, 0, 0],       // 100
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 101, 0, 0, 0, 0, 0, 0],     // 101
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 102, 0, 0, 0, 0, 0, 0],     // 102
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 103, 0, 0, 0, 0, 0, 0],     // 103
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 0, 0, 0, 0, 0, 0],     // 104
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 105, 0, 0, 0, 0, 0],   // 105
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 106, 0, 0, 0, 0, 0],       // 106
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 107, 0, 0, 0, 0, 0],       // 107
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 0, 0, 0, 0, 0],       // 108
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 109, 0, 0, 0, 0],     // 109
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 110, 0, 0, 0, 0],     // 110
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 111, 0, 0, 0, 0],           // 111
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 0, 0, 0, 0],           // 112
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 113, 0, 0, 0],         // 113
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 114, 0, 0, 0],         // 114
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 115, 0, 0, 0],         // 115
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 0, 0, 0],         // 116
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 117, 0, 0],       // 117
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 118, 0, 0],       // 118
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 119, 0, 0],       // 119
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 120, 0, 0],       // 120
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 120, 121, 0],     // 121
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 120, 122, 0],     // 122
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 120, 123, 0],     // 123
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 120, 124, 0],     // 124
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 120, 124, 125],   // 125
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 120, 124, 126],   // 126
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 120, 124, 127],   // 127
                    [4, 8, 12, 24, 28, 48, 52, 56, 60, 64, 68, 74, 78, 82, 84, 88, 92, 96, 100, 104, 108, 112, 116, 120, 124, 128],   // 128
                ][len as usize]
                    .get(level as usize)
            }

        }
    };

    let mut strides_num: Vec<u8> = vec![];
    let mut strides = vec![];
    let mut strides_all_len = vec![];
    let mut strides_all_len_accu: Vec<u8> = vec![];
    let mut strides_all_len_level = vec![];
    let mut strides_len3 = vec![];
    let mut strides_len3_l = vec![];
    let mut strides_len4 = vec![];
    let mut strides_len4_l = vec![];
    let mut strides_len5 = vec![];
    let mut strides_len5_l = vec![];

    let mut s_accu = 0_u8;

    let attrs_s = match attrs[1] {
        syn::Expr::Array(arr) => arr,
        _ => panic!("Expected an array"),
    };
    let strides_len = attrs_s.elems.len() as u8;
    let first_stride_size = &attrs_s.elems[0];

    for (len, stride) in attrs_s.elems.iter().enumerate() {
        strides_all_len.push(format_ident!("l{}", len));

        match stride {
            syn::Expr::Lit(s) => {
                if let syn::Lit::Int(i) = &s.lit {
                    let stride_len = i.base10_digits().parse::<u8>().unwrap();
                    strides_num.push(stride_len);
                    strides_all_len_level.push(format_ident!("l{}", s_accu));

                    match stride_len {
                        3 => {
                            strides_len3.push(s_accu as usize);
                            strides_len3_l.push(format_ident!("l{}", s_accu));
                        }
                        4 => {
                            strides_len4.push(s_accu as usize);
                            strides_len4_l.push(format_ident!("l{}", s_accu));
                        }
                        5 => {
                            strides_len5.push(s_accu as usize);
                            strides_len5_l.push(format_ident!("l{}", s_accu));
                        }
                        _ => panic!("Expected a stride of 3, 4 or 5"),
                    };
                    strides_all_len_accu.push(s_accu);

                    s_accu += stride_len;
                    strides.push(format_ident!("Stride{}", stride_len))
                } else {
                    panic!("Expected an integer")
                }
            }
            _ => {
                panic!("Expected a literal")
            }
        }
    }

    // Check if the strides division makes sense
    let mut len_to_stride_arr = [0_u8; 128];
    strides_all_len_accu
        .iter()
        .zip(strides_num.iter())
        .for_each(|(acc, s)| {
            len_to_stride_arr[*acc as usize] = *s;
        });

    // These are the stride sizes as an array of u8s, padded with 0s to the
    // right. It's bounded to 42 u8s to avoid having to set a const generic
    // on the type (which would have to be carried over to its parent). So
    // if a 0 is encountered, it's the end of the strides.
    let mut stride_sizes = [0; 42];
    let (left, _right) = stride_sizes.split_at_mut(strides_len as usize);
    left.swap_with_slice(&mut strides_num);

    // let mut strides = vec![];
    // let mut len_to_stride_size: [StrideType; 128] =
    //     [StrideType::Stride3; 128];
    // let mut strides_sum = 0;
    // for s in strides_vec.iter().cycle() {
    //     strides.push(*s);
    //     len_to_stride_size[strides_sum as usize] = StrideType::from(*s);
    //     strides_sum += s;
    //     if strides_sum >= Store::AF::BITS - 1 {
    //         break;
    //     }
    // }
    // assert_eq!(strides.iter().sum::<u8>(), Store::AF::BITS);

    let struct_creation = quote! {

        #[derive(Debug)]
        pub(crate) struct #buckets_name<AF: AddressFamily> {
            # ( #strides_all_len_level: NodeSet<#ip_af, #strides>, )*
            _af: PhantomData<AF>,
            stride_sizes: [u8; 42],
            strides_len: u8
            // l0: NodeSet<AF, Stride5>,
            // l5: NodeSet<AF, Stride5>,
            // l10: NodeSet<AF, Stride4>,
            // l14: NodeSet<AF, Stride3>,
            // l17: NodeSet<AF, Stride3>,
            // l20: NodeSet<AF, Stride3>,
            // l23: NodeSet<AF, Stride3>,
            // l26: NodeSet<AF, Stride3>,
            // l29: NodeSet<AF, Stride3>,
        }

    };

    let struct_impl = quote! {

        impl<AF: AddressFamily> FamilyBuckets<#ip_af> for #buckets_name<AF> {
            fn init() -> Self {
                #buckets_name {
                    #( #strides_all_len_level: NodeSet::init(1 << Self::len_to_store_bits(#strides_all_len_accu, 0).unwrap() ), )*
                    _af: PhantomData,
                    stride_sizes: [ #( #stride_sizes, )*],
                    strides_len: #strides_len
                    // l0: NodeSet::init(1 << 5),
                    // l5: NodeSet::init(1 << 10),
                    // l10: NodeSet::init(1 << 12),
                    // l14: NodeSet::init(1 << 12),
                    // l17: NodeSet::init(1 << 12),
                    // l20: NodeSet::init(1 << 12),
                    // l23: NodeSet::init(1 << 12),
                    // l26: NodeSet::init(1 << 4),
                    // l29: NodeSet::init(1 << 4),
                }
            }

            fn get_store3_mut(
                &mut self,
                id: StrideNodeId<#ip_af>,
            ) -> &mut NodeSet<#ip_af, Stride3> {
                match id.get_id().1 as usize {
                    #( #strides_len3 => &mut self.#strides_len3_l, )*
                    // 14 => &mut self.l14,
                    // 17 => &mut self.l17,
                    // 20 => &mut self.l20,
                    // 23 => &mut self.l23,
                    // 26 => &mut self.l26,
                    // 29 => &mut self.l29,
                    _ => panic!(
                        "unexpected sub prefix length {} in stride size 3 ({})",
                        id.get_id().1,
                        id
                    ),
                }
            }

            fn get_store3(&self, id: StrideNodeId<#ip_af>) -> &NodeSet<#ip_af, Stride3> {
                match id.get_id().1 as usize {
                    #( #strides_len3 => &self.#strides_len3_l, )*
                    // 14 => &self.l14,
                    // 17 => &self.l17,
                    // 20 => &self.l20,
                    // 23 => &self.l23,
                    // 26 => &self.l26,
                    // 29 => &self.l29,
                    _ => panic!(
                        "unexpected sub prefix length {} in stride size 3 ({})",
                        id.get_id().1,
                        id
                    ),
                }
            }

            fn get_store4_mut(
                &mut self,
                id: StrideNodeId<#ip_af>,
            ) -> &mut NodeSet<#ip_af, Stride4> {
                match id.get_id().1 as usize {
                    #( #strides_len4 => &mut self.#strides_len4_l, )*
                    // 10 => &mut self.l10,
                    _ => panic!(
                        "unexpected sub prefix length {} in stride size 4 ({})",
                        id.get_id().1,
                        id
                    ),
                }
            }

            fn get_store4(&self, id: StrideNodeId<#ip_af>) -> &NodeSet<#ip_af, Stride4> {
                match id.get_id().1 as usize {
                    #( #strides_len4 => &self.#strides_len4_l, )*
                    // 10 => &self.l10,
                    _ => panic!(
                        "unexpected sub prefix length {} in stride size 4 ({})",
                        id.get_id().1,
                        id
                    ),
                }
            }

            fn get_store5_mut(
                &mut self,
                id: StrideNodeId<#ip_af>,
            ) -> &mut NodeSet<#ip_af, Stride5> {
                match id.get_id().1 as usize {
                    #( #strides_len5 => &mut self.#strides_len5_l, )*
                    // 0 => &mut self.l0,
                    // 5 => &mut self.l5,
                    _ => panic!(
                        "unexpected sub prefix length {} in stride size 5 ({})",
                        id.get_id().1,
                        id
                    ),
                }
            }

            fn get_store5(&self, id: StrideNodeId<#ip_af>) -> &NodeSet<#ip_af, Stride5> {
                match id.get_id().1 as usize {
                    #( #strides_len5 => &self.#strides_len5_l, )*
                    // 0 => &self.l0,
                    // 5 => &self.l5,
                    _ => panic!(
                        "unexpected sub prefix length {} in stride size 5 ({})",
                        id.get_id().1,
                        id
                    ),
                }
            }

            #[inline]
            fn get_stride_sizes(&self) -> &[u8] {
                &self.stride_sizes[0..self.strides_len as usize]
            }

            #[inline]
            fn get_stride_for_id(&self, id: StrideNodeId<#ip_af>) -> u8 {
                [ #(#len_to_stride_arr, )* ][id.get_id().1 as usize]
            }

            #[inline]
            #store_bits

            fn get_strides_len() -> u8 {
                #strides_len
            }

            fn get_first_stride_size() -> u8 {
                #first_stride_size
            }
        }

    };

    let type_alias = quote! {
        type #type_name<Meta> = TreeBitMap<CustomAllocStorage<#ip_af, Meta, #buckets_name<#ip_af>>>;
    };

    let result = quote! {
        #struct_creation
        #struct_impl
        #type_alias
    };

    TokenStream::from(result)
}

#[proc_macro_attribute]
pub fn create_store(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemStruct);
    let store_name = &input.ident;

    let attr = parse_macro_input!(attr as syn::ExprTuple);
    let attrs = attr.elems.iter().collect::<Vec<_>>();
    let strides4 = attrs[0].clone();
    let strides6 = attrs[1].clone();
    let strides4_name = format_ident!("{}IPv4", store_name);
    let strides6_name = format_ident!("{}IPv6", store_name);

    let create_strides = quote! {
            use ::std::marker::PhantomData;
            use ::dashmap::DashMap;
            use ::routecore::record::{MergeUpdate, NoMeta};
            use ::routecore::addr::Prefix;

            #[stride_sizes((IPv4, #strides4))]
            struct #strides4_name;

            #[stride_sizes((IPv6, #strides6))]
            struct #strides6_name;
        };

    let store = quote! {
        /// A concurrently read/writable, lock-free Prefix Store, for use in a multi-threaded context.
        pub struct #store_name<
            Meta: routecore::record::Meta + MergeUpdate,
        > {
            v4: #strides4_name<Meta>,
            v6: #strides6_name<Meta>,
        }

        impl<
                Meta: routecore::record::Meta + MergeUpdate,
            > Default for Store<Meta>
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<
                Meta: routecore::record::Meta + MergeUpdate,
            > Store<Meta>
        {
            /// Creates a new empty store with a tree for IPv4 and on for IPv6.
            ///
            /// You'll have to provide the stride sizes per address family and the
            /// meta-data type. Some meta-data type are included with this crate.
            ///
            /// The stride-sizes can be any of [3,4,5], and they should add up
            /// to the total number of bits in the address family (32 for IPv4 and
            /// 128 for IPv6). Stride sizes in the array will be repeated if the sum
            /// of them falls short of the total number of bits for the address
            /// family.
            ///
            /// # Example
            /// ```
            /// use rotonda_store::MultiThreadedStore;
            /// use rotonda_store::PrefixAs;
            ///
            /// let store = MultiThreadedStore::<PrefixAs>::new(
            ///     vec![3, 3, 3, 3, 3, 3, 3, 3, 4, 4], vec![5,4,3,4]
            /// );
            /// ```
            pub fn new() -> Self {
                Store {
                    v4: #strides4_name::new(),
                    v6: #strides6_name::new(),
                }
            }
        }

        impl<
                'a,
                Meta: routecore::record::Meta + MergeUpdate,
            > Store<Meta>
        {
            pub fn match_prefix(
                &'a self,
                prefix_store_locks: (
                    &'a PrefixHashMap<IPv4, Meta>,
                    &'a PrefixHashMap<IPv6, Meta>,
                ),
                search_pfx: &Prefix,
                options: &MatchOptions,
            ) -> QueryResult<'a, Meta> {
                match search_pfx.addr() {
                    std::net::IpAddr::V4(addr) => self.v4.match_prefix(
                        prefix_store_locks.0,
                        &InternalPrefixRecord::<IPv4, NoMeta>::new(
                            addr.into(),
                            search_pfx.len(),
                        ),
                        options,
                    ),
                    std::net::IpAddr::V6(addr) => self.v6.match_prefix(
                        prefix_store_locks.1,
                        &InternalPrefixRecord::<IPv6, NoMeta>::new(
                            addr.into(),
                            search_pfx.len(),
                        ),
                        options,
                    ),
                }
            }

            pub fn insert(
                &mut self,
                prefix: &Prefix,
                meta: Meta,
            ) -> Result<(), std::boxed::Box<dyn std::error::Error>> {
                match prefix.addr() {
                    std::net::IpAddr::V4(addr) => {
                        self.v4.insert(InternalPrefixRecord::new_with_meta(
                            addr.into(),
                            prefix.len(),
                            meta,
                        ))
                    }
                    std::net::IpAddr::V6(addr) => {
                        self.v6.insert(InternalPrefixRecord::new_with_meta(
                            addr.into(),
                            prefix.len(),
                            meta,
                        ))
                    }
                }
            }

            pub fn prefixes_iter(&self) -> HashMapPrefixRecordIterator<Meta> {
                let rs4 = self.v4.store.prefixes.iter();
                let rs6 = self.v6.store.prefixes.iter();

                crate::HashMapPrefixRecordIterator::<Meta> {
                    v4: Some(rs4),
                    v6: rs6,
                }
            }

            pub fn acquire_prefixes_rwlock_read(
                &'a self,
            ) -> (
                &'a DashMap<PrefixId<IPv4>, InternalPrefixRecord<IPv4, Meta>>,
                &'a DashMap<PrefixId<IPv6>, InternalPrefixRecord<IPv6, Meta>>,
            ) {
                (&self.v4.store.prefixes, &self.v6.store.prefixes)
            }

            pub fn prefixes_len(&self) -> usize {
                self.v4.store.prefixes.len() + self.v6.store.prefixes.len()
            }

            pub fn prefixes_v4_len(&self) -> usize {
                self.v4.store.prefixes.len()
            }

            pub fn prefixes_v6_len(&self) -> usize {
                self.v6.store.prefixes.len()
            }

            pub fn nodes_len(&self) -> usize {
                self.v4.store.get_nodes_len() + self.v6.store.get_nodes_len()
            }

            pub fn nodes_v4_len(&self) -> usize {
                self.v4.store.get_nodes_len()
            }

            pub fn nodes_v6_len(&self) -> usize {
                self.v6.store.get_nodes_len()
            }

            #[cfg(feature = "cli")]
            pub fn print_funky_stats(&self) {
                println!("{}", self.v4);
                println!("{}", self.v6);
            }

            pub fn stats(&self) -> Stats {
                Stats {
                    v4: &self.v4.stats,
                    v6: &self.v6.stats,
                }
            }
        }
    };

    let result = quote! {
        #create_strides
        #store
    };

    TokenStream::from(result)
}