extern crate proc_macro;

use core::panic;

use crate::proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Fields, FieldsUnnamed, Type};

struct VariantPair {
    ident: syn::Ident,
    params: Vec<Type>,
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

    let mut ans = Vec::new();

    if let Enum(enum_data) = t.data {
        for variant in enum_data.variants {

            let variant_name = variant.ident;
            let mut variant_member = Vec::new();

            // 解析元组
            // 这样写似乎没有增加可读性。
            if let Fields::Unnamed(FieldsUnnamed{paren_token:_, unnamed:members}) = variant.fields {
                for member in members {
                    variant_member.push(member.ty.clone());
                }
            } else {
                panic!("There is enum field {} that isn't a tuple-like variant", variant_name);
            }

            ans.push(VariantPair{
                ident: variant_name,
                params: variant_member,
            })

        }

    } else {
        panic!("Only enum accepted");
    }
    ans
}

// -- 代码生成 -- //

mod sysy_macro_gen {

    use proc_macro2::{TokenStream, Span};

    use quote::{quote, ToTokens};
    use super::VariantPair;

    /// 实现 Transform 和接收函数
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

    /// 生成接收函数的函数体
    fn gen_accept_fn(variants: &Vec<VariantPair>) -> TokenStream {
        let match_item = variants.iter().map(
            |VariantPair{ident:id, params:p}| {

                let fn_name = gen_transform_fn_name(id);
                let (param_name, _) = gen_param_list(p);

                quote! {
                    Self::#id( #( #param_name),* ) => tr.#fn_name( #( #param_name ),* )
                }
            }
        );
        quote! {
            match self {
                #(#match_item),*
            }
        }
    }

    /// 生成转换器 Trait
    pub(crate) fn impl_transformer(ident: &syn::Ident, variants: &Vec<VariantPair>) -> TokenStream {
        gen_transformer_trait(ident, variants)
    }

    /// 生成转换器 Trait 的内容
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

    /// 生成转换函数
    fn gen_transform_fn(enum_id: &syn::Ident, variant: &VariantPair) -> TokenStream {
        let fn_name = gen_transform_fn_name(&variant.ident);
        let visit_fn_name = gen_visit_fn_name(&variant.ident);
        let variant_name = variant.ident.clone();
        let (fn_params , param_list)= gen_param_list(&variant.params);
        quote! {
            fn #fn_name(&mut self, #( #param_list),* ) -> #enum_id {
                self.#visit_fn_name( #( &#fn_params),* );
                #enum_id::#variant_name( #( #fn_params ),* )
            }
        }
    }

    /// 生成访问函数
    fn gen_visit_fn(variant: &VariantPair) -> TokenStream {
        let fn_name = gen_visit_fn_name(&variant.ident);
        let (_, param_list) = gen_borrow_param_list(&variant.params);
        quote! {
            fn #fn_name(&mut self, #( #param_list),* ) -> () {

            }
        }
    }



    // -- 记号生成 -- //

    /**
     * 生成一个 enum 对应的访问其的类型名
     */
    fn gen_transformer_id(enum_id: &syn::Ident) -> syn::Ident {
        let transformer_name = enum_id.to_string() + "Transformer";
        syn::Ident::new(&transformer_name, Span::call_site())
    }

    /// 根据变体名字，生成转换函数的名字
    fn gen_transform_fn_name(variant_id: &syn::Ident) -> syn::Ident {
        let fn_name = String::from("transform_") + &*to_snake_case(&*variant_id.to_string());
        syn::Ident::new(&fn_name, Span::call_site())
    }

    /// 根据变体的名字，生成访问函数的名字
    fn gen_visit_fn_name(variant_id: &syn::Ident) -> syn::Ident {
        let fn_name = String::from("visit_") + &*to_snake_case(&*variant_id.to_string());
        syn::Ident::new(&fn_name, Span::call_site())
    }

    /// 对每一个变体调用转换函数的参数名，和形参列表。
    fn gen_param_list(variant_param: &Vec<syn::Type>) -> (Vec<syn::Ident>, Vec<TokenStream>){
        let (name, ty) = get_param_name_and_type(variant_param);
        let param_list = name.iter().zip(ty.into_iter()).map(|(n, y)| {
            quote! {
                #n:#y
            }
        }).collect();

        (name, param_list)
    }

    fn gen_borrow_param_list(variant_param: &Vec<syn::Type>) -> (Vec<syn::Ident>, Vec<TokenStream>){
        let (name, ty) = get_param_name_and_type(variant_param);
        let param_list = name.iter().zip(ty.into_iter()).map(|(n, y)| {
            quote! {
                #n:&#y
            }
        }).collect();

        (name, param_list)
    }

    fn get_param_name_and_type(variant_param: &Vec<syn::Type>) -> (Vec::<syn::Ident>, Vec::<syn::Type>) {
        let mut param_names = Vec::new();
        let mut param_type = Vec::new();

        for ty in variant_param.iter() {
            let name = ty.to_token_stream().to_string();
            let snake_name = syn::Ident::new(&to_snake_case(&name), Span::call_site());

            param_names.push(snake_name.clone());
            param_type.push(ty.clone());
        }
        (param_names, param_type)

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