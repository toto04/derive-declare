extern crate proc_macro;

use self::proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, braced, parse::Parse, parse_macro_input, parse2};

// Struct to hold single field assignments
struct FieldAssignment {
    pub field_ident: syn::Ident,
    pub value: syn::Expr,
}

impl Parse for FieldAssignment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let field_ident: Ident = input.parse()?;
        let value: syn::Expr = if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;
            input.parse::<syn::Expr>()?
        } else {
            parse2(field_ident.clone().to_token_stream())?
        };

        Ok(FieldAssignment { field_ident, value })
    }
}

// Struct to hold all field assignments in a block
struct Assignments {
    expressions: Vec<FieldAssignment>,
}

impl Iterator for Assignments {
    type Item = FieldAssignment;

    fn next(&mut self) -> Option<Self::Item> {
        self.expressions.pop()
    }
}

impl Parse for Assignments {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut expressions = Vec::new();
        while !input.is_empty() {
            let expr = input.parse::<FieldAssignment>()?;
            expressions.push(expr);
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Assignments { expressions })
    }
}

// Struct that represents the declarative block
struct DeclarativeBlock {
    pub expressions: Assignments,
    pub fields: syn::ExprArray,
    pub struct_ident: syn::Ident,
}

impl Parse for DeclarativeBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);
        let expressions = content.parse::<Assignments>()?; // literally the only important thing here
        input.parse::<syn::Token![;]>()?;
        let fields: syn::ExprArray = input.parse()?; // ahh passing the fields around
        input.parse::<syn::Token![;]>()?;
        let struct_ident: syn::Ident = input.parse()?; // just for error messages

        Ok(DeclarativeBlock {
            expressions,
            fields,
            struct_ident,
        })
    }
}

// Function to parse the declarative block and generate the corresponding code
pub fn parse_declarative_block(ast: TokenStream) -> TokenStream {
    let check_block = parse_macro_input!(ast as DeclarativeBlock);

    // get all the field names from the struct
    let fields = check_block
        .fields
        .elems
        .iter()
        .map(|f| f.to_token_stream().to_string())
        .collect::<Vec<_>>();

    let struct_ident = &check_block.struct_ident;

    let parsed_expressions = check_block.expressions.map(|expr| {
        // generate the assignment for each expression
        let field_ident = expr.field_ident;
        let value = expr.value;

        // check if the field is in the list of fields
        if !fields.iter().any(|f| f == &field_ident.to_string()) {
            // here's the real syntactic sugar ;)
            return syn::Error::new(
                field_ident.span(),
                format!("Field '{field_ident}' does not exist in struct '{struct_ident}'."),
            )
            .to_compile_error();
        }

        // generate the assignment statement for each field
        // "def" is the default instance of the struct, created by the outer macro
        quote! {
            def.#field_ident = #value;
        }
    });

    // just a list of assignments
    let expanded = quote! {
        #(
            #parsed_expressions
        )*
    };

    TokenStream::from(expanded)
}
