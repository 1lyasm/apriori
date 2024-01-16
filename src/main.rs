struct Itemset {
    item_list: Vec<String>,
    support: u64
}

struct Itemsets {
    itemset_list: Vec<Itemset>
}

struct Transactions {
    txn_list: Vec<Vec<&'static str>>,
}

struct Rule {
    left: Vec<String>,
    right: Vec<String>,
}

struct Rules {
    rule_list: Vec<Rule>
}

struct Candidates {
    candidate_list: Vec<Itemsets>
}

impl Itemset {
}

impl Itemsets {
    fn find_item_first(&self, target_item: String) -> (bool, usize) {
        let (mut found, mut found_itemset) = (false, std::usize::MAX);
        let mut i = 0;
        while i < self.itemset_list.len() && !found {
            let itemset = self.itemset_list.get(i).unwrap();
            let mut j = 0;
            while j < itemset.item_list.len() && !found {
                let item = itemset.item_list.get(j).unwrap();
                if item == &target_item {
                    found_itemset = i;
                    found = true;
                }
                j += 1;
            }
            i += 1;
        }
        return (found, found_itemset);
    }


    fn delete_infrequent(&mut self, min_support: f64, txn_count: u64) {
        for itemset in &self.itemset_list {
        }
    }
}

impl Transactions {
    fn print(&self) {
        for t in &self.txn_list {
            for item in t {
                print!("{} ", item);
            }
            println!("");
        }
    }

    fn apriori(&self, min_support: f64, txn_count: usize) -> Rules {
        let mut candidates = Candidates::init(&self);
        let frequents = candidates.candidate_list.get_mut(0).unwrap();
        frequents
            .delete_infrequent(min_support, txn_count);
        return Rules{rule_list: Vec::new()};
    }
}

impl Rule {
    fn print_side(side: &Vec<String>) {
        for i in 0..side.len() {
            print!("{}", side.get(i).unwrap());
            if i != side.len() - 1 {
                print!(", ");
            }
        }
    }
}

impl Rules {
    fn print(&self) {
        for rule in &self.rule_list {
            Rule::print_side(&rule.left);
            print!("-> ");
            Rule::print_side(&rule.right);
            print!("\n");
        }
    }
}

impl Candidates {
    fn init(transactions: &Transactions) -> Candidates {
        let mut candidates = Candidates{candidate_list:
            vec![Itemsets{itemset_list: Vec::new()}]};
        let initial_candidates: &mut Itemsets =
            candidates.candidate_list.get_mut(0).unwrap();
        for txn in &transactions.txn_list {
            for item in txn {
                let (contains, index) = initial_candidates.find_item_first(item.to_string());
                if contains {
                    initial_candidates.itemset_list.get_mut(index).unwrap().support += 1;
                } else {
                    initial_candidates.itemset_list.push(
                        Itemset{item_list: vec![item.to_string()], support: 1});
                }
            }
        }
        return candidates;
    }
}

fn main() {
    let transactions = Transactions{txn_list: vec![vec!["a", "b"],
        vec!["c", "d"]]};
    transactions.print();
    let min_support = 0.5;
    transactions.apriori(min_support, transactions.txn_list.len()).print();
}

