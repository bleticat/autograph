use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Wraps the async method body in `self.db.begin(async move |uow| { ... }).await`,
/// eliminating the transaction boilerplate from command methods.
///
/// Apply this attribute to any `async` method on a struct that has a `db` field
/// implementing `Database`. Inside the body, use `uow` to access repositories
/// from the unit of work.
///
/// # Example
///
/// ```rust,ignore
/// #[transaction]
/// pub async fn add(&self, title: &str) -> Result<Project, AppErr> {
///     let title = title.to_owned();
///     uow.project().save(Project { id: Uuid::nil(), title }).await
/// }
/// ```
///
/// The above expands to:
///
/// ```rust,ignore
/// pub async fn add(&self, title: &str) -> Result<Project, AppErr> {
///     self.db.begin(async move |uow| {
///         let title = title.to_owned();
///         uow.project().save(Project { id: Uuid::nil(), title }).await
///     }).await
/// }
/// ```
#[proc_macro_attribute]
pub fn transaction(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);
    let stmts = func.block.stmts.clone();

    let new_block: syn::Block = syn::parse_quote! {
        {
            self.db.begin(async move |uow| {
                #(#stmts)*
            }).await
        }
    };

    func.block = Box::new(new_block);
    quote! { #func }.into()
}
