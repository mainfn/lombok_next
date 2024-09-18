extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_str,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Comma, Token},
    Attribute, Data, DeriveInput, Field, Fields, Ident, ItemStruct, Path, Type,
};

#[proc_macro_derive(Getter, attributes(skip_getter))]
pub fn derive_getter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("`Getter` derive macro only supports named fields of structs"),
        },
        _ => panic!("`Getter` derive macro only supports structs"),
    };

    let getters = fields.iter().filter_map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        if should_skip(&field.attrs, "skip_getter") {
            None
        } else {
            Some(quote! {
                pub fn #field_name(&self) -> &#field_type {
                    &self.#field_name
                }
            })
        }
    });

    // Capture the generics from the struct
    let generics = input.generics;
    // Add the generics to the struct implementation
    let (impl_generics, generics_type, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #struct_ident #generics_type #where_clause {
            #(#getters)*
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Setter, attributes(skip_setter))]
pub fn derive_setter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("`Setter` derive macro only supports named fields of structs"),
        },
        _ => panic!("`Setter` derive macro only supports structs"),
    };

    let setters = fields.iter().filter_map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        let setter_field_name = Ident::new(
            &format!("set_{}", field_name.as_ref().unwrap()),
            field_name.span(),
        );

        if should_skip(&field.attrs, "skip_setter") {
            None
        } else {
            Some(quote! {
                pub fn #setter_field_name(&mut self, value: #field_type) {
                    self.#field_name = value;
                }
            })
        }
    });

    // Capture the generics from the struct
    let generics = input.generics;
    // Add the generics to the struct implementation
    let (impl_generics, generics_type, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #struct_name #generics_type #where_clause {
            #(#setters)*
        }
    };

    TokenStream::from(expanded)
}

fn should_skip(attrs: &[Attribute], attr_name_for_skip: &str) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_ident(attr_name_for_skip))
}
