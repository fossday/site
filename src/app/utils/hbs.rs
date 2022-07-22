use dotenv_codegen::dotenv;

use rocket_dyn_templates::handlebars::{self, Handlebars, JsonRender};

fn make_url_helper(
    h: &handlebars::Helper<'_, '_>,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_, '_>,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    if let Some(param) = h.param(0) {
        let app_url: &str = dotenv!("APP_URL");
        let url: String = format!("{}/{}", app_url, &param.value().render());
        out.write(&url)?;
    }
    Ok(())
}

fn app_url_helper(
    _h: &handlebars::Helper<'_, '_>,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_, '_>,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    out.write(&dotenv!("APP_URL"))?;
    Ok(())
}

fn app_name_helper(
    _h: &handlebars::Helper<'_, '_>,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_, '_>,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    out.write(&dotenv!("APP_NAME"))?;
    Ok(())
}

pub fn customize(hbs: &mut Handlebars) {
    hbs.register_helper("make_url", Box::new(make_url_helper));
    hbs.register_helper("app_url", Box::new(app_url_helper));
    hbs.register_helper("app_name", Box::new(app_name_helper));
}
