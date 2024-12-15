// /// Create a PeripheralReference to a pin or resource managed by the Peripherals struct.
// ///
// /// This macro is unsafe, without the condition that only one copy of this reference is used
// /// by the user, as otherwise any drivers created using that pin or resource will not work
// /// correctly.
// #[macro_export]
// macro_rules! pref {
//     // Take in a list of expressions, which are some kind of peripheral, and
//     // create an unsafe reference to them.
//     ($($peripheral:expr),+) => {{
//             use esp_idf_hal::{peripheral::Peripheral, gpio::IOPin};
//             let mut peripherals = vec![];
//             $(
//                 peripherals.push(unsafe { $peripheral.clone_unchecked() }
//                     .downgrade()
//                     .into_ref());
//             )+
//             peripherals
//     }};
// }

use proc_macro::Span;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident,
};

struct PrefMacroInput {
    _ident: syn::Ident,
    _colon: syn::Token![:],
    range: syn::Expr,
}

impl Parse for PrefMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ident = input.parse()?;
        let _colon = input.parse()?;
        let range = input.parse()?;

        Ok(PrefMacroInput {
            _ident,
            _colon,
            range,
        })
    }
}

#[proc_macro]
pub fn pref(input: TokenStream) -> TokenStream {
    let PrefMacroInput { range, .. } = parse_macro_input!(input as PrefMacroInput);

    let (start, end) = if let syn::Expr::Range(ref range_expr) = range {
        let start = match range_expr.start.as_deref() {
            Some(syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(ref lit),
                ..
            })) => lit.base10_parse::<u32>().unwrap(),
            _ => panic!("Expected integer range start."),
        };

        let end = match range_expr.end.as_deref() {
            Some(syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(ref lit),
                ..
            })) => lit.base10_parse::<u32>().unwrap(),
            _ => panic!("Expected integer range end."),
        } + 1;

        (start, end)
    } else {
        panic!("Expected a range expression like 1..21");
    };

    let pins = (start..end).map(|n| {
        let gpio_ident = Ident::new(&format!("gpio{}", n), Span::call_site().into());
        quote! {
            unsafe { peripherals.pins.#gpio_ident.clone_unchecked() }.downgrade().into_ref()
        }
    });

    let expanded = quote! {{
        use esp_idf_hal::gpio::IOPin;
        let mut v = Vec::new();
        #(v.push(#pins);)*
        v
    }};

    TokenStream::from(expanded)
}
