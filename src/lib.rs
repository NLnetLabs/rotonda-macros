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

    let len = attr.elems.iter().len();
    println!("{}", len);

    let attrs = attr.elems.iter().collect::<Vec<_>>();

    let af = match attrs[0] {
        syn::Expr::Path(t) => t,
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

#[proc_macro_attribute]
pub fn stride_sizes(attr: TokenStream, item: TokenStream) -> TokenStream {
    // The struct that's defined underneath the macro invocation
    let input = parse_macro_input!(item as syn::ItemStruct);

    // The name of that struct
    let name = &input.ident;

    // The arguments for the macro invocation
    let attrs = parse_macro_input!(attr as syn::ExprTuple);

    let attrs = attrs.elems.iter().collect::<Vec<_>>();

    let _af = match attrs[0] {
        syn::Expr::Path(t) => t,
        _ => panic!("Expected Family Type"),
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
        pub(crate) struct #name<AF: AddressFamily> {
            # ( #strides_all_len_level: NodeSet<AF, #strides>, )*
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
        impl<AF: AddressFamily> FamilyBuckets<AF> for #name<AF> {
            fn init() -> Self {
                NodeBuckets4 {
                    #( #strides_all_len_level: NodeSet::init(1 << Self::len_to_store_bits(#strides_all_len_accu, 0).unwrap() ), )*
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

            fn get_store3_mut(
                &mut self,
                id: StrideNodeId<AF>,
            ) -> &mut NodeSet<AF, Stride3> {
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

            fn get_store3(&self, id: StrideNodeId<AF>) -> &NodeSet<AF, Stride3> {
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
                id: StrideNodeId<AF>,
            ) -> &mut NodeSet<AF, Stride4> {
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

            fn get_store4(&self, id: StrideNodeId<AF>) -> &NodeSet<AF, Stride4> {
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
                id: StrideNodeId<AF>,
            ) -> &mut NodeSet<AF, Stride5> {
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

            fn get_store5(&self, id: StrideNodeId<AF>) -> &NodeSet<AF, Stride5> {
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
            fn get_stride_sizes(&self) -> [u8; 42] {
                [ #( #stride_sizes, )*]
            }

            #[inline]
            fn get_stride_for_id(&self, id: StrideNodeId<AF>) -> u8 {
                [ #(#len_to_stride_arr, )* ][id.get_id().1 as usize]
            }

        }

    };

    let result = quote! {
        #struct_creation
        #struct_impl
    };

    TokenStream::from(result)
}
