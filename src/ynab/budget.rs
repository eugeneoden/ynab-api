pub struct Budget {
    budget: ynab_api::models::BudgetDetail,
}

impl Budget {
    pub fn new(budget: ynab_api::models::BudgetDetail) -> Self {
        Self { budget }
    }

    pub fn name(&self) -> String {
        self.budget.name.clone()
    }

    pub fn id(&self) -> String {
        self.budget.id.clone()
    }

    pub fn reimbursables(&self) -> Vec<super::transaction::Transaction> {
        let reimbursables_id =
            if let Some(categories) = &self.budget.categories {
                categories
                    .iter()
                    .find(|c| c.name == "Reimbursables")
                    .map(|c| c.id.clone())
                    .unwrap()
            } else {
                panic!("no categories found")
            };

        let mut reimbursables = vec![];
        if let Some(payees) = &self.budget.payees {
            let mut payee_map = std::collections::HashMap::new();
            for p in payees {
                payee_map.insert(p.id.clone(), p.name.clone());
            }
            let payee_map = payee_map;

            let mut transaction_map = std::collections::HashMap::new();
            if let Some(transactions) = &self.budget.transactions {
                for t in transactions {
                    transaction_map.insert(t.id.clone(), t);

                    if let Some(category_id) = &t.category_id {
                        if category_id != &reimbursables_id {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    let payee = t
                        .payee_id
                        .iter()
                        .flat_map(|payee_id| payee_map.get(payee_id).cloned())
                        .next()
                        .unwrap_or_else(|| "(none)".to_string());
                    let reimbursed = if let Some(color) = &t.flag_color {
                        color == "green"
                    } else {
                        false
                    };

                    reimbursables.push(super::transaction::Transaction {
                        date: t.date.clone(),
                        payee,
                        amount: t.amount,
                        total_amount: t.amount,
                        reimbursed,
                    })
                }
            }
            let transaction_map = transaction_map;

            if let Some(subtransactions) = &self.budget.subtransactions {
                for st in subtransactions {
                    if let Some(category_id) = &st.category_id {
                        if category_id != &reimbursables_id {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    let t = transaction_map[&st.transaction_id];
                    let payee = st
                        .payee_id
                        .iter()
                        .flat_map(|payee_id| payee_map.get(payee_id).cloned())
                        .next()
                        .unwrap_or_else(|| {
                            t.payee_id
                                .iter()
                                .flat_map(|payee_id| {
                                    payee_map.get(payee_id).cloned()
                                })
                                .next()
                                .unwrap_or_else(|| "(none)".to_string())
                        });
                    let reimbursed = if let Some(color) = &t.flag_color {
                        color == "green"
                    } else {
                        false
                    };

                    reimbursables.push(super::transaction::Transaction {
                        date: t.date.clone(),
                        payee,
                        amount: st.amount,
                        total_amount: t.amount,
                        reimbursed,
                    })
                }
            }
        } else {
            panic!("no payees?");
        }

        reimbursables.sort_by_cached_key(|t| t.date.clone());
        reimbursables
    }
}