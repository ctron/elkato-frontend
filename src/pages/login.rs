use elkato_api::Credentials;
use patternfly_yew::*;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let title = html_nested!(<Title>{"Login to Elkato"}</Title>);

    let credentials = use_session_storage::<Credentials>("credentials".into());

    let onclick = {
        let credentials = credentials.clone();
        Callback::from(move |_| {
            credentials.set(Credentials {
                username: "demo".to_string(),
                password: "demo".to_string(),
                club: "demo".to_string(),
            });
        })
    };

    let onclick_demo = {
        let credentials = credentials.clone();
        Callback::from(move |_| {
            credentials.set(Credentials {
                username: "demo".to_string(),
                password: "demo".to_string(),
                club: "demo".to_string(),
            });
        })
    };

    html!(<>
        <Background filter="contrast(65%) brightness(80%)"/>
        <patternfly_yew::Login>
            <LoginMain>
                <LoginMainHeader {title} description="Enter credentials and club for your Elkato booking system account"/>
                <LoginMainBody>
                    <Form>
                        <FormGroup label="Club">
                            <TextInput required=true name="club"/>
                        </FormGroup>
                        <FormGroup label="Username">
                            <TextInput required=true name="username"/>
                        </FormGroup>
                        <FormGroup label="Password">
                            <TextInput required=true name="password" r#type="password"/>
                        </FormGroup>
                        <ActionGroup>
                            <Button label="Log In" r#type={ButtonType::Submit} variant={Variant::Primary} {onclick}/>
                            <Button label="Use Demo" r#type={ButtonType::Submit} variant={Variant::Secondary} onclick={onclick_demo}/>
                        </ActionGroup>
                    </Form>
                </LoginMainBody>
                <LoginMainFooter />
            </LoginMain>
        </patternfly_yew::Login>
    </>)
}
