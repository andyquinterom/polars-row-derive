use proc_macro::TokenStream;
use quote::quote;

/// Allows iterators over this type to be converted into a polars DataFrame.
/// This is useful when you have a collection of rows that you want to convert into a DataFrame
/// without having to manually create a DataFrame and with type safety.
#[proc_macro_derive(IterToDataFrame)]
pub fn stream_handler_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    impl_handler_macro(&input)
}

fn impl_handler_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let trait_name_exact = syn::Ident::new(&format!("IterToDataFrame{name}"), name.span());
    let trait_name_dyn = syn::Ident::new(&format!("IterToDataFrameDyn{name}"), name.span());

    let doc_msg_exact = format!(
        "Trait to convert an exact size iterator with elements of type {} into a polars DataFrame.",
        name
    );

    let doc_msg_dyn = format!(
        "Trait to convert an iterator with elements of type {} into a polars DataFrame.",
        name
    );

    let named_fields = match &ast.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(f) => f.named.iter().map(|f| &f.ident).collect::<Vec<_>>(),
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let public = match &ast.vis {
        syn::Visibility::Public(_) => Some(syn::Ident::new("pub", name.span())),
        _ => None,
    };

    let gen = quote! {

        #[automatically_derived]
        #public trait #trait_name_exact {
            #[doc = #doc_msg_exact]
            fn to_dataframe(self) -> Result<polars::frame::DataFrame, polars::prelude::PolarsError>;
        }

        #[automatically_derived]
        #public trait #trait_name_dyn {
            #[doc = #doc_msg_dyn]
            fn to_dataframe_dyn(self) -> Result<polars::frame::DataFrame, polars::prelude::PolarsError>;
        }

        #[automatically_derived]
        impl<T> #trait_name_exact for T
        where
            T: Iterator<Item = #name> + ExactSizeIterator,
        {
            fn to_dataframe(self) -> Result<polars::frame::DataFrame, polars::prelude::PolarsError> {

                let len = self.len().to_owned();

                // Extract the field values into separate vectors
                #(let mut #named_fields = Vec::with_capacity(len);)*


                for e in self.into_iter() {
                    #(#named_fields.push(e.#named_fields);)*
                }
                polars::df! {
                    #(stringify!(#named_fields) => #named_fields,)*
                }
            }
        }

        #[automatically_derived]
        impl<T> #trait_name_dyn for T
        where
            T: Iterator<Item = #name>,
        {
            fn to_dataframe_dyn(self) -> Result<polars::frame::DataFrame, polars::prelude::PolarsError> {

                // Extract the field values into separate vectors
                #(let mut #named_fields = Vec::new();)*

                for e in self.into_iter() {
                    #(#named_fields.push(e.#named_fields);)*
                }
                polars::df! {
                    #(stringify!(#named_fields) => #named_fields,)*
                }
            }
        }
    };

    gen.into()
}
