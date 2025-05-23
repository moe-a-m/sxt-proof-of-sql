use crate::{
    base::{
        database::{
            owned_table_utility::{
                bigint, decimal75, int, int128, owned_table, smallint, timestamptz, tinyint, uint8,
            },
            ColumnType, LiteralValue, OwnedTableTestAccessor, TableRef,
        },
        math::decimal::Precision,
        posql_time::{PoSQLTimeUnit, PoSQLTimeZone},
    },
    sql::{
        proof::{exercise_verification, VerifiableQueryResult},
        proof_exprs::{
            test_utility::{aliased_plan, column, scaling_cast, tab},
            LiteralExpr,
        },
        proof_plans::test_utility::filter,
    },
};
use blitzar::proof::InnerProductProof;

#[test]
fn we_can_prove_a_simple_scale_cast_expr_from_int_to_decimal() {
    let data = owned_table([
        tinyint("a", [1]),
        uint8("b", [1]),
        smallint("c", [1i16]),
        int("d", [1i32]),
        bigint("e", [1i64]),
        int128("f", [1i128]),
    ]);
    let t = TableRef::new("sxt", "t");
    let accessor =
        OwnedTableTestAccessor::<InnerProductProof>::new_from_table(t.clone(), data, 0, ());
    let ast = filter(
        vec![
            aliased_plan(
                scaling_cast(
                    column(&t, "a", &accessor),
                    ColumnType::Decimal75(Precision::new(4).unwrap(), 1),
                ),
                "a_cast",
            ),
            aliased_plan(
                scaling_cast(
                    column(&t, "b", &accessor),
                    ColumnType::Decimal75(Precision::new(4).unwrap(), 1),
                ),
                "b_cast",
            ),
            aliased_plan(
                scaling_cast(
                    column(&t, "c", &accessor),
                    ColumnType::Decimal75(Precision::new(6).unwrap(), 1),
                ),
                "c_cast",
            ),
            aliased_plan(
                scaling_cast(
                    column(&t, "d", &accessor),
                    ColumnType::Decimal75(Precision::new(11).unwrap(), 1),
                ),
                "d_cast",
            ),
            aliased_plan(
                scaling_cast(
                    column(&t, "e", &accessor),
                    ColumnType::Decimal75(Precision::new(20).unwrap(), 1),
                ),
                "e_cast",
            ),
            aliased_plan(
                scaling_cast(
                    column(&t, "f", &accessor),
                    ColumnType::Decimal75(Precision::new(40).unwrap(), 1),
                ),
                "f_cast",
            ),
        ],
        tab(&t),
        super::DynProofExpr::Literal(LiteralExpr::new(LiteralValue::Boolean(true))),
    );
    let verifiable_res = VerifiableQueryResult::new(&ast, &accessor, &(), &[]).unwrap();
    exercise_verification(&verifiable_res, &ast, &accessor, &t);
    let res = verifiable_res
        .verify(&ast, &accessor, &(), &[])
        .unwrap()
        .table;
    let expected_res = owned_table([
        decimal75("a_cast", 4, 1, [10]),
        decimal75("b_cast", 4, 1, [10]),
        decimal75("c_cast", 6, 1, [10]),
        decimal75("d_cast", 11, 1, [10]),
        decimal75("e_cast", 20, 1, [10]),
        decimal75("f_cast", 40, 1, [10]),
    ]);
    assert_eq!(res, expected_res);
}

#[test]
fn we_can_prove_a_simple_scale_cast_expr_from_decimal_to_decimal() {
    let data = owned_table([
        decimal75("a", 4, -2, [10]),
        decimal75("b", 4, 1, [1]),
        decimal75("c", 6, 0, [10]),
    ]);
    let t = TableRef::new("sxt", "t");
    let accessor =
        OwnedTableTestAccessor::<InnerProductProof>::new_from_table(t.clone(), data, 0, ());
    let ast = filter(
        vec![
            aliased_plan(
                scaling_cast(
                    column(&t, "a", &accessor),
                    ColumnType::Decimal75(Precision::new(5).unwrap(), -1),
                ),
                "a_cast",
            ),
            aliased_plan(
                scaling_cast(
                    column(&t, "b", &accessor),
                    ColumnType::Decimal75(Precision::new(5).unwrap(), 2),
                ),
                "b_cast",
            ),
            aliased_plan(
                scaling_cast(
                    column(&t, "c", &accessor),
                    ColumnType::Decimal75(Precision::new(7).unwrap(), 0),
                ),
                "c_cast",
            ),
        ],
        tab(&t),
        super::DynProofExpr::Literal(LiteralExpr::new(LiteralValue::Boolean(true))),
    );
    let verifiable_res = VerifiableQueryResult::new(&ast, &accessor, &(), &[]).unwrap();
    exercise_verification(&verifiable_res, &ast, &accessor, &t);
    let res = verifiable_res
        .verify(&ast, &accessor, &(), &[])
        .unwrap()
        .table;
    let expected_res = owned_table([
        decimal75("a_cast", 5, -1, [100]),
        decimal75("b_cast", 5, 2, [10]),
        decimal75("c_cast", 7, 0, [10]),
    ]);
    assert_eq!(res, expected_res);
}

#[test]
fn we_can_prove_a_simple_scale_cast_expr_from_timestamp_to_timestamp() {
    let data = owned_table([timestamptz(
        "a",
        PoSQLTimeUnit::Second,
        PoSQLTimeZone::new(0),
        [1],
    )]);
    let t = TableRef::new("sxt", "t");
    let accessor =
        OwnedTableTestAccessor::<InnerProductProof>::new_from_table(t.clone(), data, 0, ());
    let ast = filter(
        vec![aliased_plan(
            scaling_cast(
                column(&t, "a", &accessor),
                ColumnType::TimestampTZ(PoSQLTimeUnit::Millisecond, PoSQLTimeZone::new(0)),
            ),
            "a_cast",
        )],
        tab(&t),
        super::DynProofExpr::Literal(LiteralExpr::new(LiteralValue::Boolean(true))),
    );
    let verifiable_res = VerifiableQueryResult::new(&ast, &accessor, &(), &[]).unwrap();
    exercise_verification(&verifiable_res, &ast, &accessor, &t);
    let res = verifiable_res
        .verify(&ast, &accessor, &(), &[])
        .unwrap()
        .table;
    let expected_res = owned_table([timestamptz(
        "a_cast",
        PoSQLTimeUnit::Millisecond,
        PoSQLTimeZone::new(0),
        [1000],
    )]);
    assert_eq!(res, expected_res);
}
