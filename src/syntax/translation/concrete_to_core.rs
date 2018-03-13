use codespan::ByteSpan;
use nameless::{FreshState, Named, Scope, Var};

use syntax::concrete;
use syntax::core;

/// Translate something to the corresponding core representation
pub trait ToCore<T> {
    fn to_core(&self, fresh: &mut FreshState) -> T;
}

/// Convert a sugary pi type from something like:
///
/// ```text
/// (a b : t1) -> t3
/// ```
///
/// To a bunch of nested pi types like:
///
/// ```text
/// (a : t1) -> (b : t1) -> t3
/// ```
fn pi_to_core(
    fresh: &mut FreshState,
    param_names: &[(ByteSpan, String)],
    ann: &concrete::Term,
    body: &concrete::Term,
) -> core::RcRawTerm {
    let ann = ann.to_core(fresh);
    let mut term = body.to_core(fresh);

    for &(span, ref name) in param_names.iter().rev() {
        // This could be wrong... :/
        term = core::RawTerm::Pi(
            core::SourceMeta {
                span: span.to(term.span()),
            },
            Scope::bind(
                Named::new(core::Name::user(name.clone()), ann.clone()),
                term,
            ),
        ).into();
    }

    term
}

/// Convert a sugary lambda from something like:
///
/// ```text
/// \(a b : t1) c (d : t2) => t3
/// ```
///
/// To a bunch of nested lambdas like:
///
/// ```text
/// \(a : t1) => \(b : t1) => \c => \(d : t2) => t3
/// ```
fn lam_to_core(
    fresh: &mut FreshState,
    params: &[(Vec<(ByteSpan, String)>, Option<Box<concrete::Term>>)],
    body: &concrete::Term,
) -> core::RcRawTerm {
    let mut term = body.to_core(fresh);

    for &(ref names, ref ann) in params.iter().rev() {
        for &(span, ref name) in names.iter().rev() {
            let name = core::Name::user(name.clone());
            let meta = core::SourceMeta {
                span: span.to(term.span()),
            };
            let ann = match *ann {
                None => core::RawTerm::Hole(core::SourceMeta::default()).into(),
                Some(ref ann) => ann.to_core(fresh),
            };
            term = core::RawTerm::Lam(meta, Scope::bind(Named::new(name, ann), term)).into();
        }
    }

    term
}

impl ToCore<core::RawModule> for concrete::Module {
    /// Convert the module in the concrete syntax to a module in the core syntax
    fn to_core(&self, fresh: &mut FreshState) -> core::RawModule {
        match *self {
            concrete::Module::Valid {
                ref name,
                ref declarations,
            } => {
                // The type claims that we have encountered so far! We'll use these when
                // we encounter their corresponding definitions later as type annotations
                let mut prev_claim = None;
                // The definitions, desugared from the concrete syntax
                let mut definitions = Vec::<core::RawDefinition>::new();

                for declaration in declarations {
                    match *declaration {
                        concrete::Declaration::Import { .. } => {
                            unimplemented!("import declarations")
                        },
                        concrete::Declaration::Claim {
                            name: (_, ref name),
                            ref ann,
                            ..
                        } => match prev_claim.take() {
                            Some((name, ann)) => {
                                let term = core::RawTerm::Hole(core::SourceMeta::default()).into();
                                definitions.push(core::RawDefinition { name, term, ann });
                            },
                            None => prev_claim = Some((name.clone(), ann.to_core(fresh))),
                        },
                        concrete::Declaration::Definition {
                            name: (_, ref name),
                            ref params,
                            ref body,
                            ..
                        } => {
                            let default_meta = core::SourceMeta::default();

                            match prev_claim.take() {
                                None => definitions.push(core::RawDefinition {
                                    name: name.clone(),
                                    ann: core::RawTerm::Hole(default_meta).into(),
                                    term: lam_to_core(fresh, params, body),
                                }),
                                Some((claim_name, ann)) => {
                                    if claim_name == *name {
                                        definitions.push(core::RawDefinition {
                                            name: name.clone(),
                                            ann,
                                            term: lam_to_core(fresh, params, body),
                                        });
                                    } else {
                                        definitions.push(core::RawDefinition {
                                            name: claim_name.clone(),
                                            ann,
                                            term: core::RawTerm::Hole(default_meta).into(),
                                        });
                                        definitions.push(core::RawDefinition {
                                            name: name.clone(),
                                            ann: core::RawTerm::Hole(default_meta).into(),
                                            term: lam_to_core(fresh, params, body),
                                        });
                                    }
                                },
                            };
                        },
                        concrete::Declaration::Error(_) => unimplemented!("error recovery"),
                    }
                }

                core::RawModule {
                    name: name.1.clone(),
                    definitions,
                }
            },
            concrete::Module::Error(_) => unimplemented!("error recovery"),
        }
    }
}

impl ToCore<core::RcRawTerm> for concrete::Term {
    /// Convert a term in the concrete syntax into a core term
    fn to_core(&self, fresh: &mut FreshState) -> core::RcRawTerm {
        let meta = core::SourceMeta { span: self.span() };
        match *self {
            concrete::Term::Parens(_, ref term) => term.to_core(fresh),
            concrete::Term::Ann(ref expr, ref ty) => {
                let expr = expr.to_core(fresh).into();
                let ty = ty.to_core(fresh).into();

                core::RawTerm::Ann(meta, expr, ty).into()
            },
            concrete::Term::Universe(_, level) => {
                core::RawTerm::Universe(meta, core::Level(level.unwrap_or(0))).into()
            },
            concrete::Term::Hole(_) => core::RawTerm::Hole(meta).into(),
            concrete::Term::Var(_, ref x) => {
                let var = Var::Free(core::Name::user(x.clone()));

                core::RawTerm::Var(meta, var).into()
            },
            concrete::Term::Pi(_, (ref names, ref ann), ref body) => {
                pi_to_core(fresh, names, ann, body)
            },
            concrete::Term::Lam(_, ref params, ref body) => lam_to_core(fresh, params, body),
            concrete::Term::Arrow(ref ann, ref body) => {
                let name = core::Name::from(fresh.next_gen());
                let ann = ann.to_core(fresh);
                let body = body.to_core(fresh);

                core::RawTerm::Pi(meta, Scope::bind(Named::new(name, ann), body)).into()
            },
            concrete::Term::App(ref fn_expr, ref arg) => {
                let fn_expr = fn_expr.to_core(fresh);
                let arg = arg.to_core(fresh);

                core::RawTerm::App(meta, fn_expr, arg).into()
            },
            concrete::Term::Error(_) => unimplemented!("error recovery"),
        }
    }
}

#[cfg(test)]
mod to_core {
    use codespan::{CodeMap, FileName};

    use library;
    use syntax::parse;

    use super::*;

    fn parse(fresh: &mut FreshState, src: &str) -> core::RcRawTerm {
        let mut codemap = CodeMap::new();
        let filemap = codemap.add_filemap(FileName::virtual_("test"), src.into());

        let (concrete_term, errors) = parse::term(&filemap);
        assert!(errors.is_empty());

        concrete_term.to_core(fresh)
    }

    mod module {
        use super::*;

        #[test]
        fn parse_prelude() {
            let mut codemap = CodeMap::new();
            let mut fresh = FreshState::new();
            let filemap = codemap.add_filemap(FileName::virtual_("test"), library::PRELUDE.into());

            let (concrete_module, errors) = parse::module(&filemap);
            assert!(errors.is_empty());

            concrete_module.to_core(&mut fresh);
        }
    }

    mod term {
        use super::*;

        use syntax::core::{Level, Name, RawTerm, SourceMeta};

        #[test]
        fn var() {
            let mut fresh = FreshState::new();

            assert_alpha_eq!(
                parse(&mut fresh, r"x"),
                RawTerm::Var(SourceMeta::default(), Var::Free(Name::user("x"))).into()
            );
        }

        #[test]
        fn var_kebab_case() {
            let mut fresh = FreshState::new();

            assert_alpha_eq!(
                parse(&mut fresh, r"or-elim"),
                RawTerm::Var(SourceMeta::default(), Var::Free(Name::user("or-elim"))).into(),
            );
        }

        #[test]
        fn ty() {
            let mut fresh = FreshState::new();

            assert_alpha_eq!(
                parse(&mut fresh, r"Type"),
                RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
            );
        }

        #[test]
        fn ty_level() {
            let mut fresh = FreshState::new();

            assert_alpha_eq!(
                parse(&mut fresh, r"Type 2"),
                RawTerm::Universe(SourceMeta::default(), Level::ZERO.succ().succ()).into()
            );
        }

        #[test]
        fn ann() {
            let mut fresh = FreshState::new();

            assert_alpha_eq!(
                parse(&mut fresh, r"Type : Type"),
                RawTerm::Ann(
                    SourceMeta::default(),
                    RawTerm::Universe(SourceMeta::default(), Level::ZERO).into(),
                    RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                ).into(),
            );
        }

        #[test]
        fn ann_ann_left() {
            let mut fresh = FreshState::new();

            assert_alpha_eq!(
                parse(&mut fresh, r"Type : Type : Type"),
                RawTerm::Ann(
                    SourceMeta::default(),
                    RawTerm::Universe(SourceMeta::default(), Level::ZERO).into(),
                    RawTerm::Ann(
                        SourceMeta::default(),
                        RawTerm::Universe(SourceMeta::default(), Level::ZERO).into(),
                        RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                    ).into(),
                ).into(),
            );
        }

        #[test]
        fn ann_ann_right() {
            let mut fresh = FreshState::new();

            assert_alpha_eq!(
                parse(&mut fresh, r"Type : (Type : Type)"),
                RawTerm::Ann(
                    SourceMeta::default(),
                    RawTerm::Universe(SourceMeta::default(), Level::ZERO).into(),
                    RawTerm::Ann(
                        SourceMeta::default(),
                        RawTerm::Universe(SourceMeta::default(), Level::ZERO).into(),
                        RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                    ).into(),
                ).into(),
            );
        }

        #[test]
        fn ann_ann_ann() {
            let mut fresh = FreshState::new();

            assert_alpha_eq!(
                parse(&mut fresh, r"(Type : Type) : (Type : Type)"),
                RawTerm::Ann(
                    SourceMeta::default(),
                    RawTerm::Ann(
                        SourceMeta::default(),
                        RawTerm::Universe(SourceMeta::default(), Level::ZERO).into(),
                        RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                    ).into(),
                    RawTerm::Ann(
                        SourceMeta::default(),
                        RawTerm::Universe(SourceMeta::default(), Level::ZERO).into(),
                        RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                    ).into(),
                ).into(),
            );
        }

        #[test]
        fn lam_ann() {
            let mut fresh = FreshState::new();

            let x = Name::user("x");

            assert_alpha_eq!(
                parse(&mut fresh, r"\x : Type -> Type => x"),
                RawTerm::Lam(
                    SourceMeta::default(),
                    Scope::bind(
                        Named::new(
                            x.clone(),
                            RawTerm::Pi(
                                SourceMeta::default(),
                                Scope::bind(
                                    Named::new(
                                        Name::user("_"),
                                        RawTerm::Universe(SourceMeta::default(), Level::ZERO)
                                            .into()
                                    ),
                                    RawTerm::Universe(SourceMeta::default(), Level::ZERO).into(),
                                )
                            ).into()
                        ),
                        RawTerm::Var(SourceMeta::default(), Var::Free(x)).into(),
                    )
                ).into(),
            );
        }

        #[test]
        fn lam() {
            let mut fresh = FreshState::new();

            let x = Name::user("x");
            let y = Name::user("y");

            assert_alpha_eq!(
                parse(&mut fresh, r"\x : (\y => y) => x"),
                RawTerm::Lam(
                    SourceMeta::default(),
                    Scope::bind(
                        Named::new(
                            x.clone(),
                            RawTerm::Lam(
                                SourceMeta::default(),
                                Scope::bind(
                                    Named::new(
                                        y.clone(),
                                        RawTerm::Hole(SourceMeta::default()).into()
                                    ),
                                    RawTerm::Var(SourceMeta::default(), Var::Free(y)).into(),
                                )
                            ).into(),
                        ),
                        RawTerm::Var(SourceMeta::default(), Var::Free(x)).into(),
                    )
                ).into(),
            );
        }

        #[test]
        fn lam_lam_ann() {
            let mut fresh = FreshState::new();

            let x = Name::user("x");
            let y = Name::user("y");

            assert_alpha_eq!(
                parse(&mut fresh, r"\(x y : Type) => x"),
                RawTerm::Lam(
                    SourceMeta::default(),
                    Scope::bind(
                        Named::new(
                            x.clone(),
                            RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                        ),
                        RawTerm::Lam(
                            SourceMeta::default(),
                            Scope::bind(
                                Named::new(
                                    y,
                                    RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                                ),
                                RawTerm::Var(SourceMeta::default(), Var::Free(x)).into(),
                            )
                        ).into(),
                    )
                ).into(),
            );
        }

        #[test]
        fn arrow() {
            let mut fresh = FreshState::new();

            assert_alpha_eq!(
                parse(&mut fresh, r"Type -> Type"),
                RawTerm::Pi(
                    SourceMeta::default(),
                    Scope::bind(
                        Named::new(
                            Name::user("_"),
                            RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                        ),
                        RawTerm::Universe(SourceMeta::default(), Level::ZERO).into(),
                    )
                ).into(),
            );
        }

        #[test]
        fn pi() {
            let mut fresh = FreshState::new();

            let x = Name::user("x");

            assert_alpha_eq!(
                parse(&mut fresh, r"(x : Type -> Type) -> x"),
                RawTerm::Pi(
                    SourceMeta::default(),
                    Scope::bind(
                        Named::new(
                            x.clone(),
                            RawTerm::Pi(
                                SourceMeta::default(),
                                Scope::bind(
                                    Named::new(
                                        Name::user("_"),
                                        RawTerm::Universe(SourceMeta::default(), Level::ZERO)
                                            .into()
                                    ),
                                    RawTerm::Universe(SourceMeta::default(), Level::ZERO).into(),
                                )
                            ).into(),
                        ),
                        RawTerm::Var(SourceMeta::default(), Var::Free(x)).into(),
                    )
                ).into(),
            );
        }

        #[test]
        fn pi_pi() {
            let mut fresh = FreshState::new();

            let x = Name::user("x");
            let y = Name::user("y");

            assert_alpha_eq!(
                parse(&mut fresh, r"(x y : Type) -> x"),
                RawTerm::Pi(
                    SourceMeta::default(),
                    Scope::bind(
                        Named::new(
                            x.clone(),
                            RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                        ),
                        RawTerm::Pi(
                            SourceMeta::default(),
                            Scope::bind(
                                Named::new(
                                    y,
                                    RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                                ),
                                RawTerm::Var(SourceMeta::default(), Var::Free(x)).into(),
                            )
                        ).into(),
                    )
                ).into(),
            );
        }

        #[test]
        fn pi_arrow() {
            let mut fresh = FreshState::new();

            let x = Name::user("x");

            assert_alpha_eq!(
                parse(&mut fresh, r"(x : Type) -> x -> x"),
                RawTerm::Pi(
                    SourceMeta::default(),
                    Scope::bind(
                        Named::new(
                            x.clone(),
                            RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                        ),
                        RawTerm::Pi(
                            SourceMeta::default(),
                            Scope::bind(
                                Named::new(
                                    Name::user("_"),
                                    RawTerm::Var(SourceMeta::default(), Var::Free(x.clone()))
                                        .into()
                                ),
                                RawTerm::Var(SourceMeta::default(), Var::Free(x)).into(),
                            )
                        ).into(),
                    )
                ).into(),
            );
        }

        #[test]
        fn lam_app() {
            let mut fresh = FreshState::new();

            let x = Name::user("x");
            let y = Name::user("y");

            assert_alpha_eq!(
                parse(&mut fresh, r"\(x : Type -> Type) (y : Type) => x y"),
                RawTerm::Lam(
                    SourceMeta::default(),
                    Scope::bind(
                        Named::new(
                            x.clone(),
                            RawTerm::Pi(
                                SourceMeta::default(),
                                Scope::bind(
                                    Named::new(
                                        Name::user("_"),
                                        RawTerm::Universe(SourceMeta::default(), Level::ZERO)
                                            .into()
                                    ),
                                    RawTerm::Universe(SourceMeta::default(), Level::ZERO).into(),
                                )
                            ).into(),
                        ),
                        RawTerm::Lam(
                            SourceMeta::default(),
                            Scope::bind(
                                Named::new(
                                    y.clone(),
                                    RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                                ),
                                RawTerm::App(
                                    SourceMeta::default(),
                                    RawTerm::Var(SourceMeta::default(), Var::Free(x)).into(),
                                    RawTerm::Var(SourceMeta::default(), Var::Free(y)).into(),
                                ).into(),
                            )
                        ).into(),
                    )
                ).into(),
            );
        }

        #[test]
        fn id() {
            let mut fresh = FreshState::new();

            let x = Name::user("x");
            let a = Name::user("a");

            assert_alpha_eq!(
                parse(&mut fresh, r"\(a : Type) (x : a) => x"),
                RawTerm::Lam(
                    SourceMeta::default(),
                    Scope::bind(
                        Named::new(
                            a.clone(),
                            RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                        ),
                        RawTerm::Lam(
                            SourceMeta::default(),
                            Scope::bind(
                                Named::new(
                                    x.clone(),
                                    RawTerm::Var(SourceMeta::default(), Var::Free(a)).into()
                                ),
                                RawTerm::Var(SourceMeta::default(), Var::Free(x)).into(),
                            )
                        ).into(),
                    )
                ).into(),
            );
        }

        #[test]
        fn id_ty() {
            let mut fresh = FreshState::new();

            let a = Name::user("a");

            assert_alpha_eq!(
                parse(&mut fresh, r"(a : Type) -> a -> a"),
                RawTerm::Pi(
                    SourceMeta::default(),
                    Scope::bind(
                        Named::new(
                            a.clone(),
                            RawTerm::Universe(SourceMeta::default(), Level::ZERO).into()
                        ),
                        RawTerm::Pi(
                            SourceMeta::default(),
                            Scope::bind(
                                Named::new(
                                    Name::user("_"),
                                    RawTerm::Var(SourceMeta::default(), Var::Free(a.clone()))
                                        .into()
                                ),
                                RawTerm::Var(SourceMeta::default(), Var::Free(a)).into(),
                            )
                        ).into(),
                    )
                ).into(),
            );
        }

        mod sugar {
            use super::*;

            #[test]
            fn lam_args() {
                let mut fresh = FreshState::new();

                assert_alpha_eq!(
                    parse(&mut fresh, r"\x (y : Type) z => x"),
                    parse(&mut fresh, r"\x => \y : Type => \z => x"),
                );
            }

            #[test]
            fn lam_args_multi() {
                let mut fresh = FreshState::new();

                assert_alpha_eq!(
                    parse(&mut fresh, r"\(x : Type) (y : Type) z => x"),
                    parse(&mut fresh, r"\(x y : Type) z => x"),
                );
            }

            #[test]
            fn pi_args() {
                let mut fresh = FreshState::new();

                assert_alpha_eq!(
                    parse(&mut fresh, r"(a : Type) -> (x y z : a) -> x"),
                    parse(
                        &mut fresh,
                        r"(a : Type) -> (x : a) -> (y : a) -> (z : a) -> x"
                    ),
                );
            }

            #[test]
            fn arrow() {
                let mut fresh = FreshState::new();

                assert_alpha_eq!(
                    parse(&mut fresh, r"(a : Type) -> a -> a"),
                    parse(&mut fresh, r"(a : Type) -> (x : a) -> a"),
                )
            }
        }
    }
}
