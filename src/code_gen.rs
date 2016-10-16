#[macro_use]
use quote;
use syn;

#[derive(Default, PartialEq, Debug)]
struct GlobalAttrData {
    default_prefix: Option<String>,
    with_default: bool,
}

impl GlobalAttrData {
    fn from_attributes(attrs: &Vec<syn::Attribute>) -> GlobalAttrData {
        let builder_ident = syn::Ident::new("Builder".to_string());
        let derive_ident = syn::Ident::new("derive".to_string());
        let default_ident = syn::Ident::new("Default".to_string());
        let prefix_ident = syn::Ident::new("prefix".to_string());
        let mut data = GlobalAttrData {
            default_prefix: None,
            with_default: false,
        };

        use syn::MetaItem::*;
        use syn::Lit::*;

        // OMFG: So much nesting
        for attr in attrs.iter() {
            if let List(ref ident, ref items) = attr.value {
                if ident == &derive_ident {
                    for attr_item in items.iter() {
                        if let &List(ref ident, _) = attr_item {
                            if *ident == default_ident {
                                data.with_default = true;
                            }
                        }
                    }
                }
            }
            if let NameValue(ref key, Str(ref prefix, _)) = attr.value {
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
}

impl FieldAttrData {
    fn from_attributes(attrs: &Vec<syn::Attribute>) -> FieldAttrData {
        use syn::MetaItem::*;
        use syn::Lit::*;
        let mut data: FieldAttrData = FieldAttrData {
            create_field: true,
            .. Default::default()
        };
        let field_name_ident = syn::Ident::new("field_name".to_string());
        let create_field_ident = syn::Ident::new("should_create".to_string());
        for attr in attrs.iter() {
            match &attr.value {
                &NameValue(ref key, Str(ref value, _)) => {
                    if field_name_ident == key {
                        data.field_name = Some(value.clone());
                    } 
                },
                &NameValue(ref key, Bool(ref value)) => {
                    if create_field_ident == key {
                        data.create_field = value.clone();
                    }
                },
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

fn expand_fields<F>(fields: &Vec<syn::Field>, f: F) -> quote::Tokens
    where F: Fn(&syn::Field) -> quote::Tokens {
    let mut tokens = quote::Tokens::new();
    tokens.append_all(fields.iter().map(f));
    tokens
}

fn get_method_name(field_attrs: &FieldAttrData, field: &syn::Field, mutable: bool, prefix: &Option<String>) -> syn::Ident {
    let _prefix = match prefix {
        &Some(ref string) => string.clone(),
        &None => "".to_string(),
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

fn expand_mutable(data: &CodeGenData, fields: &Vec<syn::Field>) -> quote::Tokens {
    let prefix = &data.attr_data.default_prefix;
    expand_fields(fields, |field| {
        let field_attrs = FieldAttrData::from_attributes(&field.attrs);
        let ident = field.ident.as_ref().unwrap();
        if field_attrs.create_field {
            let method_name = get_method_name(&field_attrs, field, true, &prefix);
            let ty = &field.ty;
            let struct_name = data.struct_name;
            let ty_generics = data.ty_generics;
            quote! {
                fn #method_name(&mut self, __value: #ty) -> &mut #struct_name #ty_generics {
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

fn expand_immutable(data: &CodeGenData, fields: &Vec<syn::Field>) -> quote::Tokens {
    let prefix = &data.attr_data.default_prefix;
    expand_fields(fields, |field| {
        let attr_data = FieldAttrData::from_attributes(&field.attrs);
        let ident = field.ident.as_ref().unwrap();
        if attr_data.create_field {
            let ty = &field.ty;
            let struct_name = data.struct_name;
            let ty_generics = data.ty_generics;
            let method_name = get_method_name(&attr_data, &field, false, &prefix);
            quote! {
                fn #method_name(self, __value: #ty) -> #struct_name #ty_generics {
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
    match body {
        &syn::Body::Struct(ref variant) => {
            match variant {
                &syn::VariantData::Struct(ref fields) => {
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
        &syn::Body::Enum(_) => panic!("Enums are not supported"),
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
