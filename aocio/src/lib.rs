#![allow(clippy::needless_lifetimes)]
extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn aocio(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;

    let (pat, ty) = match &input.sig.inputs[0] {
        syn::FnArg::Receiver(_) => panic!("Receiver type not supported"),
        syn::FnArg::Typed(x) => (&x.pat, &x.ty),
    };

    let aoc_ty = parse_ty(ty);
    let wrapper_name = syn::Ident::new(&format!("__Wrap_{}", name), name.span());

    let def = aoc_ty.definition(Some(&wrapper_name));

    let output = input.sig.output;
    let block = input.block;

    let vis = input.vis;
    let q = quote! {
        #vis fn #name(#wrapper_name(#pat): #wrapper_name) #output #block
        #def
    };
    q.into()
}

#[derive(Debug)]
enum AOCType<'a> {
    Vec(TypePunct<'a>),
    Tuple(Vec<TypePunct<'a>>),
    Other(&'a syn::Type),
    DontCare,
}

#[derive(Debug)]
struct TypePunct<'a> {
    ty: Box<AOCType<'a>>,
    punct: String,
}

impl<'a> TypePunct<'a> {
    fn dont_care(&'a self) -> bool {
        if let AOCType::DontCare = self.ty.as_ref() {
            return true;
        }
        false
    }
}

impl<'a> AOCType<'a> {
    fn real_type(&self) -> Option<proc_macro2::TokenStream> {
        Some(match self {
            AOCType::Vec(tp) => {
                let inner = tp.ty.real_type().unwrap();
                quote!(
                    Vec<#inner>
                )
            }
            AOCType::Tuple(tps) => {
                let inners: Vec<_> = tps.iter().filter_map(|tp| tp.ty.real_type()).collect();
                quote!(
                    (#(#inners),*)
                )
            }
            AOCType::Other(ty) => quote!(#ty),
            AOCType::DontCare => return None,
        })
    }

    fn definition(&self, wrapper: Option<&Ident>) -> proc_macro2::TokenStream {
        if let Some(wrapper) = wrapper {
            let ty = self.real_type();
            let inner = self.definition(None);
            return quote!(
                #[allow(non_camel_case_types)]
                pub struct #wrapper(#ty);
                impl std::str::FromStr for #wrapper {
                    type Err = String;

                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        Ok(#wrapper(
                            #inner
                        ))
                    }
                }
            );
        }
        match self {
            AOCType::Vec(TypePunct { ty, punct }) => {
                let inner = ty.definition(None);
                quote!(
                    s
                    .trim()
                    .split(#punct)
                    .filter(|x|!x.trim().is_empty())
                    .map(|s|#inner)
                    .collect()
                )
            }
            AOCType::Tuple(tps) => {
                let n = tps.len();
                let puncts: Vec<_> = tps.iter().map(|tp| &tp.punct).collect();

                struct Item {
                    inner: proc_macro2::TokenStream,
                    ident: Ident,
                }
                let items: Vec<Option<Item>> = (0..n)
                    .map(|i| {
                        if tps[i].dont_care() {
                            None
                        } else {
                            let name = format!("v{}", i);
                            let inner = tps[i].ty.definition(None);
                            let ident = Ident::new(&name, Span::call_site());
                            Some(Item { inner, ident })
                        }
                    })
                    .collect();

                let mut quotes = vec![];
                for i in 0..(n - 1) {
                    let q = items[i].as_ref().map(|item| {
                        let ident = &item.ident;
                        let inner = &item.inner;
                        quote!(
                            let #ident = {
                                let s = ss.0;
                                #inner
                            };
                        )
                    });
                    let punct = &puncts[i];
                    let q2 = quote!(
                        let ss = s.trim().split_once(#punct).unwrap();
                        #q
                        let s = ss.1;
                    );
                    quotes.push(q2);
                }

                let q = items[n - 1].as_ref().map(|i| {
                    let ident = &i.ident;
                    let inner = &i.inner;
                    quote!(
                        let #ident = #inner;
                    )
                });
                let idents: Vec<_> = items
                    .into_iter()
                    .filter_map(|oi| oi.map(|x| x.ident))
                    .collect();
                quote!({
                    #(#quotes)*
                    #q
                    (#(#idents),*)
                })
            }
            AOCType::Other(_) => {
                quote!(s.trim().parse().unwrap())
            }
            AOCType::DontCare => {
                panic!("_ is usable only in Tuple");
            }
        }
    }
}

fn parse_ty<'a>(ty: &'a syn::Type) -> AOCType<'a> {
    if let syn::Type::Infer(_) = ty {
        return AOCType::DontCare;
    }
    if let Some(vs) = try_inner("Vec", ty, "\n") {
        if vs.len() == 1 {
            return AOCType::Vec(TypePunct {
                ty: Box::new(parse_ty(vs[0].0)),
                punct: vs[0].1.clone(),
            });
        } else {
            panic!("Vec bad format {}", vs.len())
        }
    } else if let Some(vs) = try_inner("Tuple", ty, "\n\n") {
        return AOCType::Tuple(
            vs.into_iter()
                .map(|(ty, punct)| TypePunct {
                    ty: Box::new(parse_ty(ty)),
                    punct,
                })
                .collect(),
        );
    }
    AOCType::Other(ty)
}

fn try_inner<'a>(
    wrapper: &str,
    ty: &'a syn::Type,
    default_sep: &str,
) -> Option<Vec<(&'a syn::Type, String)>> {
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
                    res.push((ty, default_sep.to_owned()))
                }
                cur_ty = Some(ty);
            }
            syn::GenericArgument::Const(syn::Expr::Lit(syn::ExprLit {
                attrs: _,
                lit: syn::Lit::Str(punct),
            })) => {
                res.push((cur_ty.unwrap(), punct.value()));
                cur_ty = None;
            }
            _ => return None,
        }
    }
    if let Some(cur) = cur_ty {
        res.push((cur, default_sep.to_owned()));
    }
    Some(res)
}
