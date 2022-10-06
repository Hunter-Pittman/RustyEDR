use sqlite;

pub fn create_database(db_path: String) {
    let conn = sqlite::open(db_path).unwrap();

    conn
        .execute(
            "CREATE TABLE 'overall' (
                'hostanme'	TEXT UNIQUE,
                'ip_addr'	TEXT,
                'domain'	TEXT,
                'firewall_state'	INTEGER,
                'defender_state'	INTEGER,
                'sysmon_state'	INTEGER,
                PRIMARY KEY('hostanme')
            );"
        ).unwrap();    
}