mod generator;

fn main() {
    let mut ctx = generator::Context {
        htmlvarctr: 0,
        mathmlvarctr: 0,
        svgvarctr: 0,
        htmlvars: Vec::new(),
        htmlvargen: String::new(),
    };

    generator::generate_html_elements(&mut ctx, 5);

    println!("Generated HTML variables:");
    for var in &ctx.htmlvars {
        println!("{}: {}", var.name, var.type_);
    }

    println!("\nGenerated JavaScript code:");
    println!("{}", ctx.htmlvargen);
}
