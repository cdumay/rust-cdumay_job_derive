extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use syn::{Data, DataStruct, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn task(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => Some(&fields.named),
        _ => None,
    };
    let mut stream = match fields {
        Some(named) => {
            let field_name = named.iter().map(|field| &field.ident);
            let field_type = named.iter().map(|field| &field.ty);
            quote! {
                #[derive(Clone)]
                pub struct #name {
                    #(
                        #field_name: #field_type,
                    )*
                    message: MessageRepr,
                    status: Status,
                    result: ResultRepr,
                }
            }
        }
        None => quote! {
            #[derive(Clone)]
            pub struct #name {
                message: MessageRepr,
                status: Status,
                result: ResultRepr,
            }
        }
    };
    stream.extend(quote! {
        impl TaskInfo for #name {
            fn path() -> String {
                format!("{}.{}", module_path!(), stringify!(#name))
            }
            fn status(&self) -> Status { self.status.clone() }
            fn status_mut(&mut self) -> &mut Status { &mut self.status }
            fn message(&self) -> MessageRepr { self.message.clone() }
            fn message_mut(&mut self) -> &mut MessageRepr { &mut self.message }
            fn result(&self) -> ResultRepr { self.result.clone() }
            fn result_mut(&mut self) -> &mut ResultRepr { &mut self.result }
        }
    });
    stream.into()
}