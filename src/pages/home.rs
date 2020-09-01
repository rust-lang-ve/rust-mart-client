use yew::prelude::*;

struct Product {
    id: i32,
    name: String,
    description: String,
    image: String,
    price: f64,
}

struct State {
    products: Vec<Product>,
}

pub struct Home {
    state: State,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let products: Vec<Product> = vec![
            Product {
                id: 1,
                name: "Apple".to_string(),
                description: "An apple a day keeps the doctor way".to_string(),
                image: "/products/apple.png".to_string(),
                price: 3.65,
            },
            Product {
                id: 1,
                name: "Banana".to_string(),
                description: "An old banana leaf was once young and green".to_string(),
                image: "/products/banana.png".to_string(),
                price: 7.99,
            },
        ];

        Self {
            state: State { products },
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let products: Vec<Html> = self
            .state
            .products
            .iter()
            .map(|product: &Product| {
                html! {
                  <div>
                    <img src={&product.image}/>
                    <div>{"id: "}{&product.id}</div>
                    <div>{&product.name}</div>
                    <div>{&product.description}</div>
                    <div>{"$"}{&product.price}</div>
                  </div>
                }
            })
            .collect();

        html! {
          <span>{products}</span>
        }
    }
}
