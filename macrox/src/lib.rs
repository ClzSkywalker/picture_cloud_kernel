extern crate proc_macro;

mod locale;

#[proc_macro]
pub fn generate_i18n_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    locale::locale(input)
}
