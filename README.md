# Collect Args

```rs
let args = Args::collect();
let bar = args.input("foo").1.expect("Following value is required");
let (kek, is_kek) = args.flag("kek");
let opts = args.options(&["bar", "baz"]);

match opts.unwrap_or(String::from("nothing")) {
  String::from("bar") => todo!()
  String::from("baz") => todo!()
  _ => todo!()
}

let (sel_key, opt) = args.select("sel_key", &["opt1", "opt2", "opt3"]);
let opt_value = opr.expect("Please enter {sel_key} option");
```