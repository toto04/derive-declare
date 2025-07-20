use inflector::Inflector;
use quote::quote;

// generate the macro for the struct that derives `Declare`
pub fn generate_macro(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &ast.ident;
    // convert struct name to snake case to create the macro name
    let struct_name_snake = struct_name.to_string().to_snake_case();
    // create an identifier for the macro name
    let macro_name_ident = syn::Ident::new(&struct_name_snake, ast.ident.span());

    let priv_struct_name_snake = format!("__declare_{struct_name_snake}");
    let priv_struct_name_ident = syn::Ident::new(&priv_struct_name_snake, ast.ident.span());

    if let syn::Data::Struct(ref data) = ast.data {
        if let syn::Fields::Named(ref _fields) = data.fields {
            // get all the field names from the struct
            // let field_names = fields
            //     .named
            //     .iter()
            //     .map(|f| f.ident.as_ref())
            //     .collect::<Vec<_>>();

            // this creates the outer macro that will be exported
            // the outer macro creates a default instance of the struct and then calls the
            // inner macro to parse the block, generating the assignments for each field
            let expanded = quote! {
                macro_rules! #priv_struct_name_ident {
                    // empty macro
                    () => {
                        #struct_name::default()
                    };

                    ($($token:tt)*) => {{
                        #[allow(clippy::needless_update)]
                        #struct_name {
                          $($token)*,
                          ..Default::default()
                        }
                        // let mut def = #struct_name::default();
                        // derive_declare::__declare_parse_block!({$($token)*}; [#(#field_names),*]; #struct_name);
                        // def
                    }};
                }
                pub(crate) use #priv_struct_name_ident as #macro_name_ident;
            };
            return expanded;
        }
    }

    // catch all
    syn::Error::new(
        ast.ident.span(),
        "Only structs with named fields can derive `Declare`",
    )
    .to_compile_error()
}
