------------------
-- PERSON TABLE --
------------------
CREATE TABLE IF NOT EXISTS person 
(
    id     SERIAL       PRIMARY KEY,
    first  VARCHAR(256) NOT NULL,
    last   VARCHAR(256) NOT NULL,
    login  VARCHAR(256) NOT NULL UNIQUE,
        UNIQUE(first, last)
) ;

----------------
-- DEPARTMENT --
----------------
-- CREATE TABLE IF NOT EXISTS department
-- id  SERIAL   PRIMARY KEY,

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
SELECT p.id AS person_id, p.first, p.last, p.login,
       ph.id AS phone_id, ph.number, ph.category, ph.location
 FROM person p,phone ph,people_phones 
WHERE people_phones.person_id=p.id 
  AND people_phones.phone_id=ph.id;


-----------------------
-- ADDPHONE FUNCTION --
-----------------------
CREATE OR REPLACE FUNCTION addPhone(
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
