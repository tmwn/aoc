extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

// #[proc_macro]
// pub fn make_answer(_item: TokenStream) -> TokenStream {
//     "fn answer() -> u32 { 42 }".parse().unwrap()
// }

// #[proc_macro_derive(Parse, attributes(before, split))]
// pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
//     "fn hoge() -> u32 { 42 }".parse().unwrap()
// }

// #[proc_macro_attribute]
// pub fn aocio_wip(attr: TokenStream, item: TokenStream) -> TokenStream {
//     println!("attr: \"{}\"", attr.to_string());
//     println!("item: \"{}\"", item.to_string());

//     let input = parse_macro_input!(item as ItemFn);
//     let name = &input.sig.ident;

//     let (pat, ty) = match &input.sig.inputs[0] {
//         syn::FnArg::Receiver(_) => panic!(""),
//         syn::FnArg::Typed(x) => (&x.pat, &x.ty),
//     };

//     println!("{:#?}", &pat); // (order, cards)
//     println!("{:#?}", &ty); // Tuple<...>
//     let q = quote! {
//         fn #name(x: i32) -> i32 { x * 2 }
//     };
//     q.into()
// }

#[proc_macro_attribute]
pub fn aocio(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // println!("attr: \"{}\"", attr.to_string());
    // println!("item: \"{}\"", item.to_string());

    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;

    let (pat, ty) = match &input.sig.inputs[0] {
        syn::FnArg::Receiver(_) => panic!(""),
        syn::FnArg::Typed(x) => (&x.pat, &x.ty),
    };

    // println!("{:#?}", pat); // (order, cards)
    // println!("{:#?}", ty); // Tuple<...>

    let mut parser = Parser::new(name.to_string());

    let (n_pat, n_ty, def) = parser.parse(pat, ty);

    let q = quote! {
        fn #name(#n_pat: #n_ty) -> usize {
            (x[0] + x[1]) as usize
        }
        #def
    };
    q.into()
}

struct Parser {
    id: usize,
    func_name: String,
}

impl Parser {
    fn new(func_name: String) -> Parser {
        Parser { id: 0, func_name }
    }

    fn parse<'a>(
        &mut self,
        pat: &'a syn::Pat,
        ty: &'a syn::Type,
    ) -> (syn::Pat, syn::Type, proc_macro2::TokenStream) {
        if let Some(in_vec) = try_inner("Vec", ty) {
            let name = self.new_id("Vec", Span::call_site());
            let i = try_ident(pat).unwrap();
            let inner_ty = in_vec[0].ty;
            let sp = in_vec[0]
                .punct
                .unwrap_or(&syn::LitStr::new("\n", Span::call_site()))
                .clone();
            return (
                syn::parse_quote!(
                    #name(#i)
                ),
                syn::parse_quote!(
                    #name
                ),
                quote!(
                    struct #name(Vec<#inner_ty>);
                    impl std::str::FromStr for #name {
                        type Err = <#inner_ty as std::str::FromStr>::Err;

                        fn from_str(s: &str) -> Result<Self, Self::Err> {
                            Ok(#name(s
                                .split(#sp)
                                .filter(|x| !x.trim().is_empty())
                                .map(|x| x.trim().parse())
                                .collect::<Result<_, _>>()?))
                        }
                    }
                ),
            );
        }
        (pat.clone(), ty.clone(), quote!())
    }

    fn new_id(&mut self, prefix: &str, span: proc_macro2::Span) -> Ident {
        self.id += 1;
        proc_macro2::Ident::new(&format!("__{}{}_{}", prefix, self.id, self.func_name), span)
    }
}

fn try_ident<'a>(pat: &syn::Pat) -> Option<&syn::Ident> {
    if let syn::Pat::Ident(x) = pat {
        Some(&x.ident)
    } else {
        None
    }
}

fn try_inner<'a>(wrapper: &str, ty: &'a syn::Type) -> Option<Vec<TypePunct<'a>>> {
    let tp = if let syn::Type::Path(tp) = ty {
        tp
    } else {
        return None;
    };
    let ss = if tp.path.segments.len() == 1 && tp.path.segments[0].ident == wrapper {
        &tp.path.segments[0]
    } else {
        return None;
    };
    let inner = if let syn::PathArguments::AngleBracketed(iter) = &ss.arguments {
        &iter.args
    } else {
        return None;
    };
    let mut res = vec![];
    let mut cur_ty: Option<&'a syn::Type> = None;
    for x in inner.iter() {
        match x {
            syn::GenericArgument::Type(ty) => {
                if let Some(ty) = cur_ty {
                    res.push(TypePunct { ty, punct: None })
                }
                cur_ty = Some(ty);
            }
            syn::GenericArgument::Const(expr) => match expr {
                syn::Expr::Lit(syn::ExprLit {
                    attrs: _,
                    lit: syn::Lit::Str(punct),
                }) => {
                    res.push(TypePunct {
                        ty: cur_ty.unwrap(),
                        punct: Some(punct),
                    });
                }
                _ => return None,
            },
            _ => return None,
        }
    }
    if let Some(cur) = cur_ty {
        res.push(TypePunct {
            ty: cur,
            punct: None,
        })
    }
    Some(res)
}

#[derive(Debug)]
struct TypePunct<'a> {
    ty: &'a syn::Type,
    punct: Option<&'a syn::LitStr>,
}
