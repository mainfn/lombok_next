#[proc_macro_attribute]
pub fn extends(attr: TokenStream, item: TokenStream) -> TokenStream {
    let base_struct_ident = parse_macro_input!(attr as Ident);
    let mut derived_struct_input = parse_macro_input!(item as DeriveInput);

    // get the fields of the base struct
    let base_fields = get_struct_fields(&base_struct_ident);

    // add the base fields to the derived struct
    let derived_fields = match &mut derived_struct_input.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("`extends` macro only supports structs with named fields"),
    };

    // apply the base fields to the derived struct
    derived_fields.extend(base_fields);

    // generate the expanded code
    let expanded = quote! {
        #derived_struct_input
    };

    TokenStream::from(expanded)
}

// fn get_struct_fields(struct_ident: &Ident) -> Punctuated<Field, Comma> {
//     let struct_name = struct_ident.to_string();
//     let struct_def = format!("struct {};", struct_name);
//     let struct_item: ItemStruct =
//         parse_str(&struct_def).expect("failed to parse struct definition");

//     match struct_item.fields {
//         Fields::Named(fields) => fields.named,
//         _ => panic!("`extends` macro only supports structs with named fields"),
//     }
// }

// without string parsing
fn get_struct_fields(item_struct: &ItemStruct) -> Punctuated<Field, Comma> {
    match &item_struct.fields {
        Fields::Named(fields) => fields.named.clone(),
        _ => panic!("`extends` macro only supports structs with named fields"),
    }
}
// #[proc_macro_attribute]
// pub fn extends(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(attr as DeriveInput);
//     let meta = parse_macro_input!(attr as Path);
//     let struct_name = input.ident;
//     let struct_visibility = input.vis;

//     let fields = match &input.data {
//         Data::Struct(data_struct) => match &data_struct.fields {
//             Fields::Named(fields) => &fields.named,
//             _ => panic!("`#[extends($Struct)]` proc_macro only supports named fields of structs"),
//         },
//         _ => panic!("`#[extends($Struct)]` proc_macro only supports structs"),
//     };

//     let expanded = quote! {
//         #struct_visibility struct #struct_name {
//             #meta,
//             #(#fields,)*
//         }
//     };

//     TokenStream::from(expanded)
// }

// fn get_struct_fields(ty: &Type) -> Punctuated<Field, Comma> {
//     // extract the struct idetifier from the type of the struct
//     let struct_name = if let Type::Path(type_path) = ty {
//         // get the last segment of the path which is the struct name
//         &type_path.path.segments.last().unwrap().ident
//     } else {
//         panic!("Expected a struct name");
//     };
//     let
//     }
// #[proc_macro_attribute]
// pub fn getter(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     // Parse the input tokens into a syntax tree
//     let input = parse_macro_input!(item as ItemStruct);
//     let struct_name = &input.ident;
//     let struct_vis = &input.vis;
//     let fields = &input.fields;

//     // Generate getter methods for each field, unless it has the #[skip_getter] attribute
//     let getters = fields.iter().filter_map(|f| {
//         let field_name = &f.ident;
//         let field_type = &f.ty;
//         let field_attrs = &f.attrs;

//         // check if the field has the #[skip_getter] attribute
//         let should_skip_getter = field_attrs
//             .iter()
//             .any(|attr| attr.path().is_ident("skip_getter"));
//         // if the field has the #[skip_getter] attribute, skip it
//         if should_skip_getter {
//             None
//         } else {
//             // generate getter for the field
//             Some(quote! {
//                 #struct_vis fn #field_name(&self) -> &#field_type {
//                     &self.#field_name
//                 }
//             })
//         }
//     });

//     // Generate the expanded code
//     let expanded = quote! {
//         #input

//         impl #struct_name {
//             #(#getters)*
//         }
//     };

//     // Convert the expanded code into a TokenStream and return it
//     TokenStream::from(expanded)
// }

// // This macro does nothing, it's just a marker
// #[proc_macro_attribute]
// pub fn skip_getter(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     item
// }

// // fn generate_getters_for_struct(input ItemStruct) -> TokenStream {
// //     // name of a struct
// //     let name = &input.name;
// //     // visibility of a struct
// //     let vis = &input.vis;
// //     // fields of a struct
// //     let fields = &input.fields;

// //     let getters = fields.iter().map(|field| {
// //         let name = &field.ident;
// //         let ty = &field.ty;

// //         quote! {
// //             #vis fn #name(&self) -> &#ty {
// //                 &self.#name
// //             }
// //         }
// //     });

// //     let expanded = quote! {
// //         impl #name {
// //         }
// //         }
// //     }
// // }

// // fn generate_getter_for_struct_field(field: Field) -> TokenStream {
// //     // name of a struct field
// //     let name = &field.ident;
// //     // type of a struct field
// //     let ty = &field.ty;
// //     // visibility of a struct field
// //     let vis = &field.vis;

// //     quote! {
// //         impl #name {
// //             #vis fn #name(&self) -> &#ty {
// //                 &self.#name
// //             }
// //         }
// //     }
// // }
