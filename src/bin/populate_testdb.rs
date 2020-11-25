//use sqlx::postgres::PgPoolOptions;
use sqlx::ConnectOptions;
use sqlx::postgres::PgConnectOptions;

static CREATE: &str = r"
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

-- first, last, login, department, title
SELECT * FROM addPerson('Robert', 'Ludlum', 'rludlum', 'Vancouver Artists', 'Digital Artist');
-- SELECT * FROM addPhone('login', 'number', 'category', 'site');
SELECT * FROM addPhone('rludlum', '6432', 'Extension', 'Vancouver');
SELECT * FROM addPhone('rludlum', '6663455432', 'Cell', 'Vancouver');
SELECT * FROM addPhone('rludlum', '6663455432', 'Home', 'Vancouver');

SELECT * FROM addPerson('Robert', 'Kennedy', 'rkennedy', 'Software Development Management', 'Management');
SELECT * FROM addPhone('rkennedy', '6664445555', 'Cell', 'Vancouver');
SELECT * FROM addPhone('rkennedy', '6234', 'Extension', 'Vancouver');

SELECT * FROM addPerson('Robert', 'Beckwith', 'rbeckwith', 'Playa Software', 'Engineer, Software');
SELECT * FROM addPhone('rbeckwith', '3412', 'Extension', 'PlayaVista');
SELECT * FROM addPhone('rbeckwith', '3108889999', 'Cell', 'PlayaVista');

SELECT * FROM addPerson('Stephen', 'Sloan', 'ssloan', 'Mt Employees', 'Supervisors');
SELECT * FROM addPhone('ssloan', '9999', 'Extension', 'Montreal');

SELECT * FROM addPerson('Peter', 'Webber', 'pwebber', 'Vancouver Software', 'Engineer, Software');
SELECT * FROM addPhone('pwebber', '6547891234', 'Cell', 'Vancouver');
SELECT * FROM addPhone('pwebber', '6890', 'Extension', 'Vancouver');
SELECT * FROM addPhone('pwebber', '6789991111', 'Home', 'Vancouver');

SELECT * FROM addPerson('Rober', 'Rabbit', 'rrabbit', 'Vancouver Artists', 'Animator');
SELECT * FROM addPhone('rrabbit', '6100', 'Extension', 'Vancouver');
SELECT * FROM addPhone('rrabbit', '9994569182', 'Home', 'Vanvouver');

SELECT * FROM addPerson('Sam', 'Slade', 'sslade', 'Supervisors', 'Supervisor, Integration');
SELECT * FROM addPhone('sslade', '3567', 'Extension', 'PlayaVista');
SELECT * FROM addPhone('sslade', '3103767091', 'Cell', 'PlayaVista');

SELECT * FROM addPerson('Sam', 'Brown', 'sbrown', 'Vancouver Software', 'Engineer, Software');
SELECT * FROM addPhone('sbrown', '6991', 'Extension', 'Vancouver');
SELECT * FROM addPhone('sbrown', '9991112323', 'Cell', 'Vancouver');

SELECT * FROM addPerson('John', 'Johanson', 'jjo', 'Vancouver Software', 'Engineer, Software');
SELECT * FROM addPhone('jjo', '9994445555', 'Cell', 'Vancouver');

SELECT * FROM addPerson('Fredrik', 'Winklerblod', 'fwink', 'Vancouer Artists', 'Animator');
SELECT * FROM addPhone('fwink', '9878', 'Extension', 'Vancouver');
SELECT * FROM addPhone('fwink', '9879999999', 'Cell', 'Vancouver');

SELECT * FROM addPerson('Fredrich', 'Nietzsche', 'fnietzsche', 'Mt Software', 'Engineer, Software');
SELECT * FROM addPhone('fnietzsche', '8887776666', 'Home', 'Montreal');

SELECT * FROM addPerson('Bob', 'Law', 'blaw', 'Vancouver Artists', 'Compositor');
SELECT * FROM addPhone('blaw', '9121', 'Extension', 'Vancouver');
SELECT * FROM addPhone('blaw', '9993331111', 'Cell', 'Vancouver');

SELECT * FROM addPerson('Johnny', 'Mix', 'jmix', 'Mt Employees', 'Compositor');
SELECT * FROM addPhone('jmix', '8765', 'Extension', 'Montreal');
";

pub async fn setup(
    mut pool: sqlx::PgConnection, 
) -> Result<(), sqlx::Error> {
    let   rows = sqlx::query(&CREATE);
    
    // uncomment to print out query for debugging purposes
    // use sqlx::Execute;
    //println!("sql {}", rows.sql());
    let _ = rows.execute(&mut pool).await?;
                   
    
    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let  pool = PgPoolOptions::new()
    //     .max_connections(1)
    //     .connect(DB_URL).await?;
    let conn = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .database("test")
        .username("postgres")
        .password("example")
        .connect().await?;
    
    let _ = setup(conn).await?; 
    Ok(())
}