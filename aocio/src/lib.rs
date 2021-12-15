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
        syn::FnArg::Receiver(_) => panic!(""),
        syn::FnArg::Typed(x) => (&x.pat, &x.ty),
    };

    let aoc_ty = parse_ty(ty);

    // print!("!!! aoc_ty {:#?}", &aoc_ty);

    let wrapper_name = syn::Ident::new(&format!("__Wrap_{}", name), name.span());

    let def = aoc_ty.definition(Some(&wrapper_name));

    // println!("!!! def = {}", &def);

    let output = input.sig.output;
    let block = input.block;
    let q = quote! {
        fn #name(#wrapper_name(#pat): #wrapper_name) #output #block
        #def
    };
    q.into()
}

#[derive(Debug)]
enum AOCType<'a> {
    Vec(TypePunct<'a>),
    Tuple(Vec<TypePunct<'a>>),
    Other(&'a syn::Type),
}

#[derive(Debug)]
struct TypePunct<'a> {
    ty: Box<AOCType<'a>>,
    punct: String,
}

impl<'a> AOCType<'a> {
    fn real_type(&self) -> proc_macro2::TokenStream {
        match self {
            AOCType::Vec(tp) => {
                let inner = tp.ty.real_type();
                quote!(
                    Vec<#inner>
                )
            }
            AOCType::Tuple(tps) => {
                let inners: Vec<_> = tps.into_iter().map(|tp| tp.ty.real_type()).collect();
                quote!(
                    (#(#inners),*)
                )
            }
            AOCType::Other(ty) => {
                quote!(#ty)
            }
        }
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
                    .collect::<Vec<_>>()
                )
            }
            AOCType::Tuple(tps) => {
                let n = tps.len();
                let names: Vec<_> = (0..n).map(|i| format!("v{}", i)).collect();
                let idents: Vec<_> = names
                    .iter()
                    .map(|x| syn::Ident::new(x, Span::call_site()))
                    .collect();
                let inners: Vec<_> = tps.iter().map(|tp| tp.ty.definition(None)).collect();
                let puncts: Vec<_> = tps.iter().map(|tp| &tp.punct).collect();

                let mut quotes = vec![];
                for i in 0..(n - 1) {
                    let ident = &idents[i];
                    let punct = &puncts[i];
                    let inner = &inners[i];
                    let q = quote!(
                        let ss = s.trim().split_once(#punct).unwrap();
                        let #ident = {
                            let s = ss.0;
                            #inner
                        };
                        let s = ss.1;
                    );
                    quotes.push(q);
                }

                let ident = &idents[n - 1];
                let inner = &inners[n - 1];
                quote!({
                    #(#quotes)*
                    let #ident = #inner;
                    (#(#idents),*)
                })
            }
            AOCType::Other(_) => {
                quote!(s.trim().parse().unwrap())
            }
        }
    }
}

fn parse_ty<'a>(ty: &'a syn::Type) -> AOCType<'a> {
    if let Some(vs) = try_inner("Vec", ty) {
        if vs.len() == 1 {
            return AOCType::Vec(TypePunct {
                ty: Box::new(parse_ty(vs[0].0)),
                punct: vs[0].1.clone(),
            });
        } else {
            panic!("Vec bad format {}", vs.len())
        }
    } else if let Some(vs) = try_inner("Tuple", ty) {
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

fn try_inner<'a>(wrapper: &str, ty: &'a syn::Type) -> Option<Vec<(&'a syn::Type, String)>> {
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
                    res.push((ty, "\n".to_owned()))
                }
                cur_ty = Some(ty);
            }
            syn::GenericArgument::Const(expr) => match expr {
                syn::Expr::Lit(syn::ExprLit {
                    attrs: _,
                    lit: syn::Lit::Str(punct),
                }) => {
                    res.push((cur_ty.unwrap(), punct.value()));
                    cur_ty = None;
                }
                _ => return None,
            },
            _ => return None,
        }
    }
    if let Some(cur) = cur_ty {
        res.push((cur, "\n".to_owned()));
    }
    Some(res)
}
