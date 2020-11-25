# userdb
The `userdb` crate provides an api for interfacing with the postgres database used to store and present user information, primarily via the `phone` command.

## Setting up the testdb
Run the following in a shell from the project

```bash
target/release/teardown-testdb
sqlx migrate run
target/release/populate-testdb
```

## Developer Notes

### `Notes On My First Exploration of sqlx`

We need to have a postgres db to play with. lets jump to [docker hub](https://hub.docker.com/_/postgres) and use `stack deploy` (swarm) or `docker-compose` to stand one up.

#### `The stack.yml contents`
```yaml
# Use postgres/example user/password credentials
version: '3.1'

services:

  db:
    image: postgres
    restart: always
    environment:
      POSTGRES_PASSWORD: example

  adminer:
    image: adminer
    restart: always
    ports:
      - 8080:8080
```

#### `Execute docker compose`

```bash
docker-compose -f stack.yml up
```

### Notes on Sql Usage

#### `Inserting a Number`
```sql
WITH Y AS 
    (INSERT INTO phone (number, category, location) 
     VALUES ('3103867796', 'cell', 'portland')
     RETURNING id), 
X AS 
    (SELECT id 
       FROM person 
      WHERE first='Jane' 
        AND lASt='Doe')
INSERT INTO people_phones (person_id, phone_id)
SELECT Y.id, X.id 
  FROM X 
 CROSS JOIN Y;
```
#### `selecting info`
```sql
SELECT * 
  FROM 
     (SELECT p.id AS phone_id, p.first, p.lASt, p.login,
             ph.id AS phone_id, ph.number, ph.category, ph.location
        FROM person p,phone ph,people_phones 
       WHERE people_phones.person_id=p.id 
         AND people_phones.phone_id=ph.id) AS x 
  WHERE x.category='extension';
```

#### `Returning json`
```sql
SELECT row_to_json(ln) AS personview 
  FROM ( SELECT pv.person_id, pv.first, pv.last, pv.login,
           ( SELECT json_agg(rowval) AS phones 
               FROM 
                  ( SELECT phone_id, number, category, location 
                      FROM personview 
                     WHERE person_id = pv.person_id
                  ) 
             rowval
            ) 
         FROM personview AS pv
       ) AS ln;
```
#### `Building up Json with Qualifications`

```sql
WITH pview AS
  ( 
      SELECT * 
      FROM personview
      WHERE login='jdoe'
  )
SELECT row_to_json(ln) AS personview 
  FROM ( SELECT pv.person_id, pv.first, pv.last, pv.login,
           ( SELECT json_agg(rowval) AS phones 
               FROM 
                  ( SELECT phone_id, number, category, location 
                      FROM pview 
                     WHERE person_id = pv.person_id
                  ) 
             rowval
            ) 
         FROM pview AS pv
       ) AS ln;
```

### Population Of Data

```sql
INSERT INTO 
    department(name) 
VALUES 
    ('Features'),
    ('Features Billable'), 
    ('Vancouver Artists'),
    ('Vancouver Software'),
    ('Playa Artists'), 
    ('Playa Software'),
    ('Mt Employees'), 
    ('Mt Software'),
    ('DDIndia'),
    ('NORMAL ACCOUNT'), 
    ('Software Development Management'), 
    ('Technical Directors'), 
    ('Supervisors'),
    ('Dfx Supervisors'),
    ('Virtual Reality');

INSERT INTO 
    title(name)
VALUES 
    ('Lighting Artist'), 
    ('Compositor'),
    ('Digital Artist'),
    ('Animator'),
    ('Paint Artist'),
    ('Coordinator'),
    ('Management'),
    ('Assistant Controller'),
    ('Stage Lead'),
    ('External Contractor'),
    ('Engineer, Software'),
    ('Effects Technical Director'),
    ('Unity Developer'),
    ('Senior Producer'),
    ('Supervisor, Integration'),
    ('Supervisor, Rotoscope'),
    ('Supervisor, Modeling'),
    ('Character Fx Supervisor'),
    ('Supervisor, Computer Graphics'),
    ('Supervisor, Visual Effects'),
    ('Supervisor, Pipeline'),
    ('Environments Department Supervisor'),
    ('Lighting Department Supervisor'),
    ('Fx Department Supervisor'),
    ('Animation Director'),
    ('Directory, Technology'),
    ('Director of Software R&D'),
    ('Director');

```

```sql

WITH cte_title AS (
    SELECT
        id
    FROM
        title
    WHERE
        name = 'Supervisor, Integration'
    
),
      cte_dept AS (
        SELECT
            id
        FROM
          department
        WHERE
            name = 'Supervisors'
)
INSERT INTO 
    person (first, last, login, department_id, title_id)
VALUES
    ('Som', 'Shankar', 'som', (SELECT id FROM cte_dept), ( SELECT id FROM cte_title));
```

#### `aggregating all phones into array`

This returns all json

```sql
WITH pview AS
    ( 
        SELECT * 
        FROM personview
        WHERE fullname ILIKE 'sam%'
    )
    SELECT row_to_json(ln2) as pv2 from (
    SELECT DISTINCT ON (person_id) *  
    FROM ( SELECT pv.person_id, pv.first, pv.last, pv.login,
            ( SELECT json_agg(rowval) AS phones 
                FROM 
                    ( SELECT phone_id, number, category, location 
                        FROM pview 
                        WHERE person_id = pv.person_id
                    ) 
                rowval
            ) 
            FROM pview AS pv
        ) AS ln
    ) AS ln2;
```

whereas this turns the phone into an array of json

```sql
WITH pview AS
    ( 
        SELECT * 
        FROM personview
        WHERE fullname ILIKE 'sam%'
    )
    SELECT DISTINCT ON (person_id) *  
    FROM ( SELECT pv.person_id, pv.first, pv.last, pv.login,
        ( SELECT json_agg(rowval) AS phones 
            FROM 
                ( SELECT phone_id, number, category, location 
                    FROM pview 
                    WHERE person_id = pv.person_id
                ) 
            rowval
        ) 
        FROM pview AS pv
    ) AS ln;
```

### Examples of sqlx usage from query.rs
This crate was initally just a way of exploring sqlx.
These are examples of using different query strategies.

```rust

pub async fn using_fetch_all(pool: &sqlx::PgPool, fields: &str, age_q: i16, sex: &str) -> Result<(),sqlx::Error> {
    let select = format!("SELECT {} FROM person where age>$1 and sex=$2",fields);
    let  names = sqlx::query_as::<_,Person>(&select)
    .bind(age_q)
    .bind(sex)
    .fetch_all(pool).await?;
    
    println!("{:#?}", names);
    Ok(())
}

pub async fn using_query(pool: &sqlx::PgPool, fields: &str, age_q: i16, sex: &str) -> Result<(), sqlx::Error> {
    let select = format!("SELECT {} FROM person where age>$1 and sex=$2",fields);
    let mut rows = sqlx::query(&select)
    .bind(age_q)
    .bind(sex)
    .fetch(pool);
    while let Some(row) = rows.try_next().await? {
        let person = Person::from_row(&row).unwrap();   
        print_person(&person);
    
    }
    Ok(())
}
```