use mago_ast::*;
use mago_ast_utils::reference::*;

use crate::context::LintContext;

pub const TESTING_METHODS: [&str; 57] = [
    "anything",
    "arrayHasKey",
    "attribute",
    "attributeEqualTo",
    "callback",
    "classHasAttribute",
    "classHasStaticAttribute",
    "contains",
    "containsEqual",
    "containsIdentical",
    "containsOnly",
    "containsOnlyInstancesOf",
    "countOf",
    "directoryExists",
    "equalTo",
    "equalToCanonicalizing",
    "equalToIgnoringCase",
    "equalToWithDelta",
    "fail",
    "fileExists",
    "getCount",
    "getObjectAttribute",
    "getStaticAttribute",
    "greaterThan",
    "greaterThanOrEqual",
    "identicalTo",
    "isEmpty",
    "isFalse",
    "isFinite",
    "isInfinite",
    "isInstanceOf",
    "isJson",
    "isList",
    "isNan",
    "isNull",
    "isReadable",
    "isTrue",
    "isType",
    "isWritable",
    "lessThan",
    "lessThanOrEqual",
    "logicalAnd",
    "logicalNot",
    "logicalOr",
    "logicalXor",
    "markTestIncomplete",
    "markTestSkipped",
    "matches",
    "matchesRegularExpression",
    "objectEquals",
    "objectHasAttribute",
    "readAttribute",
    "resetCount",
    "stringContains",
    "stringEndsWith",
    "stringEqualsStringIgnoringLineEndings",
    "stringStartsWith",
];

pub const ASSERTION_METHODS: [&str; 173] = [
    "assertArrayHasKey",
    "assertArrayIsEqualToArrayIgnoringListOfKeys",
    "assertArrayIsEqualToArrayOnlyConsideringListOfKeys",
    "assertArrayIsIdenticalToArrayIgnoringListOfKeys",
    "assertArrayIsIdenticalToArrayOnlyConsideringListOfKeys",
    "assertArrayNotHasKey",
    "assertArraySubset",
    "assertAttributeContains",
    "assertAttributeContainsOnly",
    "assertAttributeCount",
    "assertAttributeEmpty",
    "assertAttributeEquals",
    "assertAttributeGreaterThan",
    "assertAttributeGreaterThanOrEqual",
    "assertAttributeInstanceOf",
    "assertAttributeInternalType",
    "assertAttributeLessThan",
    "assertAttributeLessThanOrEqual",
    "assertAttributeNotContains",
    "assertAttributeNotContainsOnly",
    "assertAttributeNotCount",
    "assertAttributeNotEmpty",
    "assertAttributeNotEquals",
    "assertAttributeNotInstanceOf",
    "assertAttributeNotInternalType",
    "assertAttributeNotSame",
    "assertAttributeSame",
    "assertClassHasAttribute",
    "assertClassHasStaticAttribute",
    "assertClassNotHasAttribute",
    "assertClassNotHasStaticAttribute",
    "assertContains",
    "assertContainsEquals",
    "assertContainsOnly",
    "assertContainsOnlyInstancesOf",
    "assertCount",
    "assertDirectoryDoesNotExist",
    "assertDirectoryExists",
    "assertDirectoryIsNotReadable",
    "assertDirectoryIsNotWritable",
    "assertDirectoryIsReadable",
    "assertDirectoryIsWritable",
    "assertDirectoryNotExists",
    "assertDirectoryNotIsReadable",
    "assertDirectoryNotIsWritable",
    "assertDoesNotMatchRegularExpression",
    "assertEmpty",
    "assertEquals",
    "assertEqualsCanonicalizing",
    "assertEqualsIgnoringCase",
    "assertEqualsWithDelta",
    "assertEqualXMLStructure",
    "assertFalse",
    "assertFileDoesNotExist",
    "assertFileEquals",
    "assertFileEqualsCanonicalizing",
    "assertFileEqualsIgnoringCase",
    "assertFileExists",
    "assertFileIsNotReadable",
    "assertFileIsNotWritable",
    "assertFileIsReadable",
    "assertFileIsWritable",
    "assertFileMatchesFormat",
    "assertFileMatchesFormatFile",
    "assertFileNotEquals",
    "assertFileNotEqualsCanonicalizing",
    "assertFileNotEqualsIgnoringCase",
    "assertFileNotExists",
    "assertFileNotIsReadable",
    "assertFileNotIsWritable",
    "assertFinite",
    "assertGreaterThan",
    "assertGreaterThanOrEqual",
    "assertInfinite",
    "assertInstanceOf",
    "assertInternalType",
    "assertIsArray",
    "assertIsBool",
    "assertIsCallable",
    "assertIsClosedResource",
    "assertIsFloat",
    "assertIsInt",
    "assertIsIterable",
    "assertIsList",
    "assertIsNotArray",
    "assertIsNotBool",
    "assertIsNotCallable",
    "assertIsNotClosedResource",
    "assertIsNotFloat",
    "assertIsNotInt",
    "assertIsNotIterable",
    "assertIsNotNumeric",
    "assertIsNotObject",
    "assertIsNotReadable",
    "assertIsNotResource",
    "assertIsNotScalar",
    "assertIsNotString",
    "assertIsNotWritable",
    "assertIsNumeric",
    "assertIsObject",
    "assertIsReadable",
    "assertIsResource",
    "assertIsScalar",
    "assertIsString",
    "assertIsWritable",
    "assertJson",
    "assertJsonFileEqualsJsonFile",
    "assertJsonFileNotEqualsJsonFile",
    "assertJsonStringEqualsJsonFile",
    "assertJsonStringEqualsJsonString",
    "assertJsonStringNotEqualsJsonFile",
    "assertJsonStringNotEqualsJsonString",
    "assertLessThan",
    "assertLessThanOrEqual",
    "assertMatchesRegularExpression",
    "assertNan",
    "assertNotContains",
    "assertNotContainsEquals",
    "assertNotContainsOnly",
    "assertNotCount",
    "assertNotEmpty",
    "assertNotEquals",
    "assertNotEqualsCanonicalizing",
    "assertNotEqualsIgnoringCase",
    "assertNotEqualsWithDelta",
    "assertNotFalse",
    "assertNotInstanceOf",
    "assertNotInternalType",
    "assertNotIsReadable",
    "assertNotIsWritable",
    "assertNotNull",
    "assertNotRegExp",
    "assertNotSame",
    "assertNotSameSize",
    "assertNotTrue",
    "assertNull",
    "assertObjectEquals",
    "assertObjectHasAttribute",
    "assertObjectHasProperty",
    "assertObjectNotEquals",
    "assertObjectNotHasAttribute",
    "assertObjectNotHasProperty",
    "assertRegExp",
    "assertSame",
    "assertSameSize",
    "assertStringContainsString",
    "assertStringContainsStringIgnoringCase",
    "assertStringContainsStringIgnoringLineEndings",
    "assertStringEndsNotWith",
    "assertStringEndsWith",
    "assertStringEqualsFile",
    "assertStringEqualsFileCanonicalizing",
    "assertStringEqualsFileIgnoringCase",
    "assertStringEqualsStringIgnoringLineEndings",
    "assertStringMatchesFormat",
    "assertStringMatchesFormatFile",
    "assertStringNotContainsString",
    "assertStringNotContainsStringIgnoringCase",
    "assertStringNotEqualsFile",
    "assertStringNotEqualsFileCanonicalizing",
    "assertStringNotEqualsFileIgnoringCase",
    "assertStringNotMatchesFormat",
    "assertStringNotMatchesFormatFile",
    "assertStringStartsNotWith",
    "assertStringStartsWith",
    "assertThat",
    "assertTrue",
    "assertXmlFileEqualsXmlFile",
    "assertXmlFileNotEqualsXmlFile",
    "assertXmlStringEqualsXmlFile",
    "assertXmlStringEqualsXmlString",
    "assertXmlStringNotEqualsXmlFile",
    "assertXmlStringNotEqualsXmlString",
];

pub fn find_assertion_references_in_method<'a>(
    method: &'a Method,
    context: &LintContext<'_>,
) -> Vec<MethodReference<'a>> {
    let MethodBody::Concrete(block) = &method.body else {
        return vec![];
    };

    find_method_references_in_block(block, &|reference| {
        let class_or_object = reference.get_class_or_object();

        if let Expression::Variable(Variable::Direct(variable)) = class_or_object {
            if context.lookup(&variable.name) != "$this" {
                return false;
            }
        } else if !matches!(class_or_object, Expression::Static(_) | Expression::Self_(_)) {
            return false;
        }

        let ClassLikeMemberSelector::Identifier(identifier) = reference.get_selector() else {
            return false;
        };

        let name = context.lookup(&identifier.value);

        ASSERTION_METHODS.contains(&name)
    })
}

pub fn find_testing_or_assertion_references_in_method<'a>(
    method: &'a Method,
    context: &LintContext<'_>,
) -> Vec<MethodReference<'a>> {
    let MethodBody::Concrete(block) = &method.body else {
        return vec![];
    };

    find_method_references_in_block(block, &|reference| {
        let class_or_object = reference.get_class_or_object();

        if let Expression::Variable(Variable::Direct(variable)) = class_or_object {
            if context.lookup(&variable.name) != "$this" {
                return false;
            }
        } else if !matches!(class_or_object, Expression::Static(_) | Expression::Self_(_)) {
            return false;
        }

        let ClassLikeMemberSelector::Identifier(identifier) = reference.get_selector() else {
            return false;
        };

        let name = context.lookup(&identifier.value);

        ASSERTION_METHODS.contains(&name) || TESTING_METHODS.contains(&name)
    })
}