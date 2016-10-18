#![allow(unknown_lints, needless_borrow)]
#[macro_use]
use quote;
use syn;
use syn::MetaItem::*;
use syn::Lit::*;

#[derive(Default, PartialEq, Debug)]
struct GlobalAttrData {
    default_prefix: Option<String>,
}

fn get_smelter_attributes(attrs: &[syn::Attribute]) -> Vec<syn::MetaItem> {
    let smelter_ident = syn::Ident::new("smelter".to_string());
    attrs.into_iter().filter(|attr| {
        if let List(ref ident, _) = attr.value {
            return ident == &smelter_ident
        }
        false
    })
    .flat_map(|attr| {
        match attr.value {
            List(_, ref items) => items.clone(),
            _ => panic!("Attempted to extract item from non-list"),
        }
    })
    .collect()
}

impl GlobalAttrData {
    fn from_attributes(attrs: &[syn::Attribute]) -> GlobalAttrData {
        let prefix_ident = syn::Ident::new("prefix".to_string());
        let mut data = GlobalAttrData {
            default_prefix: None,
        };
        let smelter_attrs = get_smelter_attributes(attrs);

        for item in smelter_attrs {
            if let NameValue(ref key, Str(ref prefix, _)) = item {
                if *key == prefix_ident {
                    data.default_prefix = Some(prefix.clone());
                }
            }
        }
        data
    }
}

#[derive(Debug, PartialEq, Default)]
struct FieldAttrData {
    field_name: Option<String>,
    create_field: bool,
    force_public: bool,
}

impl FieldAttrData {
    fn from_attributes(attrs: &[syn::Attribute]) -> FieldAttrData {
        use syn::MetaItem::*;
        use syn::Lit::*;
        let mut data: FieldAttrData = FieldAttrData {
            create_field: true,
            .. Default::default()
        };
        let field_name_ident = syn::Ident::new("field_name".to_string());
        let create_field_ident = syn::Ident::new("should_create".to_string());
        let force_public_ident = syn::Ident::new("force_public".to_string());
        let smelter_attrs = get_smelter_attributes(attrs);
        for item in smelter_attrs {
            match item {
                NameValue(ref key, Str(ref value, _)) => {
                    if field_name_ident == key {
                        data.field_name = Some(value.clone());
                    } 
                },
                NameValue(ref key, Bool(ref value)) => {
                    if create_field_ident == key {
                        data.create_field = *value;
                    } 
                },
                Word(ref ident) => {
                    if force_public_ident == ident {
                        data.force_public = true;
                    }
                }
                _ => ()
            }
        }

        data
    }
}

#[derive(PartialEq, Debug)]
struct CodeGenData<'a> {
    attr_data: &'a GlobalAttrData,
    fields: &'a Vec<syn::Field>,
    struct_name: &'a syn::Ident,
    ty_generics: &'a syn::Generics,
}

fn expand_fields<F>(fields: &[syn::Field], f: F) -> quote::Tokens
    where F: Fn(&syn::Field) -> quote::Tokens {
    let mut tokens = quote::Tokens::new();
    tokens.append_all(fields.iter().map(f));
    tokens
}

fn get_method_name(field_attrs: &FieldAttrData, field: &syn::Field, mutable: bool, prefix: &Option<String>) -> syn::Ident {
    let _prefix = match *prefix {
        Some(ref string) => string.clone(),
        None => "".to_string(),
    };
    let base_name = match field_attrs.field_name {
        Some(ref field_name) => {
            format!("{}{}", _prefix, field_name)
        },
        None => {
            format!("{}{}", _prefix, &field.ident.as_ref().unwrap())
        }
    };

    if mutable {
        syn::Ident::new(format!("{}_mut", base_name))
    } else {
        syn::Ident::new(base_name)
    }
}

fn get_visibility(field_attrs: &FieldAttrData, field: &syn::Field) -> syn::Ident {
    let is_public = field_attrs.force_public || match field.vis {
        syn::Visibility::Public => true,
        _ => false,
    };
    if is_public {
        syn::Ident::new("pub")
    } else {
        syn::Ident::new("")
    }
}

fn expand_mutable(data: &CodeGenData, fields: &[syn::Field]) -> quote::Tokens {
    let prefix = &data.attr_data.default_prefix;
    expand_fields(fields, |field| {
        let field_attrs = FieldAttrData::from_attributes(&field.attrs);
        let ident = field.ident.as_ref().unwrap();
        if field_attrs.create_field {
            let method_name = get_method_name(&field_attrs, field, true, prefix);
            let pub_ident = get_visibility(&field_attrs, &field);
            let ty = &field.ty;
            let struct_name = data.struct_name;
            let ty_generics = data.ty_generics;
            quote! {
                #pub_ident fn #method_name(&mut self, __value: #ty) -> &mut #struct_name #ty_generics {
                    self. #ident = __value;
                    self
                }
            }
        } else {
            quote! {
                // Omitted #ident
            }
        }
    })
}

fn expand_immutable(data: &CodeGenData, fields: &[syn::Field]) -> quote::Tokens {
    let prefix = &data.attr_data.default_prefix;
    expand_fields(fields, move |field| {
        let attr_data = FieldAttrData::from_attributes(&field.attrs);
        let ident = field.ident.as_ref().unwrap();
        if attr_data.create_field {
            let method_name = get_method_name(&attr_data, &field, false, prefix);
            let pub_ident = get_visibility(&attr_data, &field);
            let ty = &field.ty;
            let struct_name = data.struct_name;
            let ty_generics = data.ty_generics;
            quote! {
                #pub_ident fn #method_name(self, __value: #ty) -> #struct_name #ty_generics {
                    #struct_name { #ident : __value, .. self }
                }
            }
        } else {
            quote! {
                // something is silly here
            }
        }
    })
}

fn expand_methods(ast: &syn::MacroInput) -> quote::Tokens {
    let body = &ast.body;
    match *body {
        syn::Body::Struct(ref variant) => {
            match *variant {
                syn::VariantData::Struct(ref fields) => {
                    let struct_name = &ast.ident;
                    let (_, ty_generics, _) = ast.generics.split_for_impl();
                    let attr_data = GlobalAttrData::from_attributes(&ast.attrs);
                    let gen_data = CodeGenData {
                        ty_generics: &ty_generics,
                        struct_name: &struct_name,
                        attr_data: &attr_data,
                        fields: fields,
                    };
                    let immutable_build_methods = expand_immutable(&gen_data, fields);
                    let mutable_build_methods = expand_mutable(&gen_data, fields);
                    quote! {
                        #immutable_build_methods

                        #mutable_build_methods
                    }
                },
                _ => panic!("Derive builder is not supported for tuple structs and the unit struct")
            }
        },
        syn::Body::Enum(_) => panic!("Enums are not supported"),
    }
}


pub fn expand_builder(ast: syn::MacroInput) -> quote::Tokens {
    let methods = expand_methods(&ast);
    let struct_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        #[allow(unused_attributes)]
        #ast

        #[allow(dead_code)]
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #methods
        }
    }
}
