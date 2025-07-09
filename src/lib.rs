use proc_macro::TokenStream;
use syn::parse_macro_input;

mod macrogen;
mod parse;

///
/// `Declare` is a procedural macro that generates itself a macro for declaratively
/// creating instances of a struct with named fields.
///
/// A struct that derives `Declare` will have a macro named after the struct in
/// snake_case. The struct must implement the `Default` trait, as the generated
/// macro will use the default values for any fields not specified in the macro
/// invocation.
///
/// Supports partial initialization, and property value shorthand.
///
/// # Example usage:
/// ```rust
/// #[derive(Declare, Debug, Default)]
/// pub struct MyStruct {
///     pub field_one: String,
///     pub field_two: i32,
/// }
///
/// let my_struct: MyStruct = my_struct! {
///     field_one: "Hello".to_string(),
/// };
///
/// assert_eq!(my_struct.field_one, "Hello");
/// assert_eq!(my_struct.field_two, 0);
/// ```
///
#[proc_macro_derive(Declare)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    macrogen::generate_macro(ast).into()
}

#[doc(hidden)]
#[proc_macro]
pub fn __declare_parse_block(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse::parse_declarative_block(input)
}
