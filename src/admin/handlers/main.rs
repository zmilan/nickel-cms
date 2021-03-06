use super::*;

pub fn get_main<'mw>(_: &mut Request<Config>, res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let mut header = HeaderData::new();
    header.title = "Main";
    header.action = "main";
    render(res, |o| templates::admin::main::index(o, &header))
}
