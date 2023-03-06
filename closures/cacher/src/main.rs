// Cacher V and &V

struct Cacher<T, V>
    where T: Fn(V) -> V
{
    calculation: T,
    value: Option<V>,
}

impl<T, V> Cacher<T, V>
    where T: Fn(V) -> V, V: Copy
{
    fn new(calculation: T) -> Cacher<T, V> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value_ptr(&mut self, arg: V) -> &V {
        match self.value {
            Some(ref v) => v,
            None => {
                let x = (self.calculation)(arg);
                self.value = Some(x);
                self.value.as_ref().expect("no")
            }
        }
    }

    fn value(&mut self, arg: V) -> V {
        match self.value {
            Some(v) => v,
            None => {
                let x = (self.calculation)(arg);
                self.value = Some(x);
                self.value.expect("no")
            }
        }
    }
}

fn main() {
    let mut cacher = Cacher::new(|x| {
        println!("вычисление");
        x
    });

    print!("{}\n", cacher.value(5));
    print!("{}\n", cacher.value(8));

    //  print!("{} value_ptr\n", cacher.value_ptr(7));
    //  print!("{} value_ptr\n", cacher.value_ptr(8));
}
