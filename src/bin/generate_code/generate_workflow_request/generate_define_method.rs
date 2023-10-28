use codegen::Scope;
use http::Method;

pub fn generate_define_method(scope: &mut Scope, method: Method) {
    let function = scope
        .new_fn("define_method")
        .ret("impl Filter<Extract = (), Error = Rejection> + Copy");

    let method = method.as_str().to_lowercase();
    function.line(format!("warp::{}()", method));
}
