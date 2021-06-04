use tera::Tera;
use warp::{self, Filter};

pub fn with_tera() -> warp::filters::BoxedFilter<(Tera,)> {
    warp::any().map(move || tera_templates()).boxed()
}

pub fn tera_templates() -> Tera {
    let login_tpl = r#"
<!DOCTYPE html>
<head>
    <meta charset="UTF-8">
    <title>Login</title>
</head>
<body>
<h1>Login page</h1>
<form action="/login" method="post">
{% if login_challenge %}
    <input type="hidden" name="login_challenge" value="{{login_challenge}}"/>
{% else %}
{% endif %}

    <label for="username">Username</label>:
    <input type="text" id="username" name="username" autofocus="autofocus"/> <br/>

    <label for="password">Password</label>:
    <input type="password" id="password" name="password"/> <br/>

    <input type="submit" value="Log in"/>
</form>
</body>
</html>
"#;

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![("login.html", login_tpl)])
        .unwrap();
    tera
}
