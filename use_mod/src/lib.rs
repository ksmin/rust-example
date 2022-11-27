pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}


#[cfg(test)]
mod tests {
    use super::*;
    use a::series::of::nested_modules;
    use TrafficLight::*;

    #[test]
    fn it_works() {
        nested_modules();
        let light1 = Red;
        let light2 = Yellow;
        let light2 = Green;
    }
}
