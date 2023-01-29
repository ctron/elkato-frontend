use crate::app::Session;
use elkato_api::Credentials;
use patternfly_yew::*;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let title = html_nested!(<Title>{"Login to Elkato"}</Title>);

    let club = use_state_eq(|| String::new());
    let username = use_state_eq(|| String::new());
    let password = use_state_eq(|| String::new());

    let session = use_context::<Session>().unwrap();

    let onclick = {
        let session = session.clone();
        let username = username.clone();
        let password = password.clone();
        let club = club.clone();
        Callback::from(move |_| {
            session.login(Credentials {
                username: (*username).clone(),
                password: (*password).clone(),
                club: (*club).clone(),
            });
        })
    };

    let onclick_demo = {
        let session = session.clone();
        Callback::from(move |_| {
            session.login(Credentials {
                username: "demo".to_string(),
                password: "demo".to_string(),
                club: "demo".to_string(),
            });
        })
    };

    let set_club = {
        let club = club.clone();
        Callback::from(move |s| club.set(s))
    };
    let set_username = {
        let username = username.clone();
        Callback::from(move |s| username.set(s))
    };
    let set_password = {
        let password = password.clone();
        Callback::from(move |s| password.set(s))
    };

    html!(<>
        <Background filter="contrast(65%) brightness(80%)"/>
        <patternfly_yew::Login>
            <LoginMain>
                <LoginMainHeader {title} description="Enter credentials and club for your Elkato booking system account"/>
                <LoginMainBody>
                    <Form>
                        <FormGroup label="Club">
                            <TextInput required=true name="club" onchange={set_club}/>
                        </FormGroup>
                        <FormGroup label="Username">
                            <TextInput required=true name="username" onchange={set_username}/>
                        </FormGroup>
                        <FormGroup label="Password">
                            <TextInput required=true name="password" r#type="password" onchange={set_password}/>
                        </FormGroup>
                        <ActionGroup>
                            <Button label="Log In" r#type={ButtonType::Button} variant={Variant::Primary} {onclick}/>
                            <Button label="Use Demo" r#type={ButtonType::Button} variant={Variant::Secondary} onclick={onclick_demo}/>
                        </ActionGroup>
                    </Form>
                </LoginMainBody>
                <LoginMainFooter />
            </LoginMain>
        </patternfly_yew::Login>
    </>)
}
