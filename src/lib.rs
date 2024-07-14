/// Struct for keeping all `env.args()` in `Vec<String>`
pub struct Args {
  pub args: Vec<String>
}

impl Args {
  /// Collects args and skips the first one which is binary name.  
  pub fn collect() -> Self {
    Self {
      args: std::env::args().skip(1).collect()
    }
  }
  /// Collects key-value pair
  /// # Examples
  /// ```
  /// use collect_args::Args;
  /// 
  /// let args = Args {
  ///   args: vec![String::from("foo"), String::from("bar")]
  /// };

  /// assert_eq!(("foo", Some(String::from("bar"))), args.input("foo"))
  /// ```
  pub fn input<'a>(&'a self, key: &'a str) -> (&'a str, Option<String>) {
    let value = self.args
      .iter()
      .enumerate()
      .find_map(|(i, arg)| {
        self.args.get(i + 1).filter(|_| key == arg).and_then(|s| Some(s.to_owned()))
      });

    (key, value)
  }
  /// Options provided by key
  /// 
  /// # Examples
  /// ```
  /// use collect_args::Args;
  /// 
  /// let args = Args {
  ///   args: vec![String::from("sel"), String::from("baz")]
  /// };
  /// 
  /// assert_eq!(args.select("sel", &["foo", "bar", "baz"]), ("sel", Some(String::from("baz"))));
  /// ```
  pub fn select<'a>(&'a self, key: &'a str, args: &'a [&str]) -> (&'a str, Option<String>) {
    let value = self.args
      .iter()
      .enumerate()
      .find_map(|(i, arg)| {
        if key == arg {
          self.args.get(i + 1)
            .and_then(|val| args.iter().find(|sel| **sel == val))
            .and_then(|s| Some(s.to_string()))
        } else {
          None
        }
      });
    
    (key, value)
  }
  
  /// Takes arg includes in array of string slice
  /// # Examples
  /// ```
  /// use collect_args::Args;
  /// let args = Args {
  ///   args: vec![String::from("foo"), String::from("bar")]
  /// };
  /// 
  /// assert_eq!(Some(String::from("foo")), args.options(&["foo", "bar", "baz"]));
  /// ```
  pub fn options<'a>(&self, opts: &'a [&str]) -> Option<String> {
    self.args.iter()
    .find(|arg| opts.contains(&arg.as_str()))
    .map(|s| s.to_owned())
  }
  /// If flas_arg is entered then it's true. False by default
  /// 
  /// # Examples
  /// ```
  /// use collect_args::Args;
  /// 
  ///  let args = Args {
  ///     args: vec![String::from("foo"), String::from("bar")]
  ///  };

  ///  assert_eq!(args.flag("foo"), ("foo", true));
  ///  assert_eq!(args.flag("bar"), ("bar", true));
  ///  assert_eq!(args.flag("baz"), ("baz", false));
  /// ```
  pub fn flag<'a>(&'a self, flag_arg: &'a str) -> (&'a str, bool) {
    match self.args.iter().find(|s| *s == flag_arg) {
      Some(arg) => (arg, true),
      None => (flag_arg, false)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::*;
  
  
  #[test]
  fn collects_input() {
    let args = Args {
      args: vec![String::from("foo"), String::from("bar")]
    };
    
    assert_eq!(("foo", Some(String::from("bar"))), args.input("foo"));

    let args = Args {
      args: vec![String::from("foo")]
    };

    assert_eq!(("foo", None), args.input("foo"))
  } 
  
  #[test]
  fn collect_options() {
    let args = Args {
      args: vec![String::from("foo"), String::from("bar")]
    };

    assert_eq!(Some(String::from("foo")), args.options(&["foo", "bar", "baz"]));

    let args = Args {
      args: vec![String::from("bar"), String::from("foo")]
    };
    assert_eq!(Some(String::from("bar")), args.options(&["foo", "bar", "baz"]));

    let args = Args {
      args: vec![String::from("abc"), String::from("baz")]
    };
    assert_eq!(String::from("baz"), args.options(&["foo", "bar", "baz"]).unwrap());

    let args = Args {
      args: vec![String::from("abc"), String::from("xyz")]
    };
    assert_eq!(String::from("nothing"), args.options(&["foo", "bar", "baz"]).unwrap_or(String::from("nothing")));
  }

  #[test]
  fn collect_selected() {
    let args = Args {
      args: vec![String::from("sel"), String::from("baz")]
    };

    assert_eq!(args.select("sel", &["foo", "bar", "baz"]), ("sel", Some(String::from("baz"))));
    assert_eq!(args.select("none", &["foo", "bar", "baz"]), ("none", None));
  }

  #[test]
  fn collect_flag() {
    let args = Args {
      args: vec![String::from("foo"), String::from("bar")]
    };

    assert_eq!(args.flag("foo"), ("foo", true));
    assert_eq!(args.flag("bar"), ("bar", true));
    assert_eq!(args.flag("baz"), ("baz", false));
  }
}