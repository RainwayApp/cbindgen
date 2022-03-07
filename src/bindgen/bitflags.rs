/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Parser, Result as ParseResult};

use crate::bindgen::ir::Repr;

// $(#[$outer:meta])*
// ($($vis:tt)*) $BitFlags:ident: $T:ty {
//     $(
//         $(#[$inner:ident $($args:tt)*])*
//         const $Flag:ident = $value:expr;
//     )+
// }
#[derive(Debug)]
pub struct Bitflags {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    #[allow(dead_code)]
    struct_token: Token![struct],
    name: syn::Ident,
    #[allow(dead_code)]
    colon_token: Token![:],
    repr: syn::Type,
    flags: Flags,
}

impl Bitflags {
    pub fn expand(&mut self, force_repr_c: &[String]) -> syn::ItemEnum {
        let Bitflags {
            ref mut attrs,
            ref vis,
            ref name,
            ref repr,
            ref flags,
            ..
        } = *self;

        let repr_name = repr.into_token_stream().to_string();

        if force_repr_c.contains(&repr_name) {
            if let Some(pos) = attrs
                .iter()
                .position(|a| Repr::load(&[a.to_owned()]).is_ok())
            {
                info!(
                    "Bitflags - removing existing repr because force_repr_c includes type {}",
                    repr_name
                );
                attrs.remove(pos);
            }

            attrs.push(parse_quote! {
                #[repr(C)]
            })
        }

        let consts = flags.expand();

        let enum_: syn::ItemEnum = parse_quote! {
            /// cbindgen:internal-derive-bitflags=true
            #(#attrs)*
            #vis enum #name {
                #consts
            }
        };

        enum_
    }
}

impl Parse for Bitflags {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            struct_token: input.parse()?,
            name: input.parse()?,
            colon_token: input.parse()?,
            repr: input.parse()?,
            flags: input.parse()?,
        })
    }
}

// $(#[$inner:ident $($args:tt)*])*
// const $Flag:ident = $value:expr;
#[derive(Debug)]
struct Flag {
    attrs: Vec<syn::Attribute>,
    #[allow(dead_code)]
    const_token: Token![const],
    name: syn::Ident,
    #[allow(dead_code)]
    equals_token: Token![=],
    value: syn::Expr,
    #[allow(dead_code)]
    semicolon_token: Token![;],
}

impl Flag {
    fn expand(&self) -> TokenStream {
        let Flag {
            ref attrs,
            ref name,
            ref value,
            ..
        } = *self;
        quote! {
            #(#attrs)*
            #name = #value,
        }
    }
}

impl Parse for Flag {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            const_token: input.parse()?,
            name: input.parse()?,
            equals_token: input.parse()?,
            value: input.parse()?,
            semicolon_token: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct Flags(Vec<Flag>);

impl Parse for Flags {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let content;
        let _ = braced!(content in input);
        let mut flags = vec![];
        while !content.is_empty() {
            flags.push(content.parse()?);
        }
        Ok(Flags(flags))
    }
}

impl Flags {
    fn expand(&self) -> TokenStream {
        let mut ts = quote! {};
        for flag in &self.0 {
            ts.extend(flag.expand());
        }
        ts
    }
}

pub fn parse(tokens: TokenStream) -> ParseResult<Bitflags> {
    let parser = Bitflags::parse;
    parser.parse2(tokens)
}
