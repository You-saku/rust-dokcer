mod module; //ファイル名
mod modules;
fn main() {
    module::sample_module::hello_mod(); // ファイル名::module名::関数名

    let result = modules::dir::double(1);
    println!("{}", result);
}
