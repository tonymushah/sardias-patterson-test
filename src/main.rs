fn is_prefix<W: AsRef<str>, P: AsRef<str>>(word: W, prefix: P) -> bool {
    word.as_ref().starts_with(prefix.as_ref())
}

fn residuals<W: AsRef<str>>(word: W, code: &[String]) -> Vec<String> {
    code.iter()
        .filter(|c| is_prefix(&word, c))
        .map(|c| word.as_ref().replace(c, ""))
        .collect()
}

fn find_a1(code: &[String]) -> Vec<String> {
    let mut a_current = Vec::<String>::new();
    for c in code {
        for r in residuals(c, code) {
            if !r.is_empty() {
                a_current.push(c.clone());
            }
        }
    }
    a_current
}

const ITER_LIMIT: usize = 10;

fn sardinas_patterson(code: &[String]) -> (bool, Vec<Vec<String>>) {
    let mut ans = vec![find_a1(code)];
    let mut i = 0;
    while i < ITER_LIMIT {
        let current_a = ans.last().unwrap();
        let mut new_a: Vec<String> = Vec::new();
        for r in current_a {
            for c in code {
                let new_word = format!("{r}{c}");
                let mut new_residuals = residuals(new_word, code);
                if new_residuals.contains(&String::from("")) {
                    return (false, ans);
                }
                new_a.append(&mut new_residuals);
            }
        }
        if ans.contains(&new_a) {
            return (false, ans);
        }
        if new_a.is_empty() {
            return (true, ans);
        }
        ans.push(new_a);
        i += 1;
    }
    panic!("Iter Limit acheived");
}

fn main() {
    /*let code = vec![
        String::from("he"),
        String::from("hell"),
        String::from("lo"),
        String::from("hello"),
    ];*/
    let code = vec![
        String::from("000"),
        String::from("010"),
        String::from("011"),
        String::from("01001"),
    ];
    //println!("{:?}", residuals("hello", &code));
    let (res, _ans) = sardinas_patterson(&code);
    println!("Is the code uniquely decodable? {res}");
    println!("{:#?}", _ans)
}
