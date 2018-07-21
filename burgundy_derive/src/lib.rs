#![feature(extern_in_paths)]

use extern::burgundy_lib as burgundy;
use extern::proc_macro;
use extern::proc_macro2::TokenStream;
use extern::syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(Request, attributes(request))]
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
    let attributes = parse_header_attributes(syn_attrs);
    let to_url_path = gen_to_url_path(attributes, syn_fields);

    quote! {
        #[allow(unused_variables, dead_code, unreachable_code)]
        #[doc(hidden)]
        impl ::burgundy::Request for #name {
            #to_url_path
        }
    }
}

fn gen_to_url_path(
    attributes: Attributes,
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> TokenStream {
  quote! {
    fn to_url_path(&self) -> String {
      "/repos/Microsoft".to_string()
    }
  }
}

fn parse_header_attributes(
    syn_attrs: &[syn::Attribute],
) -> HeaderAttributes {
    let attributes = Attributes {
        method : None,
    };

    for syn_attr in syn_attrs {
        let maybe_syn_meta = syn_attr.interpret_meta();

        if let Some(syn_meta) = maybe_syn_meta {
            match syn_meta {
                syn::Meta::List(syn::MetaList { ident, attrs_attrs }) => {
                    let ident_str = ident.to_string();

                    if ident_str == "request" {
                        attrs_attrs.iter().for_each(|attrs_attr| {
                            match attrs_attr {
                                syn::NestedMeta::Meta(syn::Meta::NameValue { ident, _lit : syn::Lit::Str(lit_str) }) => {
                                    let attr_ident_str = ident.to_string();

                                    if attr_ident_str == "method" {
                                        attributes.set_method_from_str( lit_str.value() );
                                    } else if attr_ident_str == "path" {
                                        attributes.set_path( lit_str.value() );
                                    }
                                },
                                _ => {
                                    // do nothing
                                }
                            }
                        })
                    }
                },
                _ => {
                    // do nothing
                },
            }
        }
    }

    attributes
}

struct HeaderAttributes {
    method : Option<burgundy::Method>
    path : Option<String>
}

impl HeaderAttributes {
    fn set_method_from_str( &mut self, method : &str ) -> {
        if self.method.is_some() {
            panic!("Request method has been set twice");
        }

        let method = burgundy::Method::parse(method).map_err(|_| panic!("Method not recognised '{}'", method))?;
        self.method = Some(method);
    }

    fn set_path( &mut self, path : String ) -> {
        if self.path.is_some() {
            panic!("Path is set twice");
        }

        self.path = Some(path);
    }
}
j