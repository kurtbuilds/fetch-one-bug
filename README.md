# Errors

Database: local
Method: fetch_all
Message: new row for relation \"job\" violates check constraint \"job_vs_job_template_constraints\"

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Database(PgDatabaseError { severity: Error, code: "23514", message: "new row for relation \"job\" violates check constraint \"job_vs_job_template_constraints\"", detail: Some("Failing row contains (666, , null, null, f, f, f, null, null, 0.00, 0.00, , null, , null, null, f, null, null, null, null, null, null, null, null, null, null, t, f, null, null, null, null, null, null, null, null, null, null, null, null, 1, null, 2, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, 0.00, null, none, null, f, null, null, null, null, null, null, null, null, 0.00, null, null, null, null, null, null, null, f)."), hint: None, position: None, where: None, schema: Some("public"), table: Some("job"), column: None, data_type: None, constraint: Some("job_vs_job_template_constraints"), file: Some("execMain.c"), line: Some(1939), routine: Some("ExecConstraints") })', src/main.rs:137:41
```

Database: local
Method: fetch_one
Message: new row for relation \"job\" violates check constraint \"job_vs_job_template_constraints\"

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Database(PgDatabaseError { severity: Error, code: "23514", message: "new row for relation \"job\" violates check constraint \"job_vs_job_template_constraints\"", detail: Some("Failing row contains (666, , null, null, f, f, f, null, null, 0.00, 0.00, , null, , null, null, f, null, null, null, null, null, null, null, null, null, null, t, f, null, null, null, null, null, null, null, null, null, null, null, null, 1, null, 2, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, 0.00, null, none, null, f, null, null, null, null, null, null, null, null, 0.00, null, null, null, null, null, null, null, f)."), hint: None, position: None, where: None, schema: Some("public"), table: Some("job"), column: None, data_type: None, constraint: Some("job_vs_job_template_constraints"), file: Some("execMain.c"), line: Some(1939), routine: Some("ExecConstraints") })', src/main.rs:137:41
```

> **Observation**:
> 
> job_vs_job_template_constraints is MISSING from jyve_test

Database: test
Method: fetch_one

This scenario returns no error, but does not insert data into the database. Conceivably what's happening is that
data is inserted and immediately removed by a trigger? That's my best guess of what's happening. But then why
does fetch_all fail?

Observe that executing this query with DEFAULT for the id increments that value. Is that normally how failing 
insertions work? It's reasonably enough I guess, no guarantee that it's sequential.

Database: test
Method: fetch_all
Message: insert or update on table \"job\" violates foreign key constraint \"job_store_location_id_569bf3a6_fk_store_location_id\

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Database(PgDatabaseError { severity: Error, code: "23503", message: "insert or update on table \"job\" violates foreign key constraint \"job_store_location_id_569bf3a6_fk_store_location_id\"", detail: Some("Key (store_location_id)=(1) is not present in table \"store_location\"."), hint: None, position: None, where: None, schema: Some("public"), table: Some("job"), column: None, data_type: None, constraint: Some("job_store_location_id_569bf3a6_fk_store_location_id"), file: Some("ri_triggers.c"), line: Some(2539), routine: Some("ri_ReportViolation") })', src/main.rs:137:41
```

> **Hypothesis**:
>
> fetch_one appears to be returning the NEW row in postgres. There's a subtle difference in the error messages,
> where one describes a problem with new row, whereas the other says that insertion failed.


> **Confirmation**: With the check constraint, fetch_one DOES error correctly.

So why does foreign key constraint return a row?

# Minimal example

