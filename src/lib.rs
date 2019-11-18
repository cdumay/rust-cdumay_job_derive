extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(Task)]
pub fn entry_point_path_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_entry_point_path(&ast)
}

fn impl_entry_point_path(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TaskInfo for #name {
            fn new(message: &MessageRepr, result: Option<ResultRepr>) -> Self {
                Self {
                    message: message.clone(),
                    result: result.unwrap_or(ResultRepr::from(message)),
                    status: Status::Pending,
                }
            }
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
    };
    gen.into()
}