#[allow(dead_code)]
fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy, // 泛型T所具有的特征
{
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/**********泛型在 结构体 中使用***********/

#[allow(dead_code)]
#[derive(Debug)]
struct Porint<S, T> {
    x: S,
    y: T,
}

/**********泛型在 枚举 中使用***********/
#[allow(dead_code)]
enum Option<T> {
    Some(T),
    None,
}

#[allow(dead_code)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

/**********泛型在 方法 中使用***********/
#[allow(dead_code)]
impl<S, T> Porint<S, T> {
    fn new<K, V>(x: K, y: V) -> Porint<K, V> {
        Porint { x, y }
    }

    fn creat_porint<V, W>(self, other: Porint<V, W>) -> Porint<S, W>{
        Porint { x: self.x, y: other.y }
    }

    fn get_x(&self) -> &S {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

#[cfg(test)]
mod test {
    use crate::generics::largest;
    use crate::generics::Porint;

    #[test]
    pub fn test() {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);
    }

    #[test]
    pub fn generics_of_struct() {
        let a = Porint { x: 20, y: '2' };
        println!("a = {:?}", a)
    }

    #[test]
    fn generics_of_function() {
        let a = Porint { x: "2", y: 0};
        let b = Porint { x: 3, y: '5'};
        let w = a.creat_porint(b);
        println!("{:?}", w);
    }
}
