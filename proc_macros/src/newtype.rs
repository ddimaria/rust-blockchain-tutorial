use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse2, DeriveInput, FieldsUnnamed};

pub fn append(input: TokenStream2) -> TokenStream2 {
    let DeriveInput { ident, data, .. } = parse2(input).unwrap();
    let error = format!(
        "{} is not a new type struct (e.g. struct Block(SimpleBlock))",
        ident
    );

    let inner_ident = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
            _ => panic!("{}", error),
        },
        _ => panic!("{}", error),
    };

    let output = quote! {
        impl std::ops::Deref for #ident {
            type Target = #inner_ident;

            fn deref(&self) -> &#inner_ident {
                &self.0
            }
        }

        impl std::ops::DerefMut for #ident {
            fn deref_mut(&mut self) -> &mut #inner_ident {
                &mut self.0
            }
        }

        impl Into<#inner_ident> for #ident {
            fn into(self) -> #inner_ident {
                self.0
            }
        }
    };

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_syntax() {
        let input: TokenStream2 = quote! { pub(crate) struct Block(SimpleBlock); };
        let output = append(input.into());
        let expected = quote! {
            impl std::ops::Deref for Block {
                type Target = SimpleBlock;

                fn deref(&self) -> &SimpleBlock {
                    &self.0
                }
            }

            impl std::ops::DerefMut for Block {
                fn deref_mut(&mut self) -> &mut SimpleBlock {
                    &mut self.0
                }
                impl Into<SimpleBlock> for Block {
            }

                fn into(self) -> SimpleBlock {
                    self.0
                }
            }
        };

        assert_eq!(expected.to_string(), output.to_string());
    }
}
