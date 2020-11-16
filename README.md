# My first exploration of sqlx

need 
to have a postgres db to play with. lets jump to docker:
[url](https://hub.docker.com/_/postgres)

 use stack deploy (swarm) or docker-compose

### stack.yml
```bASh
# Use postgres/example user/pASsword credentials
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

### execute docker compose
```bASh
docker-dompose -f stack.yml up
```

## queries
### inserting a number
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
### selecting
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

# building up json
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
# Building up Json with qualifications

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