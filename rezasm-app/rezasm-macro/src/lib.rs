#![allow(dead_code)]
#![allow(unused_variables)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use std::iter::{repeat, zip, Repeat};
use std::ops::Deref;
use syn::{parse_macro_input, Expr, ExprClosure, Pat, PatIdent, ReturnType, Type, TypePath};

macro_rules! error {
    ($text:expr) => {
        return TokenStream::from(quote! {
            compile_error!($text)
        })
    };
}

#[proc_macro]
pub fn instruction(_input: TokenStream) -> TokenStream {
    let mut cloned_input = _input.clone().into_iter();

    if cloned_input.size_hint().0 < 3 {
        error!("Not enough arguments, expected 2: `instruction! (name, |arguments| {...})`");
    }

    let first: TokenStream = TokenStream::from(cloned_input.nth(0).unwrap());

    let closure_stream: TokenStream = cloned_input
        .enumerate()
        .filter(|index| index.0 != 0)
        .map(|x| x.1)
        .collect();

    let function_name: Ident = parse_macro_input!(first as syn::Ident);
    let function_closure: ExprClosure = parse_macro_input!(closure_stream as syn::ExprClosure);
    let parsed_name: proc_macro2::TokenStream = function_name.to_token_stream();

    let mut simulator_name: proc_macro2::TokenStream = "".to_token_stream();
    let mut argument_types: Vec<TypePath> = Vec::new();
    let mut argument_names: Vec<PatIdent> = Vec::new();

    if function_closure.inputs.iter().size_hint().0 == 0 {
        error!("Expected Simulator variable as the first argument in closure capture group");
    }

    for (index, argument) in function_closure.inputs.iter().enumerate() {
        match argument {
            Pat::Type(x) => match x.ty.deref() {
                Type::Path(type_path) => {
                    if index == 0 {
                        if match type_path.path.get_ident() {
                            None => error!(
                                "Expected Simulator type identity, not a path to a type identity"
                            ),
                            Some(x) => x.to_string(),
                        } != "Simulator"
                        {
                            error!("Expected Simulator variable as the first argument in closure capture group");
                        }
                        match x.pat.deref() {
                            Pat::Ident(name_identity) => {
                                simulator_name = name_identity.to_token_stream();
                            }
                            _ => error!("Expected valid variable name"),
                        }
                    } else {
                        match type_path.path.get_ident() {
                            None => {
                                error!("Expected a type identity, not a path to a type identity")
                            }
                            Some(i) => {
                                if i.to_string() == "InputTarget".to_string()
                                    || i.to_string() == "InputOutputTarget".to_string()
                                {
                                    argument_types.push(type_path.clone());
                                    match x.pat.deref() {
                                        Pat::Ident(name_identity) => {
                                            argument_names.push(name_identity.clone())
                                        }
                                        _ => error!("Expected valid variable name"),
                                    }
                                } else {
                                    error!("Given types must be InputTarget or InputOutputTarget");
                                }
                            }
                        }
                    }
                }
                _ => error!("Given token should be a type"),
            },
            _ => error!("Given token should be a variable"),
        }
    }

    let mut argument_subtypes: Vec<Vec<TypePath>> = Vec::new();
    for argument in argument_types.iter() {
        let argument_string = match argument.path.get_ident() {
            None => error!("Invalid argument type"),
            Some(x) => x.to_string(),
        };
        if argument_string == "InputOutputTarget" {
            argument_subtypes.push(vec![argument.clone()]);
        } else if argument_string == "InputTarget" {
            let input_target_type_path = "InputOutputTarget".to_string().parse().unwrap();
            argument_subtypes.push(vec![
                argument.clone(),
                parse_macro_input!(input_target_type_path as TypePath),
            ]);
        } else {
            error!("Invalid argument type");
        }
    }

    let mut possible_output_permutations: Vec<Vec<TypePath>> = vec![Vec::new()];
    for type_element_list in argument_subtypes.iter() {
        let initial_lists_state = possible_output_permutations.clone();
        for (type_element_index, type_element) in type_element_list.iter().enumerate() {
            if type_element_index > 0 {
                // Append a copy of the initial state, each with the next type appended to them
                for list in initial_lists_state.iter() {
                    let mut list = list.clone();
                    list.push(type_element.clone());
                    possible_output_permutations.push(list);
                }
            } else {
                // Only happens for the first element
                for list in possible_output_permutations.iter_mut() {
                    list.push(type_element.clone());
                }
            }
        }
    }

    let output_type: ReturnType = match function_closure.output {
        ReturnType::Default => ReturnType::Default,
        ReturnType::Type(_, _) => error!("Return type does not need to be specified"),
    };

    let function_body = match function_closure.body.deref() {
        Expr::Block(x) => x.clone().block,
        _ => error!("Capture group must be followed by code block"),
    };

    let mut function_declarations: Vec<proc_macro2::TokenStream> = Vec::new();
    let function_name_repeat: Repeat<proc_macro2::TokenStream> = repeat(parsed_name.clone());
    let argument_names_repeat: Repeat<Vec<PatIdent>> = repeat(argument_names.clone());

    for (permutation, field) in zip(possible_output_permutations.clone(), argument_names_repeat) {
        function_declarations.push(proc_macro2::TokenStream::from(quote! {
            fn #parsed_name (#simulator_name: &mut rezasm_core::simulation::simulator::Simulator, types: &Vec<std::any::TypeId>, arguments: &Vec<rezasm_core::instructions::argument_type::ArgumentType>) -> Result<(), rezasm_core::util::error::SimulatorError> {
                let mut _counter: usize = 0;
                #(let mut #field: #permutation = arguments[_counter].downcast::<#permutation>().unwrap().clone(); _counter += 1;)*
                #function_body
            }
        }));
    }

    let tokens = quote! {
        {
            use rezasm_core::instructions::argument_type::Downcast;
            use rezasm_core::instructions::targets::input_target::{Input, InputTarget};
            use rezasm_core::instructions::targets::input_output_target::{InputOutput, InputOutputTarget};
            use rezasm_core::instructions::targets::output_target::Output;
            use rezasm_core::util::raw_data::RawData;
            let mut instruction_field_types: Vec<std::any::TypeId> = vec![#(std::any::TypeId::of::<#argument_types>(),)*];
            let mut instructions: Vec<rezasm_core::instructions::instruction::Instruction> = Vec::new();
            #({
                #function_declarations;
                instructions.push(rezasm_core::instructions::instruction::Instruction::new(vec![#(std::any::TypeId::of::<#possible_output_permutations>(),)*], #function_name_repeat));
            })*
            rezasm_core::instructions::instruction_field::InstructionField::new(instruction_field_types, instructions, stringify!(#parsed_name).to_string())
        }
    };

    TokenStream::from(tokens)
}
