#![feature(extern_in_paths)]
#![feature(try_trait)]

use extern::burgundy_lib as burgundy;
use extern::proc_macro;
use extern::proc_macro2::TokenStream;
use extern::syn;
#[macro_use]
extern crate quote;

use std::str::FromStr;

#[proc_macro_derive(Request, attributes(request, query, body))]
pub fn request_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: syn::DeriveInput = syn::parse(input).unwrap();
  let gen = impl_request(&input);
  gen.into()
}

fn impl_request(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let struct_name = &input.ident;
    let inner_impl = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => impl_request_for_struct(struct_name, &input.attrs, &fields.named),
        _ => panic!("Burgundy only supports describing APIs as structs"),
    };

    quote!(#inner_impl)
}

fn impl_request_for_struct(
    name: &syn::Ident,
    syn_attrs: &[syn::Attribute],
    syn_fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> TokenStream {
    let maybe_attributes = parse_header_attributes(syn_attrs);
    let attributes = maybe_attributes.validate();
    let to_url_path = gen_to_url_path(attributes, syn_fields);

    quote! {
        #[allow(unused_variables, dead_code, unreachable_code)]
        #[doc(hidden)]
        impl ::burgundy::Request for #name {
            #to_url_path
        }
    }
}

fn parse_header_attributes(
    syn_attrs: &[syn::Attribute],
) -> MaybeAttributes {
    let mut attributes = MaybeAttributes {
        method : None,
        path : None,
    };

    for syn_attr in syn_attrs {
        let maybe_syn_meta = syn_attr.interpret_meta();

        if let Some(syn_meta) = maybe_syn_meta {
            match syn_meta {
                syn::Meta::List(syn::MetaList { ident, nested, .. }) => {
                    let ident_str = ident.to_string();

                    // #[request(method="POST", path="/api/blah")]
                    if ident_str == "request" {
                        nested.iter().for_each(|nested_attr| {
                            match nested_attr {
                                syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue { ident, lit : syn::Lit::Str(lit_str), .. })) => {
                                    let attr_ident_str = ident.to_string();

                                    if attr_ident_str == "method" {
                                        attributes.set_method_from_str( lit_str.value() );
                                    } else if attr_ident_str == "path" {
                                        attributes.set_path( lit_str.value() );
                                    } else {
                                        panic!("Unknown attribute given {:?}", nested_attr);
                                    }
                                },
                                _ => {
                                    panic!("Unknown attribute given {:?}", nested_attr);
                                }
                            }
                        })
                    } else {
                        error_if_struct_attr(ident);
                    }
                },
                syn::Meta::Word(ident) => {
                    error_if_struct_attr(ident);
                },
                syn::Meta::NameValue(syn::MetaNameValue { ident, .. }) => {
                    error_if_struct_attr(ident);
                },
            }
        }
    }

    attributes
}

fn gen_to_url_path(
    attributes: Attributes,
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> TokenStream {
  quote! {
    fn to_url_path(&self) -> String {
      #attributes.path
    }
  }
}

struct MaybeAttributes {
    method : Option<burgundy::Method>,
    path : Option<String>,
}

impl MaybeAttributes {
    fn set_method_from_str( &mut self, method : String ) {
        if self.method.is_some() {
            panic!("Request method has been set twice");
        }

        let method = burgundy::Method::from_str(&method).unwrap_or_else(|_| panic!("Method not recognised '{}'", method));
        self.method = Some(method);
    }

    fn set_path( &mut self, path : String ) {
        if self.path.is_some() {
            panic!("Path is set twice");
        }

        self.path = Some(path);
    }

    fn validate( self ) -> Attributes {
        Attributes {
            method : self.method.unwrap_or_else(|| panic!("No method provided")),
            path : self.path.unwrap_or_else(|| panic!("No path provided")),
        }
    }
}

struct Attributes {
    method : burgundy::Method,
    path : String,
}

fn error_if_struct_attr(
    ident : syn::Ident,
) {
    let ident_str = ident.to_string();
    if ident_str == "query" || ident_str == "body" {
        panic!("'{}' is not supported on the struct head", ident_str);
    }
}

fn error_if_struct_head_attr(
    ident : syn::Ident,
) {
    let ident_str = ident.to_string();
    if ident_str == "request" {
        panic!("'{}' is not supported on the struct fields", ident_str);
    }
}