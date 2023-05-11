use yew::prelude::*;

struct Model {
    servers: Vec<Server>
}

struct Server {
    id: i32,
    load: i32,
}

enum Msg {
    AddServer,
    RemoveServer(i32),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            servers: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddServer => {
                let id = self.servers.len() as i32;
                self.servers.push(Server { id, load: 0 });
            }
            Msg::RemoveServer(id) => {
                self.servers.retain(|server| server.id != id);
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <button onclick=|_| Msg::AddServer>{ "Add Server" }</button>
                { for self.servers.iter().map(|server| self.view_server(server)) }
            </>
        }
    }

}

impl Model {
    fn view_server(&self, server: &Server) -> Html {
        let id = server.id;
        html! {
            <div>
                <span>{ format!("Load #{}", server.load) }</span>
                <span>{ format!("Server #{}", server.id) }</span>
                <button onclick=self.link.callback(move |_| Msg::RemoveServer(id))>{ "Remove Server" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}