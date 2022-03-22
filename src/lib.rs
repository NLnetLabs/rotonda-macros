extern crate proc_macro;

mod maps;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::iter::Iterator;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn test_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemStruct);
    let name = &input.ident;

    let attr = parse_macro_input!(attr as syn::ExprTuple);

    let attrs = attr.elems.iter().collect::<Vec<_>>();

    let ip_af = match attrs[0] {
        syn::Expr::Type(t) => t,
        _ => panic!("Expected Family Type"),
    };

    let strides = match attrs[1] {
        syn::Expr::Array(a) => {
            // 1. Try to parse the second expression of the TokenStream as an
            //    array of u8. Panic if that fails.
            let strides_vec = a
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

            // 2. Check if the strides division makes sense
            let af_bits: u8 = if let syn::Expr::Path(p) = &*ip_af.expr {
                match p.path.get_ident() {
                    Some(i) => match i.to_string().as_str() {
                        "IPv4" => 32_u8,
                        "IPv6" => 128_u8,
                        _ => panic!("Expected Ipv4 or Ipv6"),
                    },
                    None => panic!("Expected an identifier"),
                }
            } else {
                panic!("Expected a path")
            };

            let mut strides = vec![];
            let mut strides_sum = 0;
            for s in strides_vec.iter().cycle() {
                strides.push(*s);
                strides_sum += s;
                if strides_sum >= af_bits - 1 {
                    break;
                }
            }
            assert_eq!(strides_vec.iter().sum::<u8>(), af_bits);

            quote! { [#( #strides ),*] }
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
    let prefixes_all_len;
    let all_len;
    let prefixes_buckets_name: syn::Ident;
    // let prefix_store_bits;
    let get_root_prefix_set;

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
        all_len = (0..=32_u8).collect::<Vec<_>>();
        prefixes_all_len = (0..=32_u8)
            .map(|l| format_ident!("p{}", l))
            .collect::<Vec<_>>();
        prefixes_buckets_name = format_ident!("PrefixBuckets4");
        // prefix_store_bits = format_ident!("prefix_store_bits_4");
        get_root_prefix_set = quote! {
            fn get_root_prefix_set(&self, len: u8) -> &'_ PrefixSet<IPv4, M> {
                [
                    &self.p0, &self.p1, &self.p2, &self.p3, &self.p4, &self.p5, &self.p6, &self.p7, &self.p8,
                    &self.p9, &self.p10, &self.p11, &self.p12, &self.p13, &self.p14, &self.p15, &self.p16,
                    &self.p17, &self.p18, &self.p19, &self.p20, &self.p21, &self.p22, &self.p23, &self.p24,
                    &self.p25, &self.p26, &self.p27, &self.p28, &self.p29, &self.p30, &self.p31, &self.p32
                ][len as usize]
            }
        };
        crate::maps::node_buckets_map_v4()
    } else {
        all_len = (0..=128_u8).collect::<Vec<_>>();
        prefixes_all_len = (0..=128_u8)
            .map(|l| format_ident!("p{}", l))
            .collect::<Vec<_>>();

        prefixes_buckets_name = format_ident!("PrefixBuckets6");
        // prefix_store_bits = format_ident!("prefix_store_bits_6");
        get_root_prefix_set = quote! {
            fn get_root_prefix_set(&self, len: u8) -> &'_ PrefixSet<IPv6, M> {
                [
                    &self.p0, &self.p1, &self.p2, &self.p3, &self.p4, &self.p5, &self.p6, &self.p7, &self.p8,
                    &self.p9, &self.p10, &self.p11, &self.p12, &self.p13, &self.p14, &self.p15, &self.p16,
                    &self.p17, &self.p18, &self.p19, &self.p20, &self.p21, &self.p22, &self.p23, &self.p24,
                    &self.p25, &self.p26, &self.p27, &self.p28, &self.p29, &self.p30, &self.p31, &self.p32,
                    &self.p33, &self.p34, &self.p35, &self.p36, &self.p37, &self.p38, &self.p39, &self.p40,
                    &self.p41, &self.p42, &self.p43, &self.p44, &self.p45, &self.p46, &self.p47, &self.p48,
                    &self.p49, &self.p50, &self.p51, &self.p52, &self.p53, &self.p54, &self.p55, &self.p56,
                    &self.p57, &self.p58, &self.p59, &self.p60, &self.p61, &self.p62, &self.p63, &self.p64,
                    &self.p65, &self.p66, &self.p67, &self.p68, &self.p69, &self.p70, &self.p71, &self.p72,
                    &self.p73, &self.p74, &self.p75, &self.p76, &self.p77, &self.p78, &self.p79, &self.p80,
                    &self.p81, &self.p82, &self.p83, &self.p84, &self.p85, &self.p86, &self.p87, &self.p88,
                    &self.p89, &self.p90, &self.p91, &self.p92, &self.p93, &self.p94, &self.p95, &self.p96,
                    &self.p97, &self.p98, &self.p99, &self.p100, &self.p101, &self.p102, &self.p103, &self.p104,
                    &self.p105, &self.p106, &self.p107, &self.p108, &self.p109, &self.p110, &self.p111, &self.p112,
                    &self.p113, &self.p114, &self.p115, &self.p116, &self.p117, &self.p118, &self.p119, &self.p120,
                    &self.p121, &self.p122, &self.p123, &self.p124, &self.p125, &self.p126, &self.p127, &self.p128
                    ][len as usize]
            }
        };
        crate::maps::node_buckets_map_v6()
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

    let struct_creation = quote! {

        #[derive(Debug)]
        pub(crate) struct #buckets_name<AF: AddressFamily> {
            // created fields for each sub-prefix (StrideNodeId) length,
            // with hard-coded field-names, like this:
            // l0: NodeSet<AF, Stride5>,
            // l5: NodeSet<AF, Stride5>,
            // l10: NodeSet<AF, Stride4>,
            // ...
            // l29: NodeSet<AF, Stride3>
            # ( #strides_all_len_level: NodeSet<#ip_af, #strides>, )*
            _af: PhantomData<AF>,
            stride_sizes: [u8; 42],
            strides_len: u8
        }

        #[derive(Debug)]
        pub(crate) struct #prefixes_buckets_name<AF: AddressFamily, M: routecore::record::Meta> {
            // creates a bucket for each prefix (PrefixId) length, with
            // hard-coded field-names, like this:
            // p0: PrefixSet<AF, M>,
            // p1: PrefixSet<AF, M>,
            // ...
            // p32: PrefixSet<AF, M>,
            #( #prefixes_all_len: PrefixSet<#ip_af, M>, )*
            _af: PhantomData<AF>,
            _m: PhantomData<M>,
        }

    };

    let prefix_buckets_map = if ip_af.path.is_ident("IPv4") {
        crate::maps::prefix_buckets_map_v4()
    } else {
        crate::maps::prefix_buckets_map_v6()
    };

    let prefix_buckets_impl = quote! {

        impl<AF: AddressFamily, M: Meta> PrefixBuckets<#ip_af, M> for #prefixes_buckets_name<AF, M> {
            fn init() -> #prefixes_buckets_name<AF, M> {
                #prefixes_buckets_name {
                    #( #prefixes_all_len: PrefixSet::init(1 << #prefixes_buckets_name::<AF, M>::get_bits_for_len(#all_len, 0).unwrap()), )*
                    _af: PhantomData,
                    _m: PhantomData,
                }
            }

            fn remove(&mut self, id: PrefixId<#ip_af>) -> Option<InternalPrefixRecord<#ip_af, M>> { unimplemented!() }

            #get_root_prefix_set

            #prefix_buckets_map

        }

    };

    let struct_impl = quote! {

        impl<AF: AddressFamily> NodeBuckets<#ip_af> for #buckets_name<AF> {
            fn init() -> Self {
                #buckets_name {
                    // creates l0, l1, ... l<AF::BITS>, but only for the
                    // levels at the end of each stride, so for strides
                    // [5,5,4,3,3,3,3,3,3] is will create l0, l5, l10, l14,
                    // l17, l20, l23, l26, l29 last level will be omitted,
                    // because that will never be used (l29 has children
                    // with prefixes up to prefix-length 32 in this example).
                    #( #strides_all_len_level: NodeSet::init(1 << #buckets_name::<AF>::len_to_store_bits(#strides_all_len_accu, 0).unwrap() ), )*
                    _af: PhantomData,
                    stride_sizes: [ #( #stride_sizes, )*],
                    strides_len: #strides_len
                }
            }

            fn get_store3(&self, id: StrideNodeId<#ip_af>) -> &NodeSet<#ip_af, Stride3> {
                match id.get_id().1 as usize {
                    #( #strides_len3 => &self.#strides_len3_l, )*
                    _ => panic!(
                        "unexpected sub prefix length {} in stride size 3 ({})",
                        id.get_id().1,
                        id
                    ),
                }
            }

            fn get_store4(&self, id: StrideNodeId<#ip_af>) -> &NodeSet<#ip_af, Stride4> {
                match id.get_id().1 as usize {
                    #( #strides_len4 => &self.#strides_len4_l, )*
                    // ex.:
                    // 10 => &self.l10,
                    _ => panic!(
                        "unexpected sub prefix length {} in stride size 4 ({})",
                        id.get_id().1,
                        id
                    ),
                }
            }

            fn get_store5(&self, id: StrideNodeId<#ip_af>) -> &NodeSet<#ip_af, Stride5> {
                match id.get_id().1 as usize {
                    #( #strides_len5 => &self.#strides_len5_l, )*
                    // ex.:
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
        type #type_name<Meta> = TreeBitMap<CustomAllocStorage<#ip_af, Meta, #buckets_name<#ip_af>, #prefixes_buckets_name<#ip_af, Meta>>>;
    };

    let result = quote! {
        #struct_creation
        #struct_impl
        #prefix_buckets_impl
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
            > Default for #store_name<Meta>
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<
                Meta: routecore::record::Meta + MergeUpdate,
            > #store_name<Meta>
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
                Self {
                    v4: #strides4_name::new(),
                    v6: #strides6_name::new(),
                }
            }
        }

        impl<
                'a,
                Meta: routecore::record::Meta + MergeUpdate,
            > #store_name<Meta>
        {
            pub fn match_prefix(
                &'a self,
                search_pfx: &Prefix,
                options: &MatchOptions,
                guard: &'a Guard,
            ) -> QueryResult<'a, Meta> {

                match search_pfx.addr() {
                    std::net::IpAddr::V4(addr) => self.v4.match_prefix(
                        // prefix_store_locks.0,
                        &InternalPrefixRecord::<IPv4, NoMeta>::new(
                            addr.into(),
                            search_pfx.len(),
                        ),
                        options,
                        guard
                    ),
                    std::net::IpAddr::V6(addr) => self.v6.match_prefix(
                        // prefix_store_locks.1,
                        &InternalPrefixRecord::<IPv6, NoMeta>::new(
                            addr.into(),
                            search_pfx.len(),
                        ),
                        options,
                        guard
                    ),
                }
            }

            pub fn more_specifics_iter_from(&'a self,
                search_pfx: &Prefix,
                guard: &'a Guard,
            ) -> QueryResult<'a, Meta> {

                match search_pfx.addr() {
                    std::net::IpAddr::V4(addr) => self.v4.more_specifics_iter_from(
                        PrefixId::<IPv4>::new(
                            addr.into(),
                            search_pfx.len(),
                        ),
                        guard
                    ),
                    std::net::IpAddr::V6(addr) => self.v6.more_specifics_iter_from(
                        PrefixId::<IPv6>::new(
                            addr.into(),
                            search_pfx.len(),
                        ),
                        guard
                    ),
                }
            }

            pub fn insert(
                &self,
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

            pub fn prefixes_iter(&'a self, guard: &'a Guard) -> impl Iterator<Item=routecore::bgp::PrefixRecord<Meta>> + 'a {
                let rs4 = self.v4.store.prefixes_iter(guard);
                let rs6 = self.v6.store.prefixes_iter(guard);

                crate::CustomAllocPrefixRecordIterator {
                    v4: Some(rs4),
                    v6: rs6,
                }
            }

            pub fn prefixes_iter_v4(&'a self, guard: &'a Guard) -> impl Iterator<Item=routecore::bgp::PrefixRecord<Meta>> + 'a {
                let rs4 = self.v4.store.prefixes_iter(guard);

                crate::SingleAFPrefixRecordIterator {
                    tree: rs4,
                    _af: PhantomData,
                    _pb: PhantomData,
                }
            }

            pub fn prefixes_iter_v6(&'a self, guard: &'a Guard) -> impl Iterator<Item=routecore::bgp::PrefixRecord<Meta>> + 'a {
                let rs6 = self.v6.store.prefixes_iter(guard);

                crate::SingleAFPrefixRecordIterator {
                    tree: rs6,
                    _af: PhantomData,
                    _pb: PhantomData,
                }
            }

            pub fn prefixes_len(&self) -> usize {
                self.v4.store.get_prefixes_len() + self.v6.store.get_prefixes_len()
            }

            pub fn prefixes_v4_len(&self) -> usize {
                self.v4.store.get_prefixes_len()
            }

            pub fn prefixes_v6_len(&self) -> usize {
                self.v6.store.get_prefixes_len()
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
