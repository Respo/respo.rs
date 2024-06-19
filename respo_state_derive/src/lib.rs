//! following <https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro>

use proc_macro::TokenStream;

use quote::quote;

/// implement conversion between `RespoState` and `serde_json::Value`.
/// if you prefer implementing `RespoState` manually, you can implement `backup` and `restore_from` by yourself.
#[proc_macro_derive(RespoState)]
pub fn respo_state_macro_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).expect("parse failed for RespoState macro");
  impl_respo_state_macro(&ast)
}

fn impl_respo_state_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
    impl RespoState for #name {
      fn backup(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self).ok()
      }
      fn restore_from(&mut self, s: &serde_json::Value) -> Result<(), String> {
        match serde_json::from_value(s.to_owned()) {
          Ok(v) => {
            *self = v;
            Ok(())
          }
          Err(e) => Err(format!("failed to deserialize: {:?}", e)),
        }
      }
    }
  };
  gen.into()
}
