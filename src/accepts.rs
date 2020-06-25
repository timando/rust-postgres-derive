use std::iter;
use syn::Ident;
use quote::Tokens;

use enums::Variant;
use composites::Field;

pub fn domain_body(name: &str, field: &syn::Field) -> Tokens {
    let ty = &field.ty;

    quote! {
        if type_.name() != #name {
            return false;
        }

        match *type_.kind() {
            ::postgres::types::Kind::Domain(ref type_) => {
                <#ty as ::postgres::types::ToSql>::accepts(type_)
            }
            _ => false,
        }
    }
}

pub fn enum_body(name: &str, variants: &[Variant]) -> Tokens {
    let num_variants = variants.len();
    let variant_names = variants.iter().map(|v| &v.name);

    quote! {
        if type_.name() != #name {
            return false;
        }

        match *type_.kind() {
            ::postgres::types::Kind::Enum(ref variants) => {
                variants.iter().all(|v| {
                    match &**v {
                        #(
                            #variant_names => true,
                        )*
                        // Allow extra variants at the type level
                        _ => true,
                    }
                })
            }
            _ => false,
        }
    }
}

pub fn composite_body(name: &str, trait_: &str, fields: &[Field]) -> Tokens {
    let num_fields = fields.len();
    let trait_ = Ident::from(trait_);
    let traits = iter::repeat(&trait_);
    let field_names = fields.iter().map(|f| &f.name);
    let field_types = fields.iter().map(|f| &f.type_);

    quote! {
        if type_.name() != #name {
            return false;
        }

        match *type_.kind() {
            ::postgres::types::Kind::Composite(ref fields) => {
                // If the composite has more fields than the rust type, allow it but ignore the extras
                if fields.len() < #num_fields {
                    return false;
                }

                fields.iter().all(|f| {
                    match f.name() {
                        #(
                            #field_names => {
                                <#field_types as ::postgres::types::#traits>::accepts(f.type_())
                            }
                        )*
                        // Allow extra fields in accepts.
                        _ => true,
                    }
                })
            }
            _ => false,
        }
    }
}
