
----------------
-- DEPARTMENT --
----------------
CREATE TABLE IF NOT EXISTS department
(
id  SERIAL   PRIMARY KEY,
name VARCHAR(256) NOT NULL UNIQUE
);
---------------
--   TITLE   --
---------------
CREATE TABLE IF NOT EXISTS title
(
id SERIAL PRIMARY KEY,
name VARCHAR(256) NOT NULL UNIQUE
);

------------------
-- PERSON TABLE --
------------------
CREATE TABLE IF NOT EXISTS person 
(
    id     SERIAL       PRIMARY KEY,
    first  VARCHAR(256) NOT NULL,
    last   VARCHAR(256) NOT NULL,
    login  VARCHAR(256) NOT NULL UNIQUE,
    department_id INT NOT NULL,
    title_id INT NOT NULL,
    UNIQUE(first, last),
    CONSTRAINT fk_department
        FOREIGN KEY(department_id)
            REFERENCES department(id),
    CONSTRAINT fk_title
        FOREIGN KEY(title_id)
            REFERENCES title(id)

) ;

-------------------
-- LOCATION TYPE --
-------------------
CREATE TYPE location AS ENUM  
('portland', 
'playavista', 
'vancouver', 
'montreal', 
'hyderabad');


------------------------
-- PHONECATEGORY TYPE --
------------------------
CREATE TYPE phonecategory AS ENUM  
('extension', 
'home', 
'cell');


-----------------
-- PHONE TABLE --
-----------------
CREATE TABLE IF NOT EXISTS phone 
(
    id        SERIAL        PRIMARY KEY,
    number    VARCHAR(30)   NOT NULL CHECK(number ~ '^[0-9]+$'),
    category  phonecategory NOT NULL,
    location  location      NOT NULL,
        UNIQUE(number, category, location)
);


-------------------------
-- PEOPLE_PHONES TABLE --
-------------------------
CREATE TABLE IF NOT EXISTS people_phones  
(
    person_id int NOT NULL,
    phone_id  int NOT NULL,
    PRIMARY KEY (person_id, phone_id),
    FOREIGN KEY (person_id) REFERENCES person(id)  ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (phone_id) REFERENCES phone(id) ON UPDATE CASCADE ON DELETE CASCADE
);


---------------------
-- PERSONVIEW VIEW --
---------------------
CREATE OR REPLACE VIEW personview AS
SELECT 
    p.id AS person_id, 
    p.first, 
    p.last, 
    p.first || ' ' || p.last as fullname, 
    p.login,
    dept.name as department,
    title.name as title,
    ph.id AS phone_id, 
    ph.number, 
    ph.category, 
    ph.location
 FROM 
    person p,
    title,
    department dept,
    phone ph,
    people_phones
WHERE 
    people_phones.person_id=p.id 
AND 
    people_phones.phone_id=ph.id
AND
    department_id = dept.id
AND
    title_id = title.id;


-----------------------
-- ADDPHONE FUNCTION --
-----------------------
CREATE OR REPLACE FUNCTION addPhone (
    login varchar, 
    number text, 
    category phonecategory, 
    site location
) RETURNS int AS $$

BEGIN
    WITH Y AS 
        (INSERT 
           INTO phone (number, category, location) 
         VALUES (number, category, site)
      RETURNING id), 
    X AS 
        (SELECT id 
           FROM person 
          WHERE person.login=addPhone.login)
    INSERT 
      INTO people_phones (person_id, phone_id)
    SELECT X.id, Y.id 
      FROM X CROSS JOIN Y;
    RETURN 1;
END;
$$
Language 'plpgsql';


CREATE OR REPLACE FUNCTION addPerson(
    first varchar, 
    last varchar,
    login varchar,
    department varchar,
    title varchar
    
) RETURNS int AS $$

BEGIN
    WITH cte_title AS (
    SELECT
        id
    FROM
        title
    WHERE
        name = addPerson.title
    
),
      cte_dept AS (
        SELECT
            id
        FROM
          department
        WHERE
            name = addPerson.department
)
INSERT INTO 
    person (first, last, login, department_id, title_id)
VALUES
    (addPerson.first, addPerson.last, addPerson.login, (SELECT id FROM cte_dept), ( SELECT id FROM cte_title));
    RETURN 1;
END;
$$
Language 'plpgsql';