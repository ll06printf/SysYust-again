extern crate proc_macro;

use core::panic;

use crate::proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Fields, FieldsUnnamed, Type, TypePath};

struct VariantPair {
    ident: syn::Ident,
    structure: TypePath,
}


/**
 * 为enum生成转换器。要求enum的所有变体均为只有一个元素的元组型。
 */
#[proc_macro_derive(Transformable)]
pub fn transformable_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let id = input.ident.clone();

    let elements = get_enum_variant_names(input);
    let accept = sysy_macro_gen::impl_accept_fn(&id, &elements);
    let transformer = sysy_macro_gen::impl_transformer(&id, &elements);
    TokenStream::from(proc_macro2::TokenStream::from_iter([accept, transformer].into_iter()))
}

/**
 * 检查并提取enum的所有变体
 */
fn get_enum_variant_names(t: DeriveInput) -> Vec<VariantPair> {
    use syn::Data::Enum;

    let enum_id = t.ident;
    let mut ans = Vec::new();

    if let Enum(enum_data) = t.data {
        for variant in enum_data.variants {

            let variant_name = variant.ident;

            // 解析元组
            // 这样写似乎没有增加可读性。
            if let Fields::Unnamed(FieldsUnnamed{paren_token:_, unnamed:members}) = variant.fields {
                let sct_ty =  if members.len() > 1 {
                    panic!("Every enum({}) tuple-like field({}) are allowed to have exactly one member", enum_id, variant_name);
                } else {
                    let sct_ty = members[0].ty.clone();
                    match sct_ty {
                        Type::Path(ty_path) => ty_path,
                        _ => panic!("Only type path allowed in enum field"),
                    }
                };

                ans.push(VariantPair{
                    ident: variant_name,
                    structure: sct_ty,
                })

            } else {
                panic!("There is enum field {} that isn't a tuple-like", variant_name);
            }

        }

    } else {
        panic!("Only enum accepted");
    }
    return ans;
}

// -- 代码生成 -- //

mod sysy_macro_gen {

    use proc_macro2::{TokenStream, Span};

    use quote::quote;
    use super::VariantPair;

    pub(crate) fn impl_accept_fn(ident: &syn::Ident, variants: &Vec<VariantPair>) -> TokenStream {
        let transformer_id = gen_transformer_id(ident);
        let accept_function = gen_accept_fn(variants);
        quote! {
            impl #ident {
                fn transform(self, tr: &mut impl #transformer_id) -> Self {
                    #accept_function
                }
            }
        }
    }

    fn gen_accept_fn(variants: &Vec<VariantPair>) -> TokenStream {
        let match_item = variants.iter().map(
            |VariantPair{ident:id, structure:_}| {
                let fn_name = gen_transform_fn_name(id);
                quote! {
                    Self::#id(v) => tr.#fn_name(v)
                }
            }
        );
        quote! {
            match self {
                #(#match_item),*
            }
        }
    }

    pub(crate) fn impl_transformer(ident: &syn::Ident, variants: &Vec<VariantPair>) -> TokenStream {
        gen_transformer_trait(ident, variants)
    }

    fn gen_transformer_trait(ident: &syn::Ident, variants: &Vec<VariantPair>) -> TokenStream {
        let transformer_name = gen_transformer_id(ident);
        let transform_functions= variants.iter().map(|v| gen_transform_fn(ident, v));
        let visit_functions = variants.iter().map(|v| gen_visit_fn(v));
        quote! {
            trait #transformer_name {
                #(#visit_functions)*
                #(#transform_functions)*
            }
        }
    }

    fn gen_transform_fn(enum_id: &syn::Ident, variant: &VariantPair) -> TokenStream {
        let fn_name = gen_transform_fn_name(&variant.ident);
        let visit_fn_name = gen_visit_fn_name(&variant.ident);
        let variant_name = variant.ident.clone();
        let fn_param = variant.structure.clone();
        quote! {
            fn #fn_name(&mut self, param: #fn_param) -> #enum_id {
                self.#visit_fn_name(&param);
                #enum_id::#variant_name(param)
            }
        }
    }

    fn gen_visit_fn(variant: &VariantPair) -> TokenStream {
        let fn_name = gen_visit_fn_name(&variant.ident);
        let fn_param = variant.structure.clone();
        quote! {
            fn #fn_name(&mut self, param: &#fn_param) -> () {

            }
        }
    }



    // -- id 生成 -- //

    /**
     * 生成一个 enum 对应的访问其的类型名
     */
    fn gen_transformer_id(enum_id: &syn::Ident) -> syn::Ident {
        let transformer_name = enum_id.to_string() + "Transformer";
        syn::Ident::new(&transformer_name, Span::call_site())
    }

    fn gen_transform_fn_name(variant_id: &syn::Ident) -> syn::Ident {
        let fn_name = String::from("transform_") + &*to_snake_case(&*variant_id.to_string());
        syn::Ident::new(&fn_name, Span::call_site())
    }

    fn gen_visit_fn_name(variant_id: &syn::Ident) -> syn::Ident {
        let fn_name = String::from("visit_") + &*to_snake_case(&*variant_id.to_string());
        syn::Ident::new(&fn_name, Span::call_site())
    }
    fn to_snake_case(s: &str) -> String {
        let mut snake = String::new();
        let mut prev_char = '_'; // 初始状态假设前一个字符是分隔符

        for c in s.chars() {
            if c.is_uppercase() {
                // 前一个字符不是分隔符且当前是大写字母时，插入下划线
                if !prev_char.is_ascii_punctuation() && prev_char != '_' {
                    snake.push('_');
                }
                snake.push(c.to_ascii_lowercase());
            } else if c == '-' || c == ' ' {
                // 将连字符和空格转换为下划线
                if !snake.ends_with('_') {
                    snake.push('_');
                }
            } else {
                // 直接添加小写字符
                snake.push(c);
            }
            prev_char = c;
        }

        // 移除首尾多余的下划线并转为全小写
        snake.trim_matches('_').to_ascii_lowercase()
    }

}