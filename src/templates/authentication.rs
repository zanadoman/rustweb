use askama::Template;

#[derive(Template)]
#[template(path = "./authentication.html")]
pub struct AuthenticationTemplate<'a> {
    token: &'a str,
    location: &'a str,
    name: Option<&'a str>,
    form: AuthenticationFormTemplate<'a>,
    form_name: AuthenticationFormNameTemplate<'a>,
    form_password: AuthenticationFormPasswordTemplate<'a>,
}

impl<'a> AuthenticationTemplate<'a> {
    pub fn new(token: &'a str) -> Self {
        Self {
            token,
            location: "Authentication",
            name: None,
            form: AuthenticationFormTemplate::new(token, false),
            form_name: AuthenticationFormNameTemplate::new(token, true),
            form_password: AuthenticationFormPasswordTemplate::new(token, true),
        }
    }
}

#[derive(Template)]
#[template(path = "./authentication/form.html")]
pub struct AuthenticationFormTemplate<'a> {
    token: &'a str,
    form_name: AuthenticationFormNameTemplate<'a>,
    form_password: AuthenticationFormPasswordTemplate<'a>,
    error: bool,
}

impl<'a> AuthenticationFormTemplate<'a> {
    pub fn new(token: &'a str, error: bool) -> Self {
        Self {
            token,
            form_name: AuthenticationFormNameTemplate::new(token, false),
            form_password: AuthenticationFormPasswordTemplate::new(
                token, false,
            ),
            error,
        }
    }

    pub fn validate(mut self, error: Option<&'a str>) -> Self {
        self.form_name = self.form_name.validate("", error);
        self.form_password = self.form_password.validate("", error);
        self
    }
}

#[derive(Template)]
#[template(path = "./authentication/form_name.html")]
pub struct AuthenticationFormNameTemplate<'a> {
    token: &'a str,
    validation: bool,
    value: &'a str,
    error: Option<&'a str>,
}

impl<'a> AuthenticationFormNameTemplate<'a> {
    pub fn new(token: &'a str, validation: bool) -> Self {
        Self {
            token,
            validation,
            value: "",
            error: None,
        }
    }

    pub fn validate(mut self, value: &'a str, error: Option<&'a str>) -> Self {
        self.value = value;
        self.error = error;
        self
    }
}

#[derive(Template)]
#[template(path = "./authentication/form_password.html")]
pub struct AuthenticationFormPasswordTemplate<'a> {
    token: &'a str,
    validation: bool,
    value: &'a str,
    error: Option<&'a str>,
}

impl<'a> AuthenticationFormPasswordTemplate<'a> {
    pub fn new(token: &'a str, validation: bool) -> Self {
        Self {
            token,
            validation,
            value: "",
            error: None,
        }
    }

    pub fn validate(mut self, value: &'a str, error: Option<&'a str>) -> Self {
        self.value = value;
        self.error = error;
        self
    }
}
