use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(ffxiv))]
struct DurationalBuffDerive {
    ident: syn::Ident,
    duration: u8,
}

pub(super) fn durational_buff_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let duration_info: DurationalBuffDerive = match FromDeriveInput::from_derive_input(&ast) {
        Ok(val) => val,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    let (ident, duration) = (duration_info.ident, duration_info.duration);

    quote!(
        #[automatically_derived]
        impl DurationalBuff for #ident {
            const BASE_DURATION: u8 = #duration;

            fn activate(self, bonus: u8) -> Self {
                Self(Self::BASE_DURATION + bonus)
            }

            fn decay(self) -> Self {
                match self {
                    Self(0) => Self(0),
                    Self(val) => Self(val-1)
                }
            }

            fn remaining_duration(&self) -> Option<u8> {
                if self.0 > 0 {
                    Some(0)
                } else {
                    None
                }
            }
        }
    )
    .into()
}
