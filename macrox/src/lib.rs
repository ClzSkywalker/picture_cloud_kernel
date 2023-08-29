extern crate proc_macro;

mod locale;

///
/// Author         : ClzSkywalker
/// Date           : 2023-08-29
/// Description    : 两参数 "locales/locale.csv,csv" 
/// return          {*}
///
#[proc_macro]
pub fn generate_i18n_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    locale::locale(input)
}
