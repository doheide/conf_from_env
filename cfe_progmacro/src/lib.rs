use proc_macro::TokenStream;
use quote::{quote, ToTokens};
//use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput};
use std::string::String;

//#[proc_macro_derive(SetFromEnv, attributes(serde))]
#[proc_macro_derive(SetStructFromEnv)]
pub fn derive_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
                                              fields: syn::Fields::Named(ref fields),
                                              ..
                                          }) = ast.data
    {
        fields
    } else {
        panic!("Only support Struct")
    };

    let mut keys = Vec::new();
    let mut keyus = Vec::new();
    let mut idents = Vec::new();
    let mut types = Vec::new();

    for field in fields.named.iter() {
        let field_name: &syn::Ident = field.ident.as_ref().unwrap();
        let name: String = field_name.to_string();
        //let literal_key_str = syn::LitStr::new(&name, field.span());
        let type_name = &field.ty;
        keys.push(field_name); //quote! { #literal_key_str });
        keyus.push(name.to_uppercase());
        idents.push(&field.ident);
        types.push(type_name.to_token_stream());
    }

    let expanded = quote! {
        impl SetStructFromEnv for #struct_name {
            fn set_from_env() -> #struct_name {
                #struct_name {
                #(
                    #keys: std::env::var(#keyus).unwrap_or_else(|_| panic!("Env var '{}' needs to be set", #keyus))
                        .parse::<#types>().unwrap_or_else(|_| panic!("Env var '{}' needs to be of type '{}'", #keyus, stringify!(#types))),
                )*
                }
            }
        }
    };
    expanded.into()
}

