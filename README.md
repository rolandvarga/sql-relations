Trying to parse & match SQL files

```bash
INFO  sql_relations > running 'sql-relations' with version '0.1.0'
INFO  sql_relations > -- ------------------------------------------
INFO  sql_relations >
+---------------+----------------+-------------+--------------------------------------------+
| FILE          | STATEMENT_TYPE | TABLES      | USED_BY                                    |
+===========================================================================================+
| insert_vg.sql | Insert         | video_games | select_with_join.sql, select_with_cols.sql |
+---------------+----------------+-------------+--------------------------------------------+
```

#### TODO
[ ] handle CTEs
[ ] support multiple statements per file