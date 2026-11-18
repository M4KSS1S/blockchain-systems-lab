use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

trait Database {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn put(&mut self, key: &[u8], value: &[u8]);
    fn writebatch(&mut self,fail_after: Option<usize>, key_value: Vec<(Vec<u8>, Vec<u8>)>) -> Result<(), String>;
}

#[derive(Debug, Clone)]
struct SoufianDb
{
    db : HashMap<Vec<u8>, Vec<u8>>
}

impl Database for SoufianDb
{
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>
    {
        self.db.get(key).cloned()
    }

    fn put(&mut self, key: &[u8], value: &[u8])
    {
        self.db.insert(key.to_vec(), value.to_vec());
    }

    fn writebatch(&mut self,fail_after: Option<usize>, key_value: Vec<(Vec<u8>, Vec<u8>)>) -> Result<(), String>
    {
        let mut overlay = self.clone();
        if fail_after == Some(0)
        {
            return Err("invalid input".to_string());
        }
        for (idx, elem) in key_value.iter().enumerate()
        {
            if elem.0.is_empty() || elem.1.is_empty()
            {
                return Err("invalid input".to_string());
            }
            overlay.put(elem.0.as_slice(), elem.1.as_slice());
            if fail_after == Some(idx + 1)
            {
                return Err("failing after err".to_string());
            }
        }
        self.db = overlay.db;
        Ok(())
    }

} 

fn execute_block(
    balances: &mut HashMap<String, u64>,
    transactions: Vec<Transaction>,
) -> Result<(), String> {
    let mut block_overlay = balances.clone();

    for tx in transactions {
        if tx.from == tx.to
        {
            continue;
        }
        let sender_balance = *block_overlay.get(&tx.from).unwrap_or(&0);

        if sender_balance < tx.amount {
            return Err("invalid transaction".to_string());
        }

        let minus = *block_overlay.get(&tx.from).unwrap_or(&0);
        let plus = *block_overlay.get(&tx.to).unwrap_or(&0);
        let new_sed = minus.checked_sub(tx.amount).ok_or("underflow amount".to_string())?;
        let new_rec = plus.checked_add(tx.amount).ok_or("overflow amount".to_string())?;
        block_overlay.insert(tx.from,new_sed);
        block_overlay.insert(tx.to,new_rec);
    }
    *balances = block_overlay;
    Ok(())
}

fn main() {}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_execute()
    {
        let transactions = vec![
            Transaction {
                from: "Alice".into(),
                to: "Bob".into(),
                amount: 80,
            },
            Transaction {
                from: "Bob".into(),
                to: "Charlie".into(),
                amount: 50,
            },
        ];
        let mut balance = HashMap::new();
        balance.insert("Bob".into(), 0);
        balance.insert("Alice".into(), 100);
        balance.insert("Charlie".into(), 0);
        let _ = execute_block(&mut balance, transactions);
        assert_eq!(balance.get("Bob"), Some(&30));
        assert_eq!(balance.get("Alice"), Some(&20));
        assert_eq!(balance.get("Charlie"), Some(&50));
    }

    #[test]
    fn test_database_put_and_get() {
        let mut db = SoufianDb {
            db: HashMap::new(),
        };

        db.put(&b"alice".to_vec(), &b"100".to_vec());
        let result = db.writebatch(
                    Some(3) ,
                vec![
                    (b"alice".to_vec(), b"50".to_vec()),
                    (b"bob".to_vec(), b"200".to_vec()),
                    (vec![], b"50".to_vec()),
                    ]
        );
        assert_eq!(result, Err("invalid input".to_string()));
        assert_eq!(db.get(b"alice"), Some(b"100".to_vec()));
        assert_eq!(db.get(b"bob"), None);
    }
}