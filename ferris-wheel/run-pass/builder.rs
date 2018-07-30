// FIXME: Make me pass! Diff budget: 30 lines.
#[derive(Clone)]
struct Builder {
    string: Option<String>,
    number: Option<usize>,
}
pub trait Set<RHS:?Sized> {
    fn string(&mut self,&RHS)->Self;
}
impl Default for Builder {
    fn default()->Self{
        Builder{string:Some(String::from("")),number:None}
    }
}
impl Set<str> for Builder{
    fn string(&mut self,s: &str)->Self{
        self.string=Some(String::from(s));
        self.clone()
    }
}
impl Set<String> for Builder{
    fn string(&mut self,s:&String)->Self{
        self.string=Some(s);
        self.clone()
    }
}
impl Builder {
    fn number(&mut self,i: usize)->Self{
        self.number=Some(i);
        self.clone()
    }
    fn to_string(self)->String{
        match (self.string,self.number) {
            (Some(ref s),None) => format!("{}",s),
            (Some(ref s),Some(ref i))=>if s=="" {
                                            format!("{}",i)
                                            }
                                        else{
                                            format!("{} {}",s,i)
                                        },

            _=>String::from("")
        }
    }
}
// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}
