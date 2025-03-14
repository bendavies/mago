use mago_ast::ast::*;
use mago_ast::sequence::Sequence;
use mago_token::T;

use crate::error::ParseError;
use crate::internal::block::parse_block;
use crate::internal::function_like::parameter::parse_function_like_parameter_list;
use crate::internal::function_like::r#return::parse_optional_function_like_return_type_hint;
use crate::internal::identifier::parse_local_identifier;
use crate::internal::token_stream::TokenStream;
use crate::internal::utils;

pub fn parse_method_with_attributes_and_modifiers(
    stream: &mut TokenStream<'_, '_>,
    attributes: Sequence<AttributeList>,
    modifiers: Sequence<Modifier>,
) -> Result<Method, ParseError> {
    Ok(Method {
        attribute_lists: attributes,
        modifiers,
        function: utils::expect_keyword(stream, T!["function"])?,
        ampersand: utils::maybe_expect(stream, T!["&"])?.map(|t| t.span),
        name: parse_local_identifier(stream)?,
        parameter_list: parse_function_like_parameter_list(stream)?,
        return_type_hint: parse_optional_function_like_return_type_hint(stream)?,
        body: parse_method_body(stream)?,
    })
}

pub fn parse_method_body(stream: &mut TokenStream<'_, '_>) -> Result<MethodBody, ParseError> {
    let next = utils::maybe_peek(stream)?;
    Ok(match next.map(|t| t.kind) {
        Some(T![";"]) => MethodBody::Abstract(MethodAbstractBody { semicolon: utils::expect_any(stream)?.span }),
        _ => MethodBody::Concrete(parse_block(stream)?),
    })
}
