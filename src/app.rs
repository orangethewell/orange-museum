use yew::prelude::*;
use yew_router::prelude::*;
use gloo_storage::{Storage, LocalStorage};


#[derive(Clone, Routable, PartialEq)]
enum Room {
    #[at("/")]
    FrontDoor,
    #[at("/hall")]
    Hall,
    #[at("/basement")]
    Basement,
    #[at("/basement/message")]
    BasementMessage
}
#[function_component(FrontDoor)]
pub fn front_door() -> Html {
    let orange_num = use_state(|| LocalStorage::get("oranges").unwrap_or_else(|_| 0));
    let navigator = use_navigator().unwrap();

    let orange_onclick = {
        let orange_num = orange_num.clone();
        Callback::from(move |_|{
            let _ = LocalStorage::set("oranges", *orange_num + 1).unwrap();
            navigator.push(&Room::Hall);
        })
    };
    html! {
        <>
        <NavigationBar orangeCount={*orange_num}/>
        <main id="front">
            <div id="frontdoor">
                <h1>{ "You found a "}<i class="underline">{"Laranjinha!"}</i></h1>
                <a onclick={orange_onclick} class="orange"></a>
            </div>
        </main>
        </>
    }
}

// Welcome to my Museum!
#[function_component(Hall)]
pub fn hall() -> Html {
    let orange_num = use_state(|| LocalStorage::get("oranges").unwrap_or_else(|_| 0));
    let img_src = use_state(|| String::from("/orange-museum/assets/spr_player_down.png"));
    
    let img_onover = {
        let img_src = img_src.clone();
        Callback::from(move |event: MouseEvent| {
                event.prevent_default();
                img_src.set("/orange-museum/public/jadepixel_walking.gif".to_owned());
            }
        )
    };

    let img_onout = {
        let img_src = img_src.clone();
        Callback::from(move |event: MouseEvent| {
                event.prevent_default();
                img_src.set("/orange-museum/assets/spr_player_down.png".to_owned());
            }
        )
    };

    html!{
        <>
        <NavigationBar orangeCount={*orange_num}/>
        <main id="homepage">
            <div id="content-area" class="max-expand">
                <div class="flex-container-1-per-3 max-expand">
                    <div id="post-area">
                        <div id="welcome-post" class="border-framed post">
                            <h1>{"Hello, world!"}</h1>
                            <p class="creation-date">{"made in July 12th, 2023"}</p>
                            <p>{"My name is "}<b>{"Orangethewell"}</b>{" and I'm a artist and programmer.
                            This website is like a blog about what I do and my creations during the time.
                            For now it's half empty, but check it regularly to see more news. :)
                            "}</p>
                        </div>
                    </div>
                    <div id="contact-area" class="border-framed">
                        <h1>{"Contact me"}</h1>
                        <ul id="contact-list" class="unstyled-list">
                        <li><a href="https://www.instagram.com/orangethewell/"> <img src="/orange-museum/public/instagram.svg"/> {"Instagram"} </a></li>
                        <li><a href="mailto:orangethewell@gmail.com"> <img src="/orange-museum/public/email.svg"/> {"E-mail"} </a></li>
                        </ul>
                    </div>
                </div>
            </div>
            <Link<Room> to={Room::Basement} classes="secret-button"><img onmouseover={img_onover} onmouseout={img_onout} src={(*img_src).clone()}/></Link<Room>>
        </main>
        </>
    }
}

#[allow(non_snake_case)]
#[derive(Properties, PartialEq)]
pub struct BasementProps {
    #[prop_or_default]
    pub showMessage: bool,
}

#[function_component(Basement)]
pub fn basement(props: &BasementProps) -> Html {
    html!{
        if props.showMessage {
            <>
                <main id="basement">
                    <div class="message-box">
                        <p class="typed-effect"></p>
                        <Link<Room> to={Room::Hall} classes="go-back-float">{"Go Back <-"}</Link<Room>>
                    </div>
                </main>
            </>
        } else {
            <>
                <canvas id="glcanvas" tabindex="1"></canvas>
                <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
                <script src="https://not-fl3.github.io/miniquad-samples/gl.js"></script>
                <script src="/orange-museum/public/loader.js"></script>
            </>
        }
    }
}

fn doors_switch(rooms: Room) -> Html {
    match rooms {
        Room::FrontDoor => html! { <FrontDoor/> },
        Room::Hall => html!{ <Hall/> },
        Room::Basement => html!{ <Basement/> },
        Room::BasementMessage => html! { <Basement showMessage=true/>}
    }
}

#[allow(non_snake_case)]
#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub orangeCount: i32,
}

#[function_component(NavigationBar)]
pub fn nav_bar(props: &NavProps) -> Html {
    html!{
        <div id="navbar">
            <ul>
                <li><span style="float: right">{props.orangeCount}{" 🍊"}</span></li>
            </ul>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {    
    html! {
        <BrowserRouter>
            <Switch<Room> render={doors_switch}/>
        </BrowserRouter>
    }
}
