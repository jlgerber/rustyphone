
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
('Portland', 
'PlayaVista', 
'Vancouver', 
'Montreal', 
'Hyderabad');


------------------------
-- PHONECATEGORY TYPE --
------------------------
CREATE TYPE phonecategory AS ENUM  
('Extension', 
'Home', 
'Cell');


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
WITH cte_pf AS (
   SELECT pp.*,ph.number,ph.category,ph.location
   FROM
      people_phones AS pp
   JOIN phone as ph ON pp.phone_id = ph.id
)
SELECT 
    p.id AS person_id, 
    p.first, 
    p.last, 
    p.first || ' ' || p.last AS fullname, 
    p.login,
    dept.name AS department,
    title.name AS title,
    cte_pf.phone_id, 
    cte_pf.number, 
    cte_pf.category, 
    cte_pf.location
FROM 
    person p
JOIN 
    title 
ON 
    p.title_id = title.id
JOIN 
    department AS dept 
ON 
    p.department_id=dept.id
LEFT JOIN 
    cte_pf 
ON 
    p.id = cte_pf.person_id;


-----------------------
-- ADDPHONE FUNCTION --
-----------------------
CREATE OR REPLACE FUNCTION addPhone (
    login text, 
    number text, 
    category phonecategory, 
    site location
) RETURNS INT AS $$

    WITH Y AS 
        (INSERT 
           INTO phone (number, category, location) 
         VALUES (number, category, site)
         ON CONFLICT DO NOTHING
      RETURNING phone.id), 
    X AS 
        (SELECT id 
           FROM person 
          WHERE person.login=addPhone.login)
    INSERT 
      INTO people_phones (person_id, phone_id)
    SELECT 
        X.id, Y.id 
    FROM 
        X 
    CROSS JOIN 
        Y
    ON CONFLICT DO NOTHING
    RETURNING 
        people_phones.phone_id;
$$
Language 'sql';


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

------------------------------------------------------------
-- DELETEPHONEFROMIDS                                     --
--                                                        --
-- Given a Person id and a Phone id, do the following     --
-- (a) delete the person/phone association and            --
-- (b) delete the phone record itself, as long as it is   --
--     not shared among people                            --
------------------------------------------------------------
CREATE OR REPLACE FUNCTION deletePhoneFromIds(
    person_id INT,
    phone_id INT
    
) RETURNS int AS $$
DECLARE
  relationship_cnt INT := 0;
BEGIN
    SELECT 
        count(people_phones.phone_id)
    INTO
        relationship_cnt
    FROM
        people_phones
    WHERE
        people_phones.person_id = deletePhoneFromIds.person_id;
    IF relationship_cnt > 0 THEN
        DELETE FROM 
            people_phones
        WHERE
            people_phones.phone_id = deletePhoneFromIds.phone_id 
        AND 
            people_phones.person_id = deletePhoneFromIds.person_id;
    END IF;
    IF relationship_cnt = 1 THEN
        DELETE FROM 
            phone 
        WHERE
            phone.id = deletePhoneFromIds.phone_id;
    END IF;
    RETURN 1;
END;
$$
Language 'plpgsql';


---------------------------------------------------------
-- DELETEPHONE                                         --
--                                                     --
-- Given a login, a number a category and a location   --
-- (a) Remove the association between phone and person --
-- (b) Delete the phone if it is not associated with   --
--     additional people.                              --
---------------------------------------------------------
CREATE OR REPLACE FUNCTION deletePhone(
    login text, 
    number text, 
    category phonecategory,
    site location
    
) RETURNS int AS $$
DECLARE
    target_person_id person.id%type;
    target_phone_id phone.id%type;
BEGIN
    SELECT 
        id 
    INTO
        target_person_id
    FROM
        person
    WHERE
        person.login = deletePhone.login;

    IF NOT found THEN
        RETURN 0;
    END IF;

    SELECT 
        id
    INTO 
        target_phone_id
    FROM 
        phone
    WHERE
        deletePhone.number = phone.number
    AND
        deletePhone.category = phone.category
    AND
        deletePhone.site = phone.location;
    
    IF NOT found THEN 
        return 0;
    END IF;

    RETURN (
        SELECT 
            * 
        FROM 
            deletePhoneFromIds(target_person_id, target_phone_id)
    );
END;
$$
Language 'plpgsql';

