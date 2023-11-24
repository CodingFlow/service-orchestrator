use codegen::Function;
use http::Method;

pub fn generate_method(function: &mut Function, method: Method) {
    let method = method.as_str().to_lowercase();
    function.line(format!("warp::{}()", method));
}
