use std::{collections::HashSet, fs};

use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use serde::{Deserialize, Serialize};
use syn::LitStr;

const FILE_JSON: &str = "json";
const FILE_CSV: &str = "csv";

#[derive(Debug)]
struct Option {
    locales_path: String,
    file_type: String,
}

impl syn::parse::Parse for Option {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let params = input.parse::<LitStr>().unwrap();
        let params_value = params.value();

        let params_vec: Vec<&str> = params_value.split(',').collect();

        assert_eq!(
            params_vec.len(),
            2,
            "The passing of two parameters is required"
        );

        let locales_path = params_vec[0].to_string();
        let file_type = params_vec[1].to_string();

        Ok(Self {
            locales_path,
            file_type,
        })
    }
}

pub fn locale(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // 校验参数
    let option = match syn::parse::<Option>(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is empty");
    let current_dir = std::path::PathBuf::from(cargo_dir);
    let locales_path = current_dir.join(option.locales_path);

    let str_list: Vec<EnumDefinition> = match option.file_type.as_str() {
        FILE_JSON => {
            let json_data = fs::read_to_string(locales_path.as_path().to_str().unwrap()).unwrap();
            serde_json::from_str(&json_data).unwrap()
        }
        FILE_CSV => {
            let mut ls: Vec<EnumDefinition> = Vec::new();
            let mut rdr = csv::Reader::from_path(locales_path.as_path().to_str().unwrap())
                .expect("path不存在");

            for result in rdr.deserialize() {
                let record: EnumDefinition = result.unwrap();
                ls.push(record);
            }
            ls
        }
        _ => {
            panic!()
        }
    };

    check_json_data(&str_list);
    // 如果返回的结构中带()这种特殊符号，会自动加上"，恶心
    // 获取 key
    let enum_case = generate_enum_case(&str_list);
    let id_func = generate_id_func_case(&str_list);
    let trans_func = generate_func(&str_list);

    let code = quote! {
        #[derive(Debug,Clone)]
        pub enum I18nKey {
            #(#enum_case)*
        }

        impl I18nKey {
            pub fn trans(&self, locale: Locale) -> String {
                match self {
                    #(#trans_func)*
                }
            }
            pub fn id(&self) -> i32 {
                match self {
                    #(#id_func)*
                }
            }
        }

        impl Default for I18nKey{
            fn default() -> Self {
                I18nKey::Ok
            }
        }

        #[derive(Debug, Clone, Copy, strum_macros::Display)]
        pub enum Locale {
            En,
            Zh,
        }

        impl Default for Locale {
            fn default() -> Self {
                Locale::En
            }
        }

        impl From<&str> for Locale {
            fn from(value: &str) -> Self {
                if value == Locale::En.to_string().as_str() {
                    return Locale::En;
                }
                Locale::Zh
            }
        }
    };
    code.into()
}

#[derive(Debug, Deserialize, Serialize)]
struct EnumDefinition {
    pub id: i32,
    pub key: String,
    pub zh: String,
    pub en: String,
}

fn check_json_data(data_list: &Vec<EnumDefinition>) {
    let mut id_map: HashSet<i32> = HashSet::new();
    let mut key_map: HashSet<String> = HashSet::new();

    for item in data_list {
        match id_map.get(&item.id) {
            Some(r) => {
                panic!("id already exists:{}", r)
            }
            None => id_map.insert(item.id),
        };
        match key_map.get(&item.key.clone()) {
            Some(r) => {
                panic!("i18nkey already exists:{}", r.clone())
            }
            None => key_map.insert(item.key.clone()),
        };
    }
}

///
/// Author         : ClzSkywalker
/// Date           : 2023-07-21
/// Description    : 生成枚举的名字与参数
/// param           {*} str_list
/// return          {*}
///
fn generate_enum_case(str_list: &[EnumDefinition]) -> Vec<TokenStream> {
    let enum_case = str_list
        .iter()
        .map(|variant| {
            let item = syn::Ident::new(&variant.key, proc_macro2::Span::call_site());

            let mut argsstr = String::from("");
            let re = Regex::new(r"%\{(\w+)\}").unwrap();
            let args: Vec<&str> = re
                .captures_iter(&variant.en)
                .map(|item| item.get(1).unwrap().as_str())
                .collect();
            if args.is_empty() {
                return quote! {#item,};
            }

            // 处理注释
            // let mut about=String::from("");
            // about.push_str(args.join(",").as_str());
            // dbg!(about.to_string());
            // let about=about;
            // let _about=syn::Ident::new(&about, proc_macro2::Span::call_site());

            // 处理参数
            let five_fives = std::iter::repeat("String").take(args.len());
            let v = Vec::from_iter(five_fives).join(",");
            argsstr.push_str(v.as_str());
            let args = syn::Ident::new(&argsstr, proc_macro2::Span::call_site());
            quote! {
                #item(#args),
            }
        })
        .collect::<Vec<_>>();

    enum_case
}

fn generate_id_func_case(str_list: &[EnumDefinition]) -> Vec<TokenStream> {
    let enum_case = str_list
        .iter()
        .map(|variant| {
            let item = syn::Ident::new(&variant.key, proc_macro2::Span::call_site());

            let mut argsstr = String::from("");
            let re = Regex::new(r"%\{(\w+)\}").unwrap();
            let args: Vec<&str> = re
                .captures_iter(&variant.en)
                .map(|item| item.get(1).unwrap().as_str())
                .collect();

            let id = variant.id;
            if args.is_empty() {
                return quote! {I18nKey::#item=>#id,};
            }

            // 处理注释
            // let mut about=String::from("");
            // about.push_str(args.join(",").as_str());
            // dbg!(about.to_string());
            // let about=about;
            // let _about=syn::Ident::new(&about, proc_macro2::Span::call_site());

            // 处理参数
            let five_fives = std::iter::repeat("_").take(args.len());
            let v = Vec::from_iter(five_fives).join(",");
            argsstr.push_str(v.as_str());
            let args = syn::Ident::new(&argsstr, proc_macro2::Span::call_site());
            quote! {
                I18nKey::#item(#args)=>#id,
            }
        })
        .collect::<Vec<_>>();

    enum_case
}

///
/// Author         : ClzSkywalker
/// Date           : 2023-07-21
/// Description    : 生成 trans方法
/// param           {*} str_list
/// return          {*}
///
fn generate_func(str_list: &[EnumDefinition]) -> Vec<TokenStream> {
    str_list
        .iter()
        .map(|variant| {
            let enum_name = syn::Ident::new(&variant.key, proc_macro2::Span::call_site());
            let en_lang = &variant.en;
            let zh_lang = &variant.zh;
            // 正则匹配参数
            let re = Regex::new(r"%\{(\w+)\}").unwrap();
            let marches: Vec<_> = re.captures_iter(&variant.en).collect();
            if marches.is_empty() {
                return quote! {
                    I18nKey::#enum_name => match locale {
                        Locale::En => {
                            String::from(#en_lang)
                        }
                        Locale::Zh => {
                            String::from(#zh_lang)
                        },
                    },
                };
            }

            let args_info: Vec<&str> = marches
                .iter()
                .map(|item| item.get(1).unwrap().as_str())
                .collect();

            // 获取参数
            let mut args = Vec::<String>::default();
            let mut replace_code_list = Vec::default();

            for index in 0..args_info.len() {
                let item = "item".to_string() + (index + 1).to_string().as_str();
                args.push(item.clone());
                let mut replace_indentity = String::from("%{");
                replace_indentity.push_str(args_info.get(index).unwrap());
                replace_indentity.push('}');

                let arg_name = syn::Ident::new(
                    args_info.get(index).unwrap(),
                    proc_macro2::Span::call_site(),
                );

                let item = syn::Ident::new(&item, proc_macro2::Span::call_site());

                let code = quote! {
                    let #arg_name = #replace_indentity;
                    lang = lang.replace(#arg_name, #item);
                };

                replace_code_list.push(code);
            }
            let args_str = args.join(",");
            let args_str = syn::Ident::new(&args_str, proc_macro2::Span::call_site());
            let code = quote! {
                I18nKey::#enum_name(#args_str) => match locale {
                    Locale::En => {
                        let mut lang = String::from(#en_lang);
                        #(#replace_code_list)*
                        lang
                    }
                    Locale::Zh => {
                        let mut lang = String::from(#zh_lang);
                        #(#replace_code_list)*
                        lang
                    },
                },
            };
            code
        })
        .collect::<Vec<_>>()
}
