use postgres::{ Client, Column, Error, NoTls, types::Type };
use dotenv;

struct Budget {
    pub name: String,
    pub amount: i32,
    pub deposit: bool,
}

impl Budget {
    pub fn new(name: String, amount: i32, deposit: bool) -> Self {
        Self {
            name,
            amount,
            deposit,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    // let mut client = Client::connect(dotenv::var("DATABASE_URL").unwrap().as_str(), NoTls)?;
    println!("Listening on http://localhost:3000");
    Ok(())
}

fn create_new_budget_table(client: &mut Client, table_name: String) -> Result<(), Error> {
    let query_string =
        format!("CREATE TABLE {} (
        id bigint GENERATED ALWAYS AS IDENTITY,
        name varchar(80),
        amount int,
        deposit bool
        )", table_name);
    client.batch_execute(&query_string.to_string())?;
    Ok(())
}

fn remove_budget_table(client: &mut Client, table_name: String) -> Result<(), Error> {
    let query_string = format!("DROP TABLE {}", table_name);
    client.batch_execute(&query_string.to_string())?;
    Ok(())
}

fn add_new_transaction(
    client: &mut Client,
    table_name: String,
    new_budget: Budget
) -> Result<(), Error> {
    let query_string =
        format!("INSERT INTO {} (name, amount, deposit) VALUES ($1, $2, $3)", table_name);
    client.execute_typed(
        &query_string,
        &[
            (&new_budget.name, Type::VARCHAR),
            (&new_budget.amount, Type::INT4),
            (&new_budget.deposit, Type::BOOL),
        ]
    )?;
    Ok(())
}
fn edit_transaction(
    client: &mut Client,
    table_name: String,
    transaction_id: i64,
    update_budget: Budget
) -> Result<(), Error> {
    let query_string = format!(
        "UPDATE {} SET name = $1, amount = $2, deposit = $3 WHERE id = {}",
        table_name,
        transaction_id
    );

    client.execute_typed(
        &query_string,
        &[
            (&update_budget.name, Type::VARCHAR),
            (&update_budget.amount, Type::INT4),
            (&update_budget.deposit, Type::BOOL),
        ]
    )?;
    Ok(())
}
fn remove_transaction(
    client: &mut Client,
    table_name: String,
    transaction_id: i64
) -> Result<(), Error> {
    let query_string = format!("DELETE FROM {} WHERE id = {}", table_name, transaction_id);
    client.execute_typed(&query_string, &[])?;
    Ok(())
}

fn display_table(client: &mut Client, table_name: String) -> Result<(), Error> {
    let query_string = format!("SELECT * FROM {table_name}");
    for row in client.query(&query_string, &[])? {
        let id: i64 = row.get(0);
        let tx_type: String = row.get(1);
        let amount: i32 = row.get(2);
        let deposit: bool = row.get(3);
        println!("ID: {} Type: {} Amount: {} Deposit: {}", id, tx_type, amount, deposit);
    }
    Ok(())
}
