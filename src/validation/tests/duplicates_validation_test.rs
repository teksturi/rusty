use crate::{
    ast::{self, SourceRange, SourceRangeFactory, CompilationUnit},
    diagnostics::Diagnostic,
    index::{visitor, Index},
    lexer::{self, IdProvider},
    parser,
    test_utils::tests::parse_and_validate,
    validation::Validator, resolver::{TypeAnnotator, AnnotationMapImpl},
};

#[test]
fn duplicate_pous_validation() {
    // GIVEN two POUs witht he same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        FUNCTION        foo : INT  END_FUNCTION

        PROGRAM         foo  END_PROGRAM

        FUNCTION_BLOCK  foo  END_FUNCTION_BLOCK
    "#,
    );
    // THEN there should be 3 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict_with_text(
                "foo",
                SourceRange::without_file(25..28),
                vec![SourceRange::without_file(74..77),],
                "Ambiguous callable symbol."
            ),
            Diagnostic::global_name_conflict_with_text(
                "foo",
                SourceRange::without_file(74..77),
                vec![SourceRange::without_file(25..28),],
                "Ambiguous callable symbol."
            ),
            Diagnostic::global_name_conflict(
                "foo",
                SourceRange::without_file(25..28),
                vec![
                    SourceRange::without_file(74..77),
                    SourceRange::without_file(116..119),
                ]
            ),
            Diagnostic::global_name_conflict(
                "foo",
                SourceRange::without_file(74..77),
                vec![
                    SourceRange::without_file(25..28),
                    SourceRange::without_file(116..119),
                ]
            ),
            Diagnostic::global_name_conflict(
                "foo",
                SourceRange::without_file(116..119),
                vec![
                    SourceRange::without_file(25..28),
                    SourceRange::without_file(74..77),
                ]
            ),
        ]
    );
}

#[test]
fn duplicate_pous_and_types_validation() {
    // GIVEN a POU and a Type with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        FUNCTION_BLOCK  foo  END_FUNCTION_BLOCK
        TYPE foo : INT END_TYPE
    "#,
    );
    // THEN there should be 3 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict_with_text(
                "foo",
                SourceRange::without_file(62..65),
                vec![SourceRange::without_file(25..28),],
                "Ambiguous datatype."
            ),
            Diagnostic::global_name_conflict_with_text(
                "foo",
                SourceRange::without_file(25..28),
                vec![SourceRange::without_file(62..65),],
                "Ambiguous datatype."
            ),
        ]
    );
}

#[test]
fn duplicate_function_and_type_is_no_issue() {
    // GIVEN a Function and a Type with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        FUNCTION  foo: INT  END_FUNCTION
        TYPE foo : INT END_TYPE
    "#,
    );
    // THEN there should be 0 duplication diagnostics
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn duplicate_global_variables() {
    // GIVEN some duplicate global variables
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        VAR_GLOBAL
            a: INT;
            b: INT;
            c: INT;
        END_VAR

        VAR_GLOBAL
            a: BOOL;
        END_VAR
    
        "#,
    );
    // THEN there should be 0 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict_with_text(
                "a",
                SourceRange::without_file(32..33),
                vec![SourceRange::without_file(128..129),],
                "Ambiguous global variable."
            ),
            Diagnostic::global_name_conflict_with_text(
                "a",
                SourceRange::without_file(128..129),
                vec![SourceRange::without_file(32..33),],
                "Ambiguous global variable."
            ),
        ]
    );
}

#[test]
fn duplicate_variables_in_same_pou() {
    // GIVEN a POU with a duplicate variable
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
        PROGRAM prg
        VAR
            a: INT;
            b: INT;
            c: INT;
        END_VAR
        VAR
            b: BOOL;
        END_VAR
        END_PROGRAM
        "#,
    );
    // THEN there should be 2 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict(
                "prg.b",
                SourceRange::without_file(65..66),
                vec![SourceRange::without_file(133..134),]
            ),
            Diagnostic::global_name_conflict(
                "prg.b",
                SourceRange::without_file(133..134),
                vec![SourceRange::without_file(65..66),]
            ),
        ]
    );
}

#[test]
fn duplicate_enum_members_in_different_types_is_no_issue() {
    // GIVEN a two enums with the same elements
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            TYPE enum1 : (red, green, yellow); END_TYPE
            TYPE enum2 : (red, green, yellow); END_TYPE
        "#,
    );
    // THEN there should be no issues
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn duplicate_fb_inst_and_function() {
    // GIVEN a global fb-instance called foo and a function called foo
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            FUNCTION_BLOCK FooFB
                VAR x : INT END_VAR
            END_FUNCTION_BLOCK

            VAR_GLOBAL
                foo: FooFB;
            END_VAR

            FUNCTION foo: INT
                VAR_INPUT
                    x: INT;
                END_VAR
            END_FUNCTION
        "#,
    );
    // THEN there should be 2 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict_with_text(
                "foo",
                SourceRange::without_file(141..144),
                vec![SourceRange::without_file(195..198),],
                "Ambiguous callable symbol."
            ),
            Diagnostic::global_name_conflict_with_text(
                "foo",
                SourceRange::without_file(195..198),
                vec![SourceRange::without_file(141..144),],
                "Ambiguous callable symbol."
            ),
        ]
    );
}

#[test]
fn duplicate_enum_variables() {
    // GIVEN an enum with two identical elements
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            TYPE enum1 : (red, green, yellow, red); END_TYPE
        "#,
    );
    // THEN there should be 2 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict(
                "enum1.red",
                SourceRange::without_file(27..30),
                vec![SourceRange::without_file(47..50),]
            ),
            Diagnostic::global_name_conflict(
                "enum1.red",
                SourceRange::without_file(47..50),
                vec![SourceRange::without_file(27..30),]
            ),
        ]
    );
}

#[test]
fn duplicate_global_and_program() {
    // GIVEN a global variable `prg` and a Program `prg`
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            VAR_GLOBAL
                a: INT;
                prg: INT;
                b: INT;
            END_VAR

            PROGRAM prg
                VAR_INPUT
                    x: INT;
                END_VAR
            END_PROGRAM
        "#,
    );
    // THEN there should be 2 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict_with_text(
                "prg",
                SourceRange::without_file(64..67),
                vec![SourceRange::without_file(139..142),],
                "Ambiguous global variable."
            ),
            Diagnostic::global_name_conflict_with_text(
                "prg",
                SourceRange::without_file(139..142),
                vec![SourceRange::without_file(64..67),],
                "Ambiguous global variable."
            ),
        ]
    );
}

#[test]
fn duplicate_action_should_be_a_problem() {
    // GIVEN a program with two actions with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            PROGRAM prg
                VAR_INPUT
                    x: INT;
                END_VAR
            END_PROGRAM

            ACTIONS 
            ACTION foo
                x := 2;
            END_ACTION

            ACTION baz
                x := 2;
            END_ACTION

            ACTION foo
                x := 2;
            END_ACTION

            END_ACTIONS
        "#,
    );

    // THEN there should be 2 duplication diagnostics
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::global_name_conflict_with_text(
                "prg.foo",
                SourceRange::without_file(168..171),
                vec![SourceRange::without_file(310..313),],
                "Ambiguous callable symbol."
            ),
            Diagnostic::global_name_conflict_with_text(
                "prg.foo",
                SourceRange::without_file(310..313),
                vec![SourceRange::without_file(168..171),],
                "Ambiguous callable symbol."
            ),
        ]
    );
}

#[test]
fn duplicate_actions_in_different_pous_are_no_issue() {
    // GIVEN two POUs with actions with the same name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            PROGRAM prg
            END_PROGRAM

            ACTIONS 
                ACTION foo END_ACTION
            END_ACTIONS

            PROGRAM prg2
            END_PROGRAM

            ACTIONS 
                ACTION foo END_ACTION
            END_ACTIONS
        "#,
    );

    // THEN there should be no duplication diagnostics
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn automatically_generated_ptr_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a pointer type
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            PROGRAM prg
            VAR
                a: LINT;
                x : INT;
            END_VAR

            a := &x;  //generates ptr_to_INT type
            a := &x;  //also? generates ptr to INT type
            END_PROGRAM
            "#,
    );

    // THEN there should be no duplication diagnostics
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn automatically_generated_string_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a pointer type
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            PROGRAM prg
            VAR
                a: STRING;
            END_VAR

            a := 'abc';  //implicitely creates STRING[4] type
            a := 'xyz';  //implicityly creates STRING[4] type again
            END_PROGRAM
            "#,
    );

    // THEN there should be no duplication diagnostics
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn automatically_generated_byref_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a ref-types
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            FUNCTION foo : INT
                VAR_INPUT {ref}
                    x : INT; //creates autoderef-ptr type to INT
                    y : INT; //creatse autoderef-ptr type to INT
                END_VAR
            END_FUNCTION
        "#,
    );

    // THEN there should be no duplication diagnostics
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn automatically_generated_inout_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a ptr-types
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            FUNCTION foo : INT
                VAR_IN_OUT
                    x : INT; //creates autoderef-ptr type to INT
                    y : INT; //creatse autoderef-ptr type to INT
                END_VAR
            END_FUNCTION
        "#,
    );

    // THEN there should be no duplication diagnostics
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn automatically_generated_output_types_dont_cause_duplication_issues() {
    // GIVEN some code that automatically generates a ptr-types
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        r#"
            FUNCTION foo : INT
                VAR_OUTPUT
                    x : INT; //creates autoderef-ptr type to INT
                    y : INT; //creatse autoderef-ptr type to INT
                END_VAR
            END_FUNCTION
        "#,
    );

    // THEN there should be no duplication diagnostics
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn automatically_generated_output_types_in_different_files_dont_cause_duplication_issues() {
    // a version of the test-util function that does not import the built-in and std-types
    // (or they will cause a duplication issue)
    fn do_index(src: &str, id_provider: IdProvider) -> Index {
        let mut index = Index::default();
        let (mut unit, ..) = parser::parse(
            lexer::lex_with_ids(src, id_provider.clone(), SourceRangeFactory::internal()),
            ast::LinkageType::Internal,
            "test.st",
        );
        ast::pre_process(&mut unit, id_provider.clone());
        index.import(visitor::visit(&unit, id_provider));
        index
    }

    // GIVEN some code that automatically generates a ptr-types
    let ids = IdProvider::default();
    let index1 = do_index(
        r#"
            FUNCTION foo : INT
                VAR_OUTPUT
                    x : INT; //creates autoderef-ptr type to INT
                    y : INT; //creatse autoderef-ptr type to INT
                END_VAR
            END_FUNCTION
        "#,
        ids.clone(),
    );

    //AND another file with also OUTPUT-INTS
    let index2 = do_index(
        r#"
            FUNCTION foo2 : INT
                VAR_OUTPUT
                    x : INT; //creates autoderef-ptr type to INT
                    y : INT; //creatse autoderef-ptr type to INT
                END_VAR
            END_FUNCTION
        "#,
        ids,
    );

    // WHEN the index is combined
    let mut global_index = Index::default();
    global_index.import(index1); //import file 1
    global_index.import(index2); //import file 2

    // THEN there should be no duplication diagnostics
    let mut validator = Validator::new();
    validator.perform_global_validation(&global_index);
    let diagnostics = validator.diagnostics();
    assert_eq!(diagnostics, vec![]);
}


#[test]
fn duplicate_with_generic() {
    // a version of the test-util function that does not import the built-in and std-types
    // (or they will cause a duplication issue)
    fn do_index(src: &str, id_provider: IdProvider, file_name: &str) -> (Index, CompilationUnit) {
        let mut index = Index::default();
        let (mut unit, ..) = parser::parse(
            lexer::lex_with_ids(src, id_provider.clone(), SourceRangeFactory::internal()),
            ast::LinkageType::Internal,
            file_name,
        );
        ast::pre_process(&mut unit, id_provider.clone());
        index.import(visitor::visit(&unit, id_provider));
        (index, unit)
    }

    // GIVEN some code that automatically generates a ptr-types
    let ids = IdProvider::default();
    let (index1, unit1) = do_index(
        r#"
{external}
FUNCTION CONCAT_DATE <T: ANY_INT> : DATE
VAR_INPUT
	year : T;
	month : T;
	day : T;
END_VAR
END_FUNCTION
        "#,
        ids.clone(),
     "file1.st");

    //AND another file with also OUTPUT-INTS
    let (index2, unit2) = do_index(
        r#"
        PROGRAM prg
            CONCAT_DATE(INT#1, SINT#2, SINT#3);
            CONCAT_DATE(DINT#1, SINT#2, SINT#3);
            CONCAT_DATE(INT#1, SINT#2, SINT#3);
            CONCAT_DATE(INT#1, SINT#2, SINT#3);
        END_VAR
        "#,
        ids.clone(), "file2.st"
    );

//AND another file with also OUTPUT-INTS
    let (index3, unit3) = do_index(
        r#"
        PROGRAM prg2
            CONCAT_DATE(INT#1, SINT#2, SINT#3);
            CONCAT_DATE(DINT#1, SINT#2, SINT#3);
            CONCAT_DATE(INT#1, SINT#2, SINT#3);
            CONCAT_DATE(INT#1, SINT#2, SINT#3);
        END_VAR
        "#,
        ids, "file3.st"
    );



    dbg!(index1.get_implementations().values().map(|it| it.get_call_name() ).collect::<Vec<_>>());
    dbg!(index2.get_implementations().values().map(|it| it.get_call_name() ).collect::<Vec<_>>());
    dbg!(index3.get_implementations().values().map(|it| it.get_call_name() ).collect::<Vec<_>>());

    // WHEN the index is combined
    let mut global_index = Index::default();
    global_index.import(index1); //import file 1
    global_index.import(index2); //import file 2
    global_index.import(index3); //import file 3

    // and the resolvers does its job
    let (annotations1, _) = TypeAnnotator::visit_unit(&global_index, &unit1);
    let (annotations2, _) = TypeAnnotator::visit_unit(&global_index, &unit2);
    let (annotations3, _) = TypeAnnotator::visit_unit(&global_index, &unit3);

    dbg!(annotations1.new_index.get_implementations().values().map(|it| it.get_call_name() ).collect::<Vec<_>>());
    dbg!(annotations2.new_index.get_implementations().values().map(|it| it.get_call_name() ).collect::<Vec<_>>());
    dbg!(annotations3.new_index.get_implementations().values().map(|it| it.get_call_name() ).collect::<Vec<_>>());
    
    let mut all_annotations = AnnotationMapImpl::default();
    all_annotations.import(annotations1);
    all_annotations.import(annotations2);
    all_annotations.import(annotations3);

    global_index.import(std::mem::take(&mut all_annotations.new_index));

    dbg!(global_index.get_implementations().values().map(|it| it.get_call_name() ).collect::<Vec<_>>());

    // THEN there should be no duplication diagnostics
    let mut validator = Validator::new();
    validator.perform_global_validation(&global_index);
    let diagnostics = validator.diagnostics();
    assert_eq!(diagnostics, vec![]);
}
