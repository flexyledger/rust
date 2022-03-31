// check-fail
// Tests error conditions for specifying diagnostics using #[derive(SessionDiagnostic)]

// The proc_macro2 crate handles spans differently when on beta/stable release rather than nightly,
// changing the output of this test. Since SessionDiagnostic is strictly internal to the compiler
// the test is just ignored on stable and beta:
// ignore-beta
// ignore-stable

#![feature(rustc_private)]
#![crate_type = "lib"]

extern crate rustc_span;
use rustc_span::symbol::Ident;
use rustc_span::Span;

extern crate rustc_macros;
use rustc_macros::SessionDiagnostic;

extern crate rustc_middle;
use rustc_middle::ty::Ty;

extern crate rustc_errors;
use rustc_errors::Applicability;

extern crate rustc_session;

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "hello-world")]
struct Hello {}

#[derive(SessionDiagnostic)]
#[warning(code = "E0123", slug = "hello-world")]
struct HelloWarn {}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
//~^ ERROR `#[derive(SessionDiagnostic)]` can only be used on structs
enum SessionDiagnosticOnEnum {
    Foo,
    Bar,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
#[error = "E0123"]
//~^ ERROR `#[error = ...]` is not a valid `SessionDiagnostic` struct attribute
struct WrongStructAttrStyle {}

#[derive(SessionDiagnostic)]
#[nonsense(code = "E0123", slug = "foo")]
//~^ ERROR `#[nonsense(...)]` is not a valid `SessionDiagnostic` struct attribute
//~^^ ERROR diagnostic kind not specified
//~^^^ ERROR cannot find attribute `nonsense` in this scope
struct InvalidStructAttr {}

#[derive(SessionDiagnostic)]
#[error("E0123")]
//~^ ERROR `#[error("...")]` is not a valid `SessionDiagnostic` struct attribute
//~^^ ERROR `slug` not specified
struct InvalidLitNestedAttr {}

#[derive(SessionDiagnostic)]
#[error(nonsense, code = "E0123", slug = "foo")]
//~^ ERROR `#[error(nonsense)]` is not a valid `SessionDiagnostic` struct attribute
struct InvalidNestedStructAttr {}

#[derive(SessionDiagnostic)]
#[error(nonsense("foo"), code = "E0123", slug = "foo")]
//~^ ERROR `#[error(nonsense(...))]` is not a valid `SessionDiagnostic` struct attribute
struct InvalidNestedStructAttr1 {}

#[derive(SessionDiagnostic)]
#[error(nonsense = "...", code = "E0123", slug = "foo")]
//~^ ERROR `#[error(nonsense = ...)]` is not a valid `SessionDiagnostic` struct attribute
struct InvalidNestedStructAttr2 {}

#[derive(SessionDiagnostic)]
#[error(nonsense = 4, code = "E0123", slug = "foo")]
//~^ ERROR `#[error(nonsense = ...)]` is not a valid `SessionDiagnostic` struct attribute
struct InvalidNestedStructAttr3 {}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct WrongPlaceField {
    #[suggestion = "this is the wrong kind of attribute"]
    //~^ ERROR `#[suggestion = ...]` is not a valid `SessionDiagnostic` field attribute
    sp: Span,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
#[error(code = "E0456", slug = "bar")] //~ ERROR `error` specified multiple times
struct ErrorSpecifiedTwice {}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
#[warning(code = "E0293", slug = "bar")]
//~^ ERROR `warning` specified when `error` was already specified
struct WarnSpecifiedAfterError {}

#[derive(SessionDiagnostic)]
#[error(code = "E0456", code = "E0457", slug = "bar")] //~ ERROR `code` specified multiple times
struct CodeSpecifiedTwice {}

#[derive(SessionDiagnostic)]
#[error(code = "E0456", slug = "foo", slug = "bar")] //~ ERROR `slug` specified multiple times
struct SlugSpecifiedTwice {}

#[derive(SessionDiagnostic)]
struct KindNotProvided {} //~ ERROR diagnostic kind not specified

#[derive(SessionDiagnostic)]
#[error(code = "E0456")] //~ ERROR `slug` not specified
struct SlugNotProvided {}

#[derive(SessionDiagnostic)]
#[error(slug = "foo")]
struct CodeNotProvided {}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct MessageWrongType {
    #[primary_span]
    //~^ ERROR `#[primary_span]` attribute can only be applied to fields of type `Span`
    foo: String,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct InvalidPathFieldAttr {
    #[nonsense]
    //~^ ERROR `#[nonsense]` is not a valid `SessionDiagnostic` field attribute
    //~^^ ERROR cannot find attribute `nonsense` in this scope
    foo: String,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct ErrorWithField {
    name: String,
    #[label = "bar"]
    span: Span,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct ErrorWithMessageAppliedToField {
    #[label = "bar"]
    //~^ ERROR the `#[label = ...]` attribute can only be applied to fields of type `Span`
    name: String,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct ErrorWithNonexistentField {
    #[suggestion(message = "This is a suggestion", code = "{name}")]
    //~^ ERROR `name` doesn't refer to a field on this type
    suggestion: (Span, Applicability),
}

#[derive(SessionDiagnostic)]
//~^ ERROR invalid format string: expected `'}'`
#[error(code = "E0123", slug = "foo")]
struct ErrorMissingClosingBrace {
    #[suggestion(message = "This is a suggestion", code = "{name")]
    suggestion: (Span, Applicability),
    name: String,
    val: usize,
}

#[derive(SessionDiagnostic)]
//~^ ERROR invalid format string: unmatched `}`
#[error(code = "E0123", slug = "foo")]
struct ErrorMissingOpeningBrace {
    #[suggestion(message = "This is a suggestion", code = "name}")]
    suggestion: (Span, Applicability),
    name: String,
    val: usize,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct LabelOnSpan {
    #[label = "bar"]
    sp: Span,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct LabelOnNonSpan {
    #[label = "bar"]
    //~^ ERROR the `#[label = ...]` attribute can only be applied to fields of type `Span`
    id: u32,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct Suggest {
    #[suggestion(message = "This is a suggestion", code = "This is the suggested code")]
    #[suggestion_short(message = "This is a suggestion", code = "This is the suggested code")]
    #[suggestion_hidden(message = "This is a suggestion", code = "This is the suggested code")]
    #[suggestion_verbose(message = "This is a suggestion", code = "This is the suggested code")]
    suggestion: (Span, Applicability),
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct SuggestWithoutCode {
    #[suggestion(message = "This is a suggestion")]
    suggestion: (Span, Applicability),
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct SuggestWithBadKey {
    #[suggestion(nonsense = "This is nonsense")]
    //~^ ERROR `nonsense` is not a valid key for `#[suggestion(...)]`
    suggestion: (Span, Applicability),
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct SuggestWithShorthandMsg {
    #[suggestion(msg = "This is a suggestion")]
    //~^ ERROR `msg` is not a valid key for `#[suggestion(...)]`
    suggestion: (Span, Applicability),
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct SuggestWithoutMsg {
    #[suggestion(code = "This is suggested code")]
    //~^ ERROR missing suggestion message
    suggestion: (Span, Applicability),
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct SuggestWithTypesSwapped {
    #[suggestion(message = "This is a message", code = "This is suggested code")]
    suggestion: (Applicability, Span),
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct SuggestWithWrongTypeApplicabilityOnly {
    #[suggestion(message = "This is a message", code = "This is suggested code")]
    //~^ ERROR wrong field type for suggestion
    suggestion: Applicability,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct SuggestWithSpanOnly {
    #[suggestion(message = "This is a message", code = "This is suggested code")]
    suggestion: Span,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct SuggestWithDuplicateSpanAndApplicability {
    #[suggestion(message = "This is a message", code = "This is suggested code")]
    //~^ ERROR type of field annotated with `#[suggestion(...)]` contains more than one `Span`
    suggestion: (Span, Span, Applicability),
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct SuggestWithDuplicateApplicabilityAndSpan {
    #[suggestion(message = "This is a message", code = "This is suggested code")]
    //~^ ERROR type of field annotated with `#[suggestion(...)]` contains more than one
    suggestion: (Applicability, Applicability, Span),
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct WrongKindOfAnnotation {
    #[label("bar")]
    //~^ ERROR invalid annotation list `#[label(...)]`
    z: Span,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct OptionsInErrors {
    #[label = "bar"]
    label: Option<Span>,
    #[suggestion(message = "suggestion message")]
    opt_sugg: Option<(Span, Applicability)>,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0456", slug = "foo")]
struct MoveOutOfBorrowError<'tcx> {
    name: Ident,
    ty: Ty<'tcx>,
    #[primary_span]
    #[label = "bar"]
    span: Span,
    #[label = "qux"]
    other_span: Span,
    #[suggestion(message = "consider cloning here", code = "{name}.clone()")]
    opt_sugg: Option<(Span, Applicability)>,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct ErrorWithLifetime<'a> {
    #[label = "bar"]
    span: Span,
    name: &'a str,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct ErrorWithDefaultLabelAttr<'a> {
    #[label]
    span: Span,
    name: &'a str,
}

#[derive(SessionDiagnostic)]
//~^ ERROR no method named `into_diagnostic_arg` found for struct `Hello` in the current scope
#[error(code = "E0123", slug = "foo")]
struct ArgFieldWithoutSkip {
    #[primary_span]
    span: Span,
    other: Hello,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct ArgFieldWithSkip {
    #[primary_span]
    span: Span,
    // `Hello` does not implement `IntoDiagnosticArg` so this would result in an error if
    // not for `#[skip_arg]`.
    #[skip_arg]
    other: Hello,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct ErrorWithSpannedNote {
    #[note]
    span: Span,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct ErrorWithSpannedNoteCustom {
    #[note = "bar"]
    span: Span,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
#[note]
struct ErrorWithNote {
    val: String,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
#[note = "bar"]
struct ErrorWithNoteCustom {
    val: String,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct ErrorWithSpannedHelp {
    #[help]
    span: Span,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
struct ErrorWithSpannedHelpCustom {
    #[help = "bar"]
    span: Span,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
#[help]
struct ErrorWithHelp {
    val: String,
}

#[derive(SessionDiagnostic)]
#[error(code = "E0123", slug = "foo")]
#[help = "bar"]
struct ErrorWithHelpCustom {
    val: String,
}

#[derive(SessionDiagnostic)]
#[help]
//~^ ERROR `#[help]` must come after `#[error(..)]` or `#[warn(..)]`
#[error(code = "E0123", slug = "foo")]
struct ErrorWithHelpWrongOrder {
    val: String,
}

#[derive(SessionDiagnostic)]
#[help = "bar"]
//~^ ERROR `#[help = ...]` must come after `#[error(..)]` or `#[warn(..)]`
#[error(code = "E0123", slug = "foo")]
struct ErrorWithHelpCustomWrongOrder {
    val: String,
}

#[derive(SessionDiagnostic)]
#[note]
//~^ ERROR `#[note]` must come after `#[error(..)]` or `#[warn(..)]`
#[error(code = "E0123", slug = "foo")]
struct ErrorWithNoteWrongOrder {
    val: String,
}

#[derive(SessionDiagnostic)]
#[note = "bar"]
//~^ ERROR `#[note = ...]` must come after `#[error(..)]` or `#[warn(..)]`
#[error(code = "E0123", slug = "foo")]
struct ErrorWithNoteCustomWrongOrder {
    val: String,
}
